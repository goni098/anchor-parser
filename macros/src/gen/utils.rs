use quote::{format_ident, quote};

use super::common::gen_discriminator;
use crate::idl::Idl;

/// Generate the `utils` module with `Event` and `Account` wrapper enums.
pub fn gen_utils_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let event_enum = gen_event_enum(idl);
    let account_enum = gen_account_enum(idl);

    quote! {
        /// Parser utilities.
        pub mod utils {
            use ::anchor_parser::__private::*;
            #[allow(unused_imports)]
            use super::events::*;
            #[allow(unused_imports)]
            use super::accounts::*;

            #event_enum
            #account_enum
        }
    }
}

fn gen_event_enum(idl: &Idl) -> proc_macro2::TokenStream {
    if idl.events.is_empty() {
        return quote!();
    }

    let variants: Vec<_> = idl
        .events
        .iter()
        .map(|ev| {
            let name = format_ident!("{}", ev.name);
            quote!(#name(#name))
        })
        .collect();

    let from_logs_arms: Vec<_> = idl
        .events
        .iter()
        .map(|ev| {
            let name = format_ident!("{}", ev.name);
            let disc = gen_discriminator(&ev.discriminator);
            let disc_len = ev.discriminator.len();
            quote! {
                {
                    const __DISC: [u8; #disc_len] = #disc;
                    if data.len() >= #disc_len && data[..#disc_len] == __DISC {
                        if let Ok(inner) = ::anchor_parser::__private::BorshDeserialize::try_from_slice(
                            &data[#disc_len..],
                        ) {
                            return Some(Event::#name(inner));
                        }
                    }
                }
            }
        })
        .collect();

    let from_cpi_arms: Vec<_> = idl
        .events
        .iter()
        .map(|ev| {
            let name = format_ident!("{}", ev.name);
            let disc = gen_discriminator(&ev.discriminator);
            let disc_len = ev.discriminator.len();
            quote! {
                {
                    const __DISC: [u8; #disc_len] = #disc;
                    if data.len() >= #disc_len && data[..#disc_len] == __DISC {
                        if let Ok(inner) = ::anchor_parser::__private::BorshDeserialize::try_from_slice(
                            &data[#disc_len..],
                        ) {
                            return Some(Event::#name(inner));
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        /// Enum wrapping all program event types.
        #[derive(Debug, Clone)]
        pub enum Event {
            #(#variants,)*
        }

        impl Event {
            /// Parse all program events from `emit!` log lines.
            ///
            /// Matches every `"Program data: <base64>"` line regardless of
            /// which program emitted it.
            pub fn from_logs<T, I>(logs: T) -> Vec<Self>
            where
                T: IntoIterator<Item = I>,
                I: AsRef<str>,
            {
                logs.into_iter()
                    .filter_map(|log| {
                        let data_str = log.as_ref().strip_prefix("Program data: ")?;
                        let data = ::anchor_parser::__private::base64_decode(data_str)?;
                        #(#from_logs_arms)*
                        None
                    })
                    .collect()
            }

            /// Parse all program events from `emit_cpi!` inner instruction data.
            ///
            /// Each entry should be the **bs58-encoded `data`** field from
            /// an inner instruction (i.e.
            /// `tx.meta.innerInstructions[].instructions[].data`).
            ///
            /// The expected binary layout is:
            /// `[8-byte CPI event tag][8-byte event discriminator][borsh payload]`
            pub fn from_cpi_logs<T, I>(logs: T) -> Vec<Self>
            where
                T: IntoIterator<Item = I>,
                I: AsRef<str>,
            {
                const CPI_TAG_LEN: usize = 8;
                logs.into_iter()
                    .filter_map(|log| {
                        let raw = ::anchor_parser::__private::bs58_decode(log.as_ref())?;
                        if raw.len() < CPI_TAG_LEN {
                            return None;
                        }
                        let data = &raw[CPI_TAG_LEN..];
                        #(#from_cpi_arms)*
                        None
                    })
                    .collect()
            }
        }
    }
}

fn gen_account_enum(idl: &Idl) -> proc_macro2::TokenStream {
    if idl.accounts.is_empty() {
        return quote!();
    }

    let variants: Vec<_> = idl
        .accounts
        .iter()
        .map(|acc| {
            let name = format_ident!("{}", acc.name);
            quote!(#name(#name))
        })
        .collect();

    let parse_arms: Vec<_> = idl
        .accounts
        .iter()
        .map(|acc| {
            let name = format_ident!("{}", acc.name);
            let disc = gen_discriminator(&acc.discriminator);
            let disc_len = acc.discriminator.len();
            quote! {
                {
                    const __DISC: [u8; #disc_len] = #disc;
                    if data.len() >= #disc_len && data[..#disc_len] == __DISC {
                        if let Ok(inner) = <#name as ::anchor_parser::AccountDeserialize>::deserialize(data) {
                            return Ok(Account::#name(inner));
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        /// Enum wrapping all program account types.
        #[derive(Debug, Clone)]
        pub enum Account {
            #(#variants,)*
        }

        impl Account {
            /// Parse an account from raw account data by testing all known discriminators.
            pub fn parse(data: &[u8]) -> Result<Self, std::io::Error> {
                #(#parse_arms)*
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unknown account discriminator",
                ))
            }
        }
    }
}
