use serde::Deserialize;

/// Top-level IDL structure (Anchor IDL spec 0.1.0).
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Idl {
    pub address: String,
    pub metadata: IdlMetadata,
    #[serde(default)]
    pub docs: Vec<String>,
    pub instructions: Vec<IdlInstruction>,
    #[serde(default)]
    pub accounts: Vec<IdlAccount>,
    #[serde(default)]
    pub events: Vec<IdlEvent>,
    #[serde(default)]
    pub errors: Vec<IdlErrorCode>,
    #[serde(default)]
    pub types: Vec<IdlTypeDef>,
    #[serde(default)]
    pub constants: Vec<IdlConst>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct IdlMetadata {
    pub name: String,
    pub version: String,
    pub spec: String,
    #[serde(default)]
    pub description: Option<String>,
}

// ── Instructions ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct IdlInstruction {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<IdlInstructionAccountItem>,
    pub args: Vec<IdlField>,
    #[serde(default)]
    pub returns: Option<IdlType>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlInstructionAccountItem {
    Composite(IdlInstructionAccounts),
    Single(IdlInstructionAccount),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct IdlInstructionAccount {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(default)]
    pub writable: bool,
    #[serde(default)]
    pub signer: bool,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct IdlInstructionAccounts {
    pub name: String,
    pub accounts: Vec<IdlInstructionAccountItem>,
}

// ── Accounts & Events (metadata only) ───────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct IdlAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlEvent {
    pub name: String,
    pub discriminator: Vec<u8>,
}

// ── Errors ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct IdlErrorCode {
    pub code: u32,
    pub name: String,
    #[serde(default)]
    pub msg: Option<String>,
}

// ── Constants ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct IdlConst {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: String,
}

// ── Fields ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct IdlField {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(rename = "type")]
    pub ty: IdlType,
}

// ── Type Definitions ─────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct IdlTypeDef {
    pub name: String,
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(default)]
    pub serialization: IdlSerialization,
    #[serde(default)]
    pub repr: Option<IdlRepr>,
    #[serde(default)]
    pub generics: Vec<IdlTypeDefGeneric>,
    #[serde(rename = "type")]
    pub ty: IdlTypeDefTy,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum IdlSerialization {
    #[default]
    Borsh,
    Bytemuck,
    BytemuckUnsafe,
    Custom(String),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlRepr {
    Rust(IdlReprModifier),
    C(IdlReprModifier),
    Transparent,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlReprModifier {
    #[serde(default)]
    pub packed: bool,
    #[serde(default)]
    pub align: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefGeneric {
    Type {
        name: String,
    },
    Const {
        name: String,
        #[serde(rename = "type")]
        ty: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefTy {
    Struct {
        #[serde(default)]
        fields: Option<IdlDefinedFields>,
    },
    Enum {
        variants: Vec<IdlEnumVariant>,
    },
    Type {
        alias: IdlType,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(default)]
    pub fields: Option<IdlDefinedFields>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum IdlDefinedFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

// ── Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdlType {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    Pubkey,
    Option(Box<IdlType>),
    Vec(Box<IdlType>),
    Array(Box<IdlType>, IdlArrayLen),
    Defined {
        name: String,
        #[serde(default)]
        generics: Vec<IdlGenericArg>,
    },
    Generic(String),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdlArrayLen {
    Generic(String),
    #[serde(untagged)]
    Value(usize),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlGenericArg {
    Type {
        #[serde(rename = "type")]
        ty: IdlType,
    },
    Const {
        value: String,
    },
}
