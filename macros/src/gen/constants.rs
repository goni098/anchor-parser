use quote::{format_ident, quote};

use super::common::{convert_idl_type_to_tokens, gen_docs};
use crate::idl::{Idl, IdlType};

/// Generate the `constants` module.
pub fn gen_constants_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let consts = idl.constants.iter().map(|c| {
        let name = format_ident!("{}", c.name);
        let docs = gen_docs(&c.docs);
        let ty = convert_idl_type_to_tokens(&c.ty, true);
        let value = gen_const_value(&c.ty, &c.value);

        quote! {
            #docs
            pub const #name: #ty = #value;
        }
    });

    quote! {
        /// Program constants.
        pub mod constants {
            use ::anchor_parser::__private::*;

            #(#consts)*
        }
    }
}

fn gen_const_value(ty: &IdlType, value: &str) -> proc_macro2::TokenStream {
    match ty {
        IdlType::Pubkey => {
            // value is a base58-encoded public key
            let bytes = bs58::decode(value)
                .into_vec()
                .unwrap_or_else(|e| panic!("Invalid base58 pubkey constant '{}': {}", value, e));
            assert_eq!(
                bytes.len(),
                32,
                "Pubkey must be 32 bytes, got {}",
                bytes.len()
            );
            let byte_lits = bytes.iter().copied();
            quote! { Pubkey::new_from_array([#(#byte_lits),*]) }
        }
        IdlType::Bytes => {
            // value is e.g. "[97, 98, 99]"
            let tokens: proc_macro2::TokenStream = value.parse().unwrap();
            quote! { &#tokens }
        }
        // For everything else, the value is already a valid Rust expression
        _ => {
            let tokens: proc_macro2::TokenStream = value.parse().unwrap();
            quote! { #tokens }
        }
    }
}
