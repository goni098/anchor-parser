use heck::ToUpperCamelCase;
use quote::{format_ident, quote};

use super::common::{convert_idl_type_to_tokens, flatten_accounts, gen_discriminator, gen_docs};
use crate::idl::Idl;

/// Generate the `instructions` module with account structs and builder functions.
pub fn gen_instructions_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let instructions = idl.instructions.iter().map(|ix| {
        let fn_name = format_ident!("{}", ix.name);
        let accounts_struct_name = format_ident!("{}Accounts", ix.name.to_upper_camel_case());
        let disc = gen_discriminator(&ix.discriminator);
        let docs = gen_docs(&ix.docs);
        // Flatten accounts
        let flat = flatten_accounts(&ix.accounts, "");

        // Generate accounts struct fields
        let acc_fields: Vec<_> = flat
            .iter()
            .map(|a| {
                let field_name = format_ident!("{}", a.name);
                if a.optional {
                    quote! { pub #field_name: Option<Pubkey> }
                } else {
                    quote! { pub #field_name: Pubkey }
                }
            })
            .collect();

        let accounts_struct = if acc_fields.is_empty() {
            quote! {
                #[derive(Debug, Clone)]
                pub struct #accounts_struct_name;
            }
        } else {
            quote! {
                #[derive(Debug, Clone)]
                pub struct #accounts_struct_name {
                    #(#acc_fields,)*
                }
            }
        };

        // Generate AccountMeta entries
        let account_metas: Vec<_> = flat
            .iter()
            .map(|a| {
                let field_name = format_ident!("{}", a.name);
                let writable = a.writable;
                let signer = a.signer;

                if a.optional {
                    // For optional accounts, use program ID as placeholder when None
                    if writable {
                        quote! {
                            match accounts.#field_name {
                                Some(key) => AccountMeta::new(key, #signer),
                                None => AccountMeta::new_readonly(*program_id, false),
                            }
                        }
                    } else {
                        quote! {
                            match accounts.#field_name {
                                Some(key) => AccountMeta::new_readonly(key, #signer),
                                None => AccountMeta::new_readonly(*program_id, false),
                            }
                        }
                    }
                } else if writable {
                    quote! { AccountMeta::new(accounts.#field_name, #signer) }
                } else {
                    quote! { AccountMeta::new_readonly(accounts.#field_name, #signer) }
                }
            })
            .collect();

        // Builder function parameters (instruction args)
        let fn_params: Vec<_> = ix
            .args
            .iter()
            .map(|arg| {
                let name = format_ident!("{}", arg.name);
                let ty = convert_idl_type_to_tokens(&arg.ty, false);
                quote! { #name: #ty }
            })
            .collect();

        // Serialize instruction data
        let serialize_args: Vec<_> = ix
            .args
            .iter()
            .map(|arg| {
                let name = format_ident!("{}", arg.name);
                quote! {
                    BorshSerialize::serialize(&#name, &mut __ix_data)
                        .expect("Failed to serialize instruction argument");
                }
            })
            .collect();

        let accounts_param = if acc_fields.is_empty() {
            quote!()
        } else {
            quote!(accounts: &#accounts_struct_name,)
        };

        let account_metas_expr = if account_metas.is_empty() {
            quote!(Vec::new())
        } else {
            quote!(vec![#(#account_metas),*])
        };

        let builder_fn = quote! {
            #docs
            pub fn #fn_name(
                program_id: &Pubkey,
                #accounts_param
                #(#fn_params,)*
            ) -> Instruction {
                let mut __ix_data = Vec::with_capacity(256);
                // Write discriminator
                let __disc: &[u8] = &#disc;
                __ix_data.extend_from_slice(__disc);
                // Serialize args
                #(#serialize_args)*

                Instruction {
                    program_id: *program_id,
                    accounts: #account_metas_expr,
                    data: __ix_data,
                }
            }
        };

        quote! {
            #accounts_struct
            #builder_fn
        }
    });

    quote! {
        /// Instruction builders.
        pub mod instructions {
            use ::anchor_parser::__private::*;
            #[allow(unused_imports)]
            use super::types::*;

            #(#instructions)*
        }
    }
}
