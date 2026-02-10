mod accounts;
mod common;
mod constants;
mod events;
mod instructions;
mod types;
mod utils;

use quote::{format_ident, quote};
use std::path::PathBuf;

use crate::idl::Idl;

/// Load the IDL JSON and generate the complete program module.
pub fn generate(name: &syn::Ident) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let idl = load_idl(&name.to_string())?;

    let mod_name = format_ident!("{}", name);

    // Decode program address to bytes for Pubkey::new_from_array
    let address_bytes = bs58::decode(&idl.address).into_vec().map_err(|e| {
        format!(
            "Invalid program address '{}' in IDL: {}",
            idl.address, e
        )
    })?;
    if address_bytes.len() != 32 {
        return Err(format!(
            "Program address must be 32 bytes, got {}",
            address_bytes.len()
        )
        .into());
    }
    let addr_bytes = address_bytes.iter().copied();

    let types_mod = types::gen_types_mod(&idl);
    let accounts_mod = accounts::gen_accounts_mod(&idl);
    let events_mod = events::gen_events_mod(&idl);
    let instructions_mod = instructions::gen_instructions_mod(&idl);
    let constants_mod = constants::gen_constants_mod(&idl);
    let utils_mod = utils::gen_utils_mod(&idl);

    let output = quote! {
        #[allow(dead_code, unused_imports, unused_variables, clippy::all)]
        pub mod #mod_name {
            /// Program ID.
            pub const ID: ::anchor_parser::__private::Pubkey =
                ::anchor_parser::__private::Pubkey::new_from_array([#(#addr_bytes),*]);

            #types_mod
            #accounts_mod
            #events_mod
            #instructions_mod
            #constants_mod
            #utils_mod
        }
    };

    Ok(output)
}

/// Find and parse the IDL JSON file.
///
/// Walks up from `CARGO_MANIFEST_DIR` looking for `idls/{name}.json`.
fn load_idl(name: &str) -> Result<Idl, Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        "CARGO_MANIFEST_DIR not set. This macro must be invoked during cargo build."
    })?;

    let mut dir = PathBuf::from(&manifest_dir);
    loop {
        let idl_path = dir.join("idls").join(format!("{name}.json"));
        if idl_path.exists() {
            let content = std::fs::read_to_string(&idl_path).map_err(|e| {
                format!("Failed to read IDL file '{}': {}", idl_path.display(), e)
            })?;
            let idl: Idl = serde_json::from_str(&content).map_err(|e| {
                format!("Failed to parse IDL file '{}': {}", idl_path.display(), e)
            })?;
            return Ok(idl);
        }
        if !dir.pop() {
            break;
        }
    }

    Err(format!(
        "Could not find IDL file 'idls/{name}.json'. \
         Searched from '{manifest_dir}' upward. \
         Place your IDL JSON file at '<workspace>/idls/{name}.json'."
    )
    .into())
}
