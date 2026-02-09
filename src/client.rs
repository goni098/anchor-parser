//! Async RPC client helpers for fetching and deserializing accounts.
//!
//! Requires the `client` feature (adds a `solana-client` dependency).
//!
//! ```ignore
//! use anchor_parser::client;
//! use my_program::accounts::MyAccount;
//!
//! let account = client::fetch_account::<MyAccount>(&rpc_client, &address).await?;
//! let accounts = client::fetch_accounts::<MyAccount>(&rpc_client, &[addr1, addr2]).await?;
//! ```

use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::AccountDeserialize;

/// Fetch and deserialize a single account from an async RPC client.
pub async fn fetch_account<T: AccountDeserialize>(
    client: &RpcClient,
    address: &Pubkey,
) -> Result<T, ClientError> {
    let account = client.get_account(address).await?;
    T::deserialize(&account.data).map_err(|e| e.into())
}

/// Fetch and deserialize multiple accounts of the same type.
///
/// Returns `None` for accounts that don't exist or fail deserialization.
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
