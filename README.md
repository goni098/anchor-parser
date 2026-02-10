# anchor-parser

[![Crates.io](https://img.shields.io/crates/v/anchor-parser.svg)](https://crates.io/crates/anchor-parser)
[![docs.rs](https://docs.rs/anchor-parser/badge.svg)](https://docs.rs/anchor-parser)
[![MIT License](https://img.shields.io/crates/l/anchor-parser.svg)](https://github.com/goni098/anchor-parser/blob/main/LICENSE)

Generate Rust types and helpers from [Anchor](https://www.anchor-lang.com/) IDL
JSON files using **solana-sdk** types directly — no `anchor-lang` dependency
required.

## Features

- **Accounts** — Structs with discriminator constants and deserialization
  (Borsh & bytemuck/zero-copy).
- **Events** — Structs with `from_logs` (`emit!`) and `from_cpi_logs`
  (`emit_cpi!`) parsers.
- **Instructions** — Builder functions that return
  `solana_sdk::instruction::Instruction`.
- **Types** — Shared structs, enums, and type aliases from the IDL.
- **Constants** — Program constants with doc comments.
- **Utils** — `Event` and `Account` wrapper enums for generic parsing across
  all program types.
- **Client** *(optional)* — Async `fetch_account` / `fetch_accounts` via
  `solana-client`.

## Installation

```toml
[dependencies]
anchor-parser = "0.1.3"
```

To enable the async RPC client ([`solana-client`](https://docs.rs/solana-client) dependency):

```toml
[dependencies]
anchor-parser = { version = "0.1.3", features = ["client"] }
```

## Quick start

1. Place an Anchor IDL JSON file in an `idls/` directory at your crate root:

```
my-crate/
├── Cargo.toml
├── idls/
│   └── my_program.json
└── src/
    └── main.rs
```

2. Declare the program:

```rust
use anchor_parser::declare_program;

declare_program!(my_program);
```

This generates a `my_program` module with the following sub-modules:

| Module | Contents |
|--------|----------|
| `my_program::accounts` | Account structs with `DISCRIMINATOR` and `from_account_data` |
| `my_program::events` | Event structs with `from_logs` and `from_cpi_logs` |
| `my_program::instructions` | Builder functions → `Instruction` |
| `my_program::types` | Shared structs, enums, type aliases |
| `my_program::constants` | Program constants |
| `my_program::utils` | `Event` / `Account` wrapper enums |

The program ID is available as `my_program::ID`.

## Usage

### Accounts

```rust
use my_program::accounts::MyAccount;

// Deserialize from raw account data (discriminator + payload)
let account = MyAccount::from_account_data(&raw_bytes)?;

// Check discriminator
assert_eq!(MyAccount::DISCRIMINATOR, [/* 8 bytes */]);
```

### Fetch accounts via RPC

> Requires the `client` feature.

Every account type gets `fetch` and `fetch_multiple` methods via the
`AccountDeserialize` trait:

```rust
use anchor_parser::AccountDeserialize;
use my_program::accounts::MyAccount;

// Fetch a single account
let account = MyAccount::fetch(&rpc, &address).await?;

// Fetch multiple accounts at once
let accounts = MyAccount::fetch_multiple(&rpc, &[addr1, addr2]).await?;
```

Or use the standalone functions from the `client` module:

```rust
use anchor_parser::client;

let account = client::fetch_account::<MyAccount>(&rpc, &address).await?;
```

### Events

```rust
use my_program::events::SwapEvent;

// Parse events from emit! logs (base64-encoded "Program data:" log lines)
let events = SwapEvent::from_logs(&log_messages);

// Parse events from emit_cpi! inner instruction data (bs58-encoded)
let cpi_events = SwapEvent::from_cpi_logs(&inner_instruction_data);
```

Both methods accept any `IntoIterator<Item = I> where I: AsRef<str>` —
vec, slice, array, iterator, `&[String]`, `&[&str]`, etc.

### Event & Account enums

The `utils` module provides wrapper enums that try all known discriminators:

```rust
use my_program::utils::{Event, Account};

// Parse any program event from logs
let events: Vec<Event> = Event::from_logs(&log_messages);
let events: Vec<Event> = Event::from_cpi_logs(&log_messages);

match &events[0] {
    Event::SwapEvent(e) => println!("swap amount: {}", e.amount_0),
    Event::PoolCreatedEvent(e) => println!("new pool: {}", e.pool_state),
    _ => {}
}

// Parse any program account from raw data
let account: Account = Account::parse(&raw_bytes)?;
```

### Instructions

```rust
use my_program::instructions;

let ix = instructions::swap(
    &my_program::ID,
    &instructions::SwapAccounts {
        payer: wallet.pubkey(),
        pool_state: pool_address,
        // ...
    },
    amount,
    min_out,
);
// ix: solana_sdk::instruction::Instruction
```

### Constants

```rust
use my_program::constants;

let fee = constants::FEE_DENOMINATOR; // u64
let prefix = constants::POOL_PREFIX;  // &[u8]
```

## `from_logs` vs `from_cpi_logs`

| Method | Source | Input | Decoding |
|--------|--------|-------|----------|
| `from_logs` | `emit!` | Transaction log messages | Scans for `"Program data: <base64>"` lines, base64-decodes, skips 8-byte discriminator, borsh-deserializes |
| `from_cpi_logs` | `emit_cpi!` | Inner instruction data strings | bs58-decodes each string, skips 16 bytes (8-byte CPI event tag + 8-byte discriminator), borsh-deserializes |

Both methods share the same generic signature:

```rust
pub fn from_logs<T, I>(logs: T) -> Vec<Self>
where
    T: IntoIterator<Item = I>,
    I: AsRef<str>,
```

### How `emit_cpi!` events work

Anchor's `emit_cpi!` emits events as **self-CPI inner instructions**.
The instruction data is laid out as:

```text
[8 bytes: Anchor CPI event tag] [8 bytes: event discriminator] [borsh payload]
```

To use `from_cpi_logs`, pass the **bs58-encoded inner instruction data** strings
(not log messages) from instructions targeting your program.

## Supported serialization formats

| Format | Accounts | Events |
|--------|----------|--------|
| Borsh | ✅ | ✅ |
| Bytemuck (zero-copy) | ✅ | — |
| Bytemuck-unsafe (packed) | ✅ | — |

## IDL compatibility

Supports the Anchor IDL JSON format. The IDL file name
(without `.json`) becomes the Rust module name.

## Tests

```sh
cargo test
```

The test suite covers four real-world programs (306 tests total):

| Program | IDL | Tests |
|---------|-----|-------|
| Pumpfun | `idls/pumpfun.json` | 47 |
| Meteora DAMM v2 | `idls/meteora_damm_v2.json` | 83 |
| Meteora DLMM | `idls/meteora_dlmm.json` | 112 |
| Raydium CLMM | `idls/raydium_clmm.json` | 64 |

## License

MIT
