extern crate proc_macro;

mod codegen;
mod idl;

use proc_macro::TokenStream;
use quote::quote;

/// Generate a complete program module from an Anchor IDL JSON file.
///
/// Searches for `idls/{name}.json` by walking up from `CARGO_MANIFEST_DIR`.
///
/// # Generated items
///
/// - **`ID`** — the program's [`Pubkey`](solana_sdk::pubkey::Pubkey)
/// - **`types`** — shared structs, enums, and type aliases
/// - **`accounts`** — account structs with discriminator and deserialization
/// - **`events`** — event structs with `from_logs` / `from_cpi_logs` parsers
/// - **`instructions`** — builder functions → [`Instruction`](solana_sdk::instruction::Instruction)
/// - **`constants`** — program constants
/// - **`utils`** — `Event` / `Account` wrapper enums
///
/// # Example
///
/// ```ignore
/// anchor_parser::declare_program!(my_program);
///
/// // Use generated types:
/// use my_program::accounts::MyAccount;
/// use my_program::events::MyEvent;
/// use my_program::instructions;
/// ```
#[proc_macro]
pub fn declare_program(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::Ident);
    match codegen::generate(&name) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            let msg = err.to_string();
            quote! { compile_error!(#msg); }.into()
        }
    }
}
