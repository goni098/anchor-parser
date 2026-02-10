use proc_macro2::Literal;
use quote::{format_ident, quote};

use crate::idl::*;

// ── Type conversion ──────────────────────────────────────────────────

/// Convert an `IdlType` to a `syn::Type` token stream.
/// When `is_const` is true, `String` → `&'static str`, `Bytes` → `&'static [u8]`.
pub fn convert_idl_type_to_tokens(ty: &IdlType, is_const: bool) -> proc_macro2::TokenStream {
    match ty {
        IdlType::Bool => quote!(bool),
        IdlType::U8 => quote!(u8),
        IdlType::I8 => quote!(i8),
        IdlType::U16 => quote!(u16),
        IdlType::I16 => quote!(i16),
        IdlType::U32 => quote!(u32),
        IdlType::I32 => quote!(i32),
        IdlType::F32 => quote!(f32),
        IdlType::U64 => quote!(u64),
        IdlType::I64 => quote!(i64),
        IdlType::F64 => quote!(f64),
        IdlType::U128 => quote!(u128),
        IdlType::I128 => quote!(i128),
        IdlType::U256 => quote!([u8; 32]), // u256 not native, represent as bytes
        IdlType::I256 => quote!([u8; 32]),
        IdlType::Bytes => {
            if is_const {
                quote!(&'static [u8])
            } else {
                quote!(Vec<u8>)
            }
        }
        IdlType::String => {
            if is_const {
                quote!(&'static str)
            } else {
                quote!(String)
            }
        }
        IdlType::Pubkey => quote!(Pubkey),
        IdlType::Option(inner) => {
            let inner = convert_idl_type_to_tokens(inner, is_const);
            quote!(Option<#inner>)
        }
        IdlType::Vec(inner) => {
            let inner = convert_idl_type_to_tokens(inner, is_const);
            quote!(Vec<#inner>)
        }
        IdlType::Array(inner, len) => {
            let inner = convert_idl_type_to_tokens(inner, is_const);
            let len_tok = match len {
                IdlArrayLen::Value(n) => {
                    let lit = Literal::usize_unsuffixed(*n);
                    quote!(#lit)
                }
                IdlArrayLen::Generic(g) => {
                    let ident = format_ident!("{}", g);
                    quote!(#ident)
                }
            };
            quote!([#inner; #len_tok])
        }
        IdlType::Defined { name, generics } => {
            let name_ident = format_ident!("{}", name);
            if generics.is_empty() {
                quote!(#name_ident)
            } else {
                let generic_args = generics.iter().map(|g| match g {
                    IdlGenericArg::Type { ty } => convert_idl_type_to_tokens(ty, is_const),
                    IdlGenericArg::Const { value } => {
                        let tokens: proc_macro2::TokenStream = value.parse().unwrap();
                        quote!(#tokens)
                    }
                });
                quote!(#name_ident<#(#generic_args),*>)
            }
        }
        IdlType::Generic(name) => {
            let ident = format_ident!("{}", name);
            quote!(#ident)
        }
    }
}

// ── Discriminator literal ────────────────────────────────────────────

pub fn gen_discriminator(disc: &[u8]) -> proc_macro2::TokenStream {
    let bytes = disc.iter().copied();
    quote! { [#(#bytes),*] }
}

// ── Documentation ────────────────────────────────────────────────────

pub fn gen_docs(docs: &[String]) -> proc_macro2::TokenStream {
    let doc_attrs = docs.iter().map(|doc| {
        let s = format!("{}{doc}", if doc.is_empty() { "" } else { " " });
        quote!(#[doc = #s])
    });
    quote!(#(#doc_attrs)*)
}

// ── Type definition generation ───────────────────────────────────────

/// Generate a struct/enum/type-alias from an `IdlTypeDef`.
pub fn gen_type_def(ty_def: &IdlTypeDef, all_ty_defs: &[IdlTypeDef]) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", ty_def.name);
    let docs = gen_docs(&ty_def.docs);

    // Generics
    let generics = {
        let params: Vec<_> = ty_def
            .generics
            .iter()
            .map(|g| match g {
                IdlTypeDefGeneric::Type { name } => {
                    let n = format_ident!("{}", name);
                    quote!(#n)
                }
                IdlTypeDefGeneric::Const { name, ty } => {
                    let n = format_ident!("{}", name);
                    let t = format_ident!("{}", ty);
                    quote!(const #n: #t)
                }
            })
            .collect();
        if params.is_empty() {
            quote!()
        } else {
            quote!(<#(#params),*>)
        }
    };

    // Derives & attributes
    let (derives, repr_attr) = gen_derives_and_repr(ty_def, all_ty_defs);

    match &ty_def.ty {
        IdlTypeDefTy::Struct { fields } => {
            let body = match fields {
                None => quote!(;),
                Some(IdlDefinedFields::Named(fields)) => {
                    let fs = fields.iter().map(|f| {
                        let fname = format_ident!("{}", f.name);
                        let fty = convert_idl_type_to_tokens(&f.ty, false);
                        let fdocs = gen_docs(&f.docs);
                        quote! { #fdocs pub #fname: #fty }
                    });
                    quote!({ #(#fs,)* })
                }
                Some(IdlDefinedFields::Tuple(tys)) => {
                    let fs = tys.iter().map(|t| {
                        let ty = convert_idl_type_to_tokens(t, false);
                        quote!(pub #ty)
                    });
                    quote!((#(#fs,)*);)
                }
            };
            quote! {
                #docs
                #derives
                #repr_attr
                pub struct #name #generics #body
            }
        }
        IdlTypeDefTy::Enum { variants } => {
            let vs = variants.iter().map(|v| {
                let vname = format_ident!("{}", v.name);
                match &v.fields {
                    None => quote!(#vname),
                    Some(IdlDefinedFields::Named(fields)) => {
                        let fs = fields.iter().map(|f| {
                            let fname = format_ident!("{}", f.name);
                            let fty = convert_idl_type_to_tokens(&f.ty, false);
                            quote!(#fname: #fty)
                        });
                        quote!(#vname { #(#fs,)* })
                    }
                    Some(IdlDefinedFields::Tuple(tys)) => {
                        let fs = tys.iter().map(|t| convert_idl_type_to_tokens(t, false));
                        quote!(#vname(#(#fs,)*))
                    }
                }
            });
            quote! {
                #docs
                #derives
                #repr_attr
                pub enum #name #generics {
                    #(#vs,)*
                }
            }
        }
        IdlTypeDefTy::Type { alias } => {
            let alias_ty = convert_idl_type_to_tokens(alias, false);
            quote! {
                #docs
                pub type #name #generics = #alias_ty;
            }
        }
    }
}

fn gen_derives_and_repr(
    ty_def: &IdlTypeDef,
    all_ty_defs: &[IdlTypeDef],
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let is_enum = matches!(ty_def.ty, IdlTypeDefTy::Enum { .. });

    let mut derive_items = vec![quote!(Debug)];

    if !is_enum && can_derive_default(ty_def, all_ty_defs) {
        derive_items.push(quote!(Default));
    }

    match &ty_def.serialization {
        IdlSerialization::Borsh => {
            derive_items.push(quote!(::anchor_parser::__private::BorshSerialize));
            derive_items.push(quote!(::anchor_parser::__private::BorshDeserialize));
            derive_items.push(quote!(Clone));
            if can_derive_copy(ty_def, all_ty_defs) {
                derive_items.push(quote!(Copy));
            }
        }
        IdlSerialization::Bytemuck | IdlSerialization::BytemuckUnsafe => {
            derive_items.push(quote!(Clone));
            derive_items.push(quote!(Copy));
        }
        _ => {}
    };

    let derives = quote!(#[derive(#(#derive_items),*)]);

    let repr = ty_def
        .repr
        .as_ref()
        .map(|repr| {
            let kind = match repr {
                IdlRepr::Rust(_) => format_ident!("Rust"),
                IdlRepr::C(_) => format_ident!("C"),
                IdlRepr::Transparent => format_ident!("transparent"),
            };
            let modifier = match repr {
                IdlRepr::Rust(m) | IdlRepr::C(m) => {
                    let packed = m.packed.then_some(quote!(packed));
                    let align = m
                        .align
                        .map(Literal::usize_unsuffixed)
                        .map(|a| quote!(align(#a)));
                    match (packed, align) {
                        (None, None) => None,
                        (Some(p), None) => Some(quote!(, #p)),
                        (None, Some(a)) => Some(quote!(, #a)),
                        (Some(p), Some(a)) => Some(quote!(, #p, #a)),
                    }
                }
                _ => None,
            };
            quote!(#[repr(#kind #modifier)])
        })
        .unwrap_or_else(|| {
            // Default repr(C) for bytemuck types
            if matches!(
                ty_def.serialization,
                IdlSerialization::Bytemuck | IdlSerialization::BytemuckUnsafe
            ) {
                quote!(#[repr(C)])
            } else {
                quote!()
            }
        });

    (derives, repr)
}

// ── Copy / Default derivability checks ───────────────────────────────

fn can_derive_copy(ty_def: &IdlTypeDef, all: &[IdlTypeDef]) -> bool {
    match &ty_def.ty {
        IdlTypeDefTy::Struct { fields } => check_fields(fields.as_ref(), all, can_copy_ty),
        IdlTypeDefTy::Enum { variants } => variants
            .iter()
            .all(|v| check_fields(v.fields.as_ref(), all, can_copy_ty)),
        IdlTypeDefTy::Type { alias } => can_copy_ty(alias, all),
    }
}

fn can_derive_default(ty_def: &IdlTypeDef, all: &[IdlTypeDef]) -> bool {
    match &ty_def.ty {
        IdlTypeDefTy::Struct { fields } => check_fields(fields.as_ref(), all, can_default_ty),
        IdlTypeDefTy::Enum { .. } => false,
        IdlTypeDefTy::Type { alias } => can_default_ty(alias, all),
    }
}

fn can_copy_ty(ty: &IdlType, all: &[IdlTypeDef]) -> bool {
    match ty {
        IdlType::Option(inner) => can_copy_ty(inner, all),
        IdlType::Array(inner, len) => {
            can_copy_ty(inner, all) && matches!(len, IdlArrayLen::Value(_))
        }
        IdlType::Defined { name, .. } => all
            .iter()
            .find(|d| &d.name == name)
            .map(|d| can_derive_copy(d, all))
            .unwrap_or(false),
        IdlType::Bytes | IdlType::String | IdlType::Vec(_) | IdlType::Generic(_) => false,
        _ => true,
    }
}

fn can_default_ty(ty: &IdlType, all: &[IdlTypeDef]) -> bool {
    match ty {
        IdlType::Option(inner) | IdlType::Vec(inner) => can_default_ty(inner, all),
        IdlType::Array(inner, len) => {
            can_default_ty(inner, all)
                && match len {
                    IdlArrayLen::Value(n) => *n <= 32,
                    IdlArrayLen::Generic(_) => false,
                }
        }
        IdlType::Defined { name, .. } => all
            .iter()
            .find(|d| &d.name == name)
            .map(|d| can_derive_default(d, all))
            .unwrap_or(false),
        IdlType::Generic(_) => false,
        _ => true,
    }
}

fn check_fields(
    fields: Option<&IdlDefinedFields>,
    all: &[IdlTypeDef],
    predicate: fn(&IdlType, &[IdlTypeDef]) -> bool,
) -> bool {
    match fields {
        None => true,
        Some(IdlDefinedFields::Named(fs)) => fs.iter().all(|f| predicate(&f.ty, all)),
        Some(IdlDefinedFields::Tuple(ts)) => ts.iter().all(|t| predicate(t, all)),
    }
}

// ── Flatten instruction accounts ─────────────────────────────────────

pub struct FlatAccount {
    pub name: String,
    pub writable: bool,
    pub signer: bool,
    pub optional: bool,
}

/// Recursively flatten composite accounts into a flat list.
/// Composite field names are prefixed: `composite_fieldname`.
pub fn flatten_accounts(items: &[IdlInstructionAccountItem], prefix: &str) -> Vec<FlatAccount> {
    items
        .iter()
        .flat_map(|item| match item {
            IdlInstructionAccountItem::Single(acc) => {
                let name = if prefix.is_empty() {
                    acc.name.clone()
                } else {
                    format!("{}_{}", prefix, acc.name)
                };
                vec![FlatAccount {
                    name,
                    writable: acc.writable,
                    signer: acc.signer,
                    optional: acc.optional,
                }]
            }
            IdlInstructionAccountItem::Composite(comp) => {
                let new_prefix = if prefix.is_empty() {
                    comp.name.clone()
                } else {
                    format!("{}_{}", prefix, comp.name)
                };
                flatten_accounts(&comp.accounts, &new_prefix)
            }
        })
        .collect()
}
