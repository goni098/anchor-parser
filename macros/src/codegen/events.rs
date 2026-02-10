use quote::{format_ident, quote};

use super::common::{gen_discriminator, gen_type_def};
use crate::idl::Idl;

/// Generate the `events` module with event types, discriminators, and `from_logs`.
pub fn gen_events_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let events = idl.events.iter().map(|ev| {
        let name = format_ident!("{}", ev.name);
        let disc = gen_discriminator(&ev.discriminator);
        let disc_len = ev.discriminator.len();

        let ty_def = idl
            .types
            .iter()
            .find(|ty| ty.name == ev.name)
            .expect("Event type definition must exist in idl.types");

        let struct_def = gen_type_def(ty_def, &idl.types);

        quote! {
            #struct_def

            impl #name {
                pub const DISCRIMINATOR: [u8; #disc_len] = #disc;

                /// Parse all occurrences of this event from `emit!` log lines.
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
                            if data.len() < #disc_len {
                                return None;
                            }
                            if data[..#disc_len] != Self::DISCRIMINATOR {
                                return None;
                            }
                            ::anchor_parser::__private::BorshDeserialize::try_from_slice(
                                &data[#disc_len..],
                            )
                            .ok()
                        })
                        .collect()
                }

                /// Parse all occurrences of this event from `emit_cpi!` inner
                /// instruction data.
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
                            if raw.len() < CPI_TAG_LEN + #disc_len {
                                return None;
                            }
                            if raw[CPI_TAG_LEN..CPI_TAG_LEN + #disc_len] != Self::DISCRIMINATOR {
                                return None;
                            }
                            ::anchor_parser::__private::BorshDeserialize::try_from_slice(
                                &raw[CPI_TAG_LEN + #disc_len..],
                            )
                            .ok()
                        })
                        .collect()
                }
            }
        }
    });

    quote! {
        /// Program event types.
        pub mod events {
            use ::anchor_parser::__private::*;
            #[allow(unused_imports)]
            use super::types::*;

            #(#events)*
        }
    }
}
