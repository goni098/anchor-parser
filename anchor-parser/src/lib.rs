#![cfg_attr(docsrs, feature(doc_cfg))]

//! # anchor-parser
//!
//! [![Crates.io](https://img.shields.io/crates/v/anchor-parser.svg)](https://crates.io/crates/anchor-parser)
//! [![docs.rs](https://docs.rs/anchor-parser/badge.svg)](https://docs.rs/anchor-parser)
//! [![MIT License](https://img.shields.io/crates/l/anchor-parser.svg)](https://github.com/goni098/anchor-parser/blob/main/LICENSE)
//!
//! Generate Rust types and helpers from [Anchor](https://www.anchor-lang.com/) IDL
//! JSON files using [`solana-sdk`] types directly — no `anchor-lang` dependency required.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! anchor-parser = "0.1.3"
//! ```
//!
//! Place an Anchor IDL JSON file at `<crate-root>/idls/<name>.json`, then:
//!
//! ```ignore
//! anchor_parser::declare_program!(my_program);
//! ```
//!
//! # Generated modules
//!
//! [`declare_program!`] generates a module with the following sub-modules:
//!
//! | Module | Contents |
//! |--------|----------|
//! | `accounts` | Account structs with [`AccountDeserialize`] and `from_account_data` |
//! | `events` | Event structs with `from_logs` and `from_cpi_logs` |
//! | `instructions` | Builder functions returning [`solana_sdk::instruction::Instruction`] |
//! | `types` | Shared structs, enums, and type aliases from the IDL |
//! | `constants` | Program constants with doc comments |
//! | `utils` | `Event` / `Account` wrapper enums for generic parsing |
//!
//! # Examples
//!
//! ## Deserializing accounts
//!
//! ```ignore
//! use my_program::accounts::MyAccount;
//!
//! // Deserialize from raw account data (discriminator + payload)
//! let account = MyAccount::from_account_data(&raw_bytes)?;
//!
//! // Check discriminator
//! assert_eq!(MyAccount::DISCRIMINATOR, [/* 8 bytes */]);
//! ```
//!
//! ## Fetching accounts via RPC
//!
//! *Requires the [`client`] feature.*
//!
//! Every account type gets `fetch` and `fetch_multiple` methods via the
//! [`AccountDeserialize`] trait:
//!
//! ```ignore
//! use anchor_parser::AccountDeserialize;
//! use my_program::accounts::MyAccount;
//!
//! // Fetch a single account
//! let account = MyAccount::fetch(&rpc, &address).await?;
//!
//! // Fetch multiple accounts at once
//! let accounts = MyAccount::fetch_multiple(&rpc, &[addr1, addr2]).await?;
//! ```
//!
//! Or use the standalone functions from the [`client`] module:
//!
//! ```ignore
//! use anchor_parser::client;
//!
//! let account = client::fetch_account::<MyAccount>(&rpc, &address).await?;
//! ```
//!
//! ## Parsing events
//!
//! ```ignore
//! use my_program::events::SwapEvent;
//! use my_program::utils::Event;
//!
//! // Parse events from emit! log lines
//! let events = SwapEvent::from_logs(&log_messages);
//! let all_events = Event::from_logs(&log_messages);
//!
//! // Parse events from emit_cpi! inner instruction data (bs58-encoded)
//! let events = SwapEvent::from_cpi_logs(&inner_ix_data_strings);
//! let all_events = Event::from_cpi_logs(&inner_ix_data_strings);
//! ```
//!
//! ## Building instructions
//!
//! ```ignore
//! use my_program::instructions;
//!
//! let ix = instructions::swap(
//!     &my_program::ID,
//!     &instructions::SwapAccounts { /* ... */ },
//!     amount,
//!     min_out,
//! );
//! ```
//!
//! # Feature flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `client` | Enables [`AccountDeserialize::fetch`] / [`AccountDeserialize::fetch_multiple`] and the [`client`] module via `solana-client` |

/// Generates a module from an Anchor IDL JSON file.
///
/// Looks for `idls/{name}.json` by walking up from `CARGO_MANIFEST_DIR`.
///
/// # Generated items
///
/// - `ID` — program [`Pubkey`](solana_sdk::pubkey::Pubkey)
/// - `accounts` — account structs implementing [`AccountDeserialize`]
/// - `events` — event structs with `from_logs` / `from_cpi_logs`
/// - `instructions` — builder functions returning [`Instruction`](solana_sdk::instruction::Instruction)
/// - `types` — shared structs, enums, and type aliases
/// - `constants` — program constants
/// - `utils` — `Event` / `Account` wrapper enums
///
/// # Example
///
/// ```ignore
/// anchor_parser::declare_program!(my_program);
///
/// // Now use my_program::accounts, my_program::events, etc.
/// ```
pub use anchor_parser_macros::declare_program;

/// Async RPC helpers for fetching and deserializing on-chain accounts.
///
/// Enable with the `client` feature:
///
/// ```toml
/// [dependencies]
/// anchor-parser = { version = "0.1.3", features = ["client"] }
/// ```
///
/// # Example
///
/// ```ignore
/// use anchor_parser::client;
/// use my_program::accounts::MyAccount;
///
/// let account = client::fetch_account::<MyAccount>(&rpc, &address).await?;
/// let accounts = client::fetch_accounts::<MyAccount>(&rpc, &[a1, a2]).await?;
/// ```
#[cfg(feature = "client")]
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
pub mod client;

/// Trait implemented by all generated account types.
///
/// Provides a discriminator constant and a method to deserialize from raw
/// on-chain account data (discriminator prefix + serialized payload).
///
/// You generally won't need to use this trait directly — generated account
/// types also expose a convenient `from_account_data` method.
///
/// # Example
///
/// ```ignore
/// use anchor_parser::AccountDeserialize;
///
/// fn parse<T: AccountDeserialize>(data: &[u8]) -> std::io::Result<T> {
///     T::deserialize(data)
/// }
/// ```
pub trait AccountDeserialize: Sized {
    /// The discriminator bytes that prefix this account's on-chain data.
    const DISCRIMINATOR: &'static [u8];

    /// Deserialize from raw account data (including the discriminator prefix).
    ///
    /// Returns an error if the data is too short, the discriminator doesn't
    /// match, or deserialization fails.
    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error>;

    /// Fetch and deserialize a single account from an async RPC client.
    ///
    /// Requires the `client` feature.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use my_program::accounts::MyAccount;
    ///
    /// let account = MyAccount::fetch(&rpc, &address).await?;
    /// ```
    #[cfg(feature = "client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
    fn fetch(
        client: &solana_client::nonblocking::rpc_client::RpcClient,
        address: &solana_sdk::pubkey::Pubkey,
    ) -> impl std::future::Future<Output = Result<Self, solana_client::client_error::ClientError>>
    {
        crate::client::fetch_account::<Self>(client, address)
    }

    /// Fetch and deserialize multiple accounts in a single RPC call.
    ///
    /// Returns `None` for accounts that don't exist or fail deserialization.
    ///
    /// Requires the `client` feature.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use my_program::accounts::MyAccount;
    ///
    /// let accounts = MyAccount::fetch_multiple(&rpc, &[a1, a2]).await?;
    /// ```
    #[cfg(feature = "client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
    fn fetch_multiple(
        client: &solana_client::nonblocking::rpc_client::RpcClient,
        addresses: &[solana_sdk::pubkey::Pubkey],
    ) -> impl std::future::Future<
        Output = Result<Vec<Option<Self>>, solana_client::client_error::ClientError>,
    > {
        crate::client::fetch_accounts::<Self>(client, addresses)
    }
}

#[doc(hidden)]
pub mod __private {
    pub use solana_sdk::instruction::{AccountMeta, Instruction};
    pub use solana_sdk::pubkey::Pubkey;

    pub use borsh::{BorshDeserialize, BorshSerialize};
    pub use bytemuck::{Pod, Zeroable};

    #[inline]
    pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(input).ok()
    }

    #[inline]
    pub fn bs58_decode(input: &str) -> Option<Vec<u8>> {
        bs58::decode(input).into_vec().ok()
    }

    #[inline]
    pub fn bytemuck_read<T: bytemuck::Pod>(data: &[u8]) -> T {
        bytemuck::pod_read_unaligned(data)
    }
}
