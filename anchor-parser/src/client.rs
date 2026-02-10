//! Async RPC client helpers for fetching and deserializing on-chain accounts.
//!
//! This module is available when the `client` feature is enabled, which adds
//! a dependency on [`solana-client`](https://docs.rs/solana-client).
//!
//! ```toml
//! [dependencies]
//! anchor-parser = { version = "0.1.3", features = ["client"] }
//! ```
//!
//! # Examples
//!
//! ```ignore
//! use anchor_parser::client;
//! use my_program::accounts::MyAccount;
//!
//! let rpc = solana_client::nonblocking::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
//!
//! // Fetch a single account
//! let account = client::fetch_account::<MyAccount>(&rpc, &address).await?;
//!
//! // Fetch multiple accounts at once
//! let accounts = client::fetch_accounts::<MyAccount>(&rpc, &[addr1, addr2]).await?;
//! ```

use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::AccountDeserialize;

/// Fetch a single account and deserialize it into type `T`.
///
/// Uses [`RpcClient::get_account`] under the hood, then delegates to
/// [`AccountDeserialize::deserialize`] which checks the discriminator and
/// borsh/bytemuck-deserializes the payload.
///
/// # Errors
///
/// Returns an error if the RPC call fails or if deserialization fails
/// (wrong discriminator, data too short, etc.).
///
/// # Example
///
/// ```ignore
/// use anchor_parser::client;
/// use my_program::accounts::MyAccount;
///
/// let account = client::fetch_account::<MyAccount>(&rpc, &address).await?;
/// ```
pub async fn fetch_account<T: AccountDeserialize>(
    client: &RpcClient,
    address: &Pubkey,
) -> Result<T, ClientError> {
    let account = client.get_account(address).await?;
    T::deserialize(&account.data).map_err(|e| e.into())
}

/// Fetch multiple accounts of the same type in a single RPC call.
///
/// Uses [`RpcClient::get_multiple_accounts`] under the hood. Returns `None`
/// for addresses that don't exist on-chain or whose data fails deserialization.
///
/// # Errors
///
/// Returns an error if the RPC call itself fails. Individual deserialization
/// failures are silently mapped to `None`.
///
/// # Example
///
/// ```ignore
/// use anchor_parser::client;
/// use my_program::accounts::MyAccount;
///
/// let results = client::fetch_accounts::<MyAccount>(&rpc, &[addr1, addr2]).await?;
/// for (i, maybe_account) in results.iter().enumerate() {
///     match maybe_account {
///         Some(account) => println!("Account {i}: {account:?}"),
///         None => println!("Account {i}: not found or invalid"),
///     }
/// }
/// ```
pub async fn fetch_accounts<T: AccountDeserialize>(
    client: &RpcClient,
    addresses: &[Pubkey],
) -> Result<Vec<Option<T>>, ClientError> {
    let accounts = client.get_multiple_accounts(addresses).await?;
    Ok(accounts
        .into_iter()
        .map(|maybe_acc| maybe_acc.and_then(|acc| T::deserialize(&acc.data).ok()))
        .collect())
}
