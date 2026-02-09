//! # anchor-parser
//!
//! Generates Rust types and helpers from Anchor IDL JSON files using `solana-sdk`
//! types directly (no `anchor-lang` dependency).
//!
//! ## Generated modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | `accounts` | Account structs with discriminators and deserialization |
//! | `events` | Event structs with `from_logs` parsing |
//! | `instructions` | Builder functions → `solana_sdk::instruction::Instruction` |
//! | `types` | Shared structs, enums, and type aliases |
//! | `constants` | Program constants |
//! | `utils` | `Event` / `Account` wrapper enums for generic parsing |
//!
//! ## Usage
//!
//! ```ignore
//! anchor_parser::declare_program!(my_program);
//!
//! use my_program::accounts::MyAccount;
//! use my_program::events::MyEvent;
//! use my_program::instructions;
//! use my_program::constants;
//! use my_program::utils::Event;
//!
//! // Build an instruction
//! let ix = instructions::update(
//!     &my_program::ID,
//!     &instructions::UpdateAccounts { authority, my_account },
//!     42,
//! );
//!
//! // Fetch an account (requires "client" feature)
//! use anchor_parser::client;
//! let account = client::fetch_account::<MyAccount>(&rpc_client, &address).await?;
//! let accounts = client::fetch_accounts::<MyAccount>(&rpc_client, &[addr1, addr2]).await?;
//!
//! // Deserialize from raw account data
//! let account = MyAccount::from_account_data(&raw_bytes)?;
//!
//! // Parse events from emit! logs
//! let events = MyEvent::from_logs(&log_messages);
//! let all_events = Event::from_logs(&log_messages);
//!
//! // Parse events from emit_cpi! inner instruction data (bs58-encoded)
//! let events = MyEvent::from_cpi_logs(&inner_ix_data_strings);
//! let all_events = Event::from_cpi_logs(&inner_ix_data_strings);
//! ```

pub use anchor_parser_macros::declare_program;

// ── Async RPC client (requires "client" feature) ──────────────────────

#[cfg(feature = "client")]
pub mod client;

// ── Public trait for account deserialization ───────────────────────────

/// Trait implemented by generated account types.
///
/// Provides a discriminator and a method to deserialize from raw account data
/// (discriminator prefix + serialized payload).
pub trait AccountDeserialize: Sized {
    /// The discriminator bytes that prefix this account's on-chain data.
    const DISCRIMINATOR: &'static [u8];

    /// Deserialize from raw account data (including discriminator prefix).
    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error>;
}

// ── Private re-exports for generated code ─────────────────────────────

/// Re-exports used by the generated `declare_program!` code.
///
/// **Do not use directly.** This module is an implementation detail.
#[doc(hidden)]
pub mod __private {
    pub use solana_sdk::instruction::{AccountMeta, Instruction};
    pub use solana_sdk::pubkey::Pubkey;

    pub use borsh::{BorshDeserialize, BorshSerialize};

    // Bytemuck re-exports
    pub use bytemuck::{Pod, Zeroable};

    /// Decode a base64 string. Returns `None` on failure.
    pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(input).ok()
    }

    /// Decode a bs58 string. Returns `None` on failure.
    pub fn bs58_decode(input: &str) -> Option<Vec<u8>> {
        bs58::decode(input).into_vec().ok()
    }

    /// Read a `Pod` type from a byte slice (handles unaligned data).
    pub fn bytemuck_read<T: bytemuck::Pod>(data: &[u8]) -> T {
        bytemuck::pod_read_unaligned(data)
    }
}
