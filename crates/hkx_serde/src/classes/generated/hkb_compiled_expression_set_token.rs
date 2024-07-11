//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCompiledExpressionSetToken`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbCompiledExpressionSetToken`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xc6aaccc8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCompiledExpressionSetToken {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub data: f32,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum TokenType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub _type: TokenType,
    /// # C++ Class Fields Info
    /// -   name:`"operator"`
    /// -   type: `enum Operator`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    pub operator: Operator,
}

impl Serialize for HkbCompiledExpressionSetToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCompiledExpressionSetTokenVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCompiledExpressionSetToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCompiledExpressionSetTokenVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbCompiledExpressionSetTokenVisitor>> for HkbCompiledExpressionSetToken {
    fn from(_values: Vec<HkbCompiledExpressionSetTokenVisitor>) -> Self {
            let mut data = None;
            let mut _type = None;
            let mut operator = None;


        for _value in _values {
            match _value {
                HkbCompiledExpressionSetTokenVisitor::Data(m) => data = Some(m),
                HkbCompiledExpressionSetTokenVisitor::Type(m) => _type = Some(m),
                HkbCompiledExpressionSetTokenVisitor::Operator(m) => operator = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            data: data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            operator: operator.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbCompiledExpressionSetToken> for Vec<HkbCompiledExpressionSetTokenVisitor> {
    fn from(data: &HkbCompiledExpressionSetToken) -> Self {
        vec![
            HkbCompiledExpressionSetTokenVisitor::Data(data.data.into()),
            HkbCompiledExpressionSetTokenVisitor::Type(data._type.clone().into()),
            HkbCompiledExpressionSetTokenVisitor::Operator(data.operator.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCompiledExpressionSetToken {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkbCompiledExpressionSetTokenVisitor {
    /// Visitor fields
    #[serde(rename = "data")]
    Data(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<TokenType>),
    /// Visitor fields
    #[serde(rename = "operator")]
    Operator(Primitive<Operator>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSetTokenVisitor, "@name",
    ("data" => Data(Primitive<f32>)),
    ("type" => Type(Primitive<TokenType>)),
    ("operator" => Operator(Primitive<Operator>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum TokenType {
    #[serde(rename = "TOKEN_TYPE_NONE")]
    #[default]
    TokenTypeNone = 0,
    #[serde(rename = "TOKEN_TYPE_OPERATOR")]
    TokenTypeOperator = 1,
    #[serde(rename = "TOKEN_TYPE_NUMBER")]
    TokenTypeNumber = 2,
    #[serde(rename = "TOKEN_TYPE_VARIABLE_INDEX")]
    TokenTypeVariableIndex = 3,
    #[serde(rename = "TOKEN_TYPE_OPENING_PAREN")]
    TokenTypeOpeningParen = 4,
    #[serde(rename = "TOKEN_TYPE_CLOSING_PAREN")]
    TokenTypeClosingParen = 5,
    #[serde(rename = "TOKEN_TYPE_COMMA")]
    TokenTypeComma = 6,
    #[serde(rename = "TOKEN_TYPE_CHARACTER_PROPERTY_INDEX")]
    TokenTypeCharacterPropertyIndex = 7,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Operator {
    #[serde(rename = "OP_NOP")]
    #[default]
    OpNop = 0,
    #[serde(rename = "OP_RAND01")]
    OpRand01 = 1,
    #[serde(rename = "OP_LOGICAL_NOT")]
    OpLogicalNot = 2,
    #[serde(rename = "OP_UNARY_MINUS")]
    OpUnaryMinus = 3,
    #[serde(rename = "OP_UNARY_PLUS")]
    OpUnaryPlus = 4,
    #[serde(rename = "OP_SIN")]
    OpSin = 5,
    #[serde(rename = "OP_COS")]
    OpCos = 6,
    #[serde(rename = "OP_ASIN")]
    OpAsin = 7,
    #[serde(rename = "OP_ACOS")]
    OpAcos = 8,
    #[serde(rename = "OP_SQRT")]
    OpSqrt = 9,
    #[serde(rename = "OP_FABS")]
    OpFabs = 10,
    #[serde(rename = "OP_CEIL")]
    OpCeil = 11,
    #[serde(rename = "OP_FLOOR")]
    OpFloor = 12,
    #[serde(rename = "OP_SQRTINV")]
    OpSqrtinv = 13,
    #[serde(rename = "OP_MUL")]
    OpMul = 14,
    #[serde(rename = "OP_DIV")]
    OpDiv = 15,
    #[serde(rename = "OP_ADD")]
    OpAdd = 16,
    #[serde(rename = "OP_SUB")]
    OpSub = 17,
    #[serde(rename = "OP_LOGICAL_OR")]
    OpLogicalOr = 18,
    #[serde(rename = "OP_LOGICAL_AND")]
    OpLogicalAnd = 19,
    #[serde(rename = "OP_EQ")]
    OpEq = 20,
    #[serde(rename = "OP_NEQ")]
    OpNeq = 21,
    #[serde(rename = "OP_LT")]
    OpLt = 22,
    #[serde(rename = "OP_GT")]
    OpGt = 23,
    #[serde(rename = "OP_LEQ")]
    OpLeq = 24,
    #[serde(rename = "OP_GEQ")]
    OpGeq = 25,
    #[serde(rename = "OP_POW")]
    OpPow = 26,
    #[serde(rename = "OP_MAX2")]
    OpMax2 = 27,
    #[serde(rename = "OP_MIN2")]
    OpMin2 = 28,
    #[serde(rename = "OP_RANDRANGE")]
    OpRandrange = 29,
    #[serde(rename = "OP_ATAN2APPROX")]
    OpAtan2Approx = 30,
    #[serde(rename = "OP_CLAMP")]
    OpClamp = 31,
    #[serde(rename = "OP_MOD")]
    OpMod = 32,
    #[serde(rename = "OP_DEG2RAD")]
    OpDeg2Rad = 33,
    #[serde(rename = "OP_RAD2DEG")]
    OpRad2Deg = 34,
    #[serde(rename = "OP_COSD")]
    OpCosd = 35,
    #[serde(rename = "OP_SIND")]
    OpSind = 36,
    #[serde(rename = "OP_ACOSD")]
    OpAcosd = 37,
    #[serde(rename = "OP_ASIND")]
    OpAsind = 38,
    #[serde(rename = "OP_ATAN2APPROXD")]
    OpAtan2Approxd = 39,
    #[serde(rename = "OP_SIGN")]
    OpSign = 40,
    #[serde(rename = "OP_LERP")]
    OpLerp = 41,
    #[serde(rename = "OP_CLERP")]
    OpClerp = 42,
    #[serde(rename = "OP_COND")]
    OpCond = 43,
}
