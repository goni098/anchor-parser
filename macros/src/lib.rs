extern crate proc_macro;

mod gen;
mod idl;

use proc_macro::TokenStream;
use quote::quote;

/// Declare a program module from an Anchor IDL JSON file.
///
/// Looks for `idls/{name}.json` by walking up from `CARGO_MANIFEST_DIR`.
///
/// # Example
/// ```ignore
/// anchor_parser::declare_program!(my_program);
/// ```
///
/// This generates a module `my_program` containing:
/// - `ID` constant (`Pubkey`)
/// - `types` module (struct/enum/alias definitions)
/// - `accounts` module (account types with discriminators + fetch)
/// - `events` module (event types with `from_log`)
/// - `instructions` module (instruction builders → `Instruction`)
/// - `constants` module
#[proc_macro]
pub fn declare_program(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::Ident);
    match gen::generate(&name) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            let msg = err.to_string();
            quote! { compile_error!(#msg); }.into()
        }
    }
}
