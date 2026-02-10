use quote::{format_ident, quote};

use super::common::{gen_discriminator, gen_type_def};
use crate::idl::{Idl, IdlSerialization};

/// Generate the `accounts` module with account types, discriminators,
/// `deserialize`, and `fetch` methods.
pub fn gen_accounts_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let accounts = idl.accounts.iter().map(|acc| {
        let name = format_ident!("{}", acc.name);
        let disc = gen_discriminator(&acc.discriminator);
        let disc_len = acc.discriminator.len();

        let ty_def = idl
            .types
            .iter()
            .find(|ty| ty.name == acc.name)
            .expect("Account type definition must exist in idl.types");

        let struct_def = gen_type_def(ty_def, &idl.types);

        let deserialize_body = match ty_def.serialization {
            IdlSerialization::Borsh => {
                quote! {
                    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
                        if data.len() < #disc_len {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Account data too short for discriminator",
                            ));
                        }
                        if data[..#disc_len] != Self::DISCRIMINATOR {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid account discriminator",
                            ));
                        }
                        BorshDeserialize::try_from_slice(&data[#disc_len..])
                    }
                }
            }
            IdlSerialization::Bytemuck | IdlSerialization::BytemuckUnsafe => {
                quote! {
                    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
                        if data.len() < #disc_len {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Account data too short for discriminator",
                            ));
                        }
                        if data[..#disc_len] != Self::DISCRIMINATOR {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid account discriminator",
                            ));
                        }
                        let payload = &data[#disc_len..];
                        let expected = std::mem::size_of::<Self>();
                        if payload.len() < expected {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Account data too short for bytemuck type",
                            ));
                        }
                        Ok(::anchor_parser::__private::bytemuck_read::<Self>(
                            &payload[..expected],
                        ))
                    }
                }
            }
            _ => {
                quote! {
                    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Unsupported serialization format",
                        ))
                    }
                }
            }
        };

        // For BytemuckUnsafe, generate unsafe Pod + Zeroable impls
        let bytemuck_impls = match ty_def.serialization {
            IdlSerialization::BytemuckUnsafe => {
                quote! {
                    unsafe impl ::anchor_parser::__private::Pod for #name {}
                    unsafe impl ::anchor_parser::__private::Zeroable for #name {}
                }
            }
            IdlSerialization::Bytemuck => {
                quote! {
                    unsafe impl ::anchor_parser::__private::Pod for #name {}
                    unsafe impl ::anchor_parser::__private::Zeroable for #name {}
                }
            }
            _ => quote!(),
        };

        quote! {
            #struct_def

            #bytemuck_impls

            impl ::anchor_parser::AccountDeserialize for #name {
                const DISCRIMINATOR: &'static [u8] = &#disc;

                #deserialize_body
            }

            impl #name {
                pub const DISCRIMINATOR: [u8; #disc_len] = #disc;

                /// Deserialize from raw account data (including discriminator prefix).
                pub fn from_account_data(data: &[u8]) -> Result<Self, std::io::Error> {
                    <Self as ::anchor_parser::AccountDeserialize>::deserialize(data)
                }
            }
        }
    });

    quote! {
        /// Program account types.
        pub mod accounts {
            use ::anchor_parser::__private::*;
            #[allow(unused_imports)]
            use super::types::*;

            #(#accounts)*
        }
    }
}
