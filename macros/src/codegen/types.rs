use quote::quote;

use super::common::gen_type_def;
use crate::idl::Idl;

/// Generate the `types` module containing non-account, non-event type definitions.
pub fn gen_types_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let types = idl
        .types
        .iter()
        .filter(|ty| {
            // Skip types that are accounts or events (they get their own modules)
            !idl.accounts.iter().any(|a| a.name == ty.name)
                && !idl.events.iter().any(|e| e.name == ty.name)
        })
        .map(|ty| gen_type_def(ty, &idl.types));

    quote! {
        /// Program type definitions (structs, enums, aliases).
        ///
        /// Account and event types are in their own dedicated modules.
        pub mod types {
            use ::anchor_parser::__private::*;

            #(#types)*
        }
    }
}
