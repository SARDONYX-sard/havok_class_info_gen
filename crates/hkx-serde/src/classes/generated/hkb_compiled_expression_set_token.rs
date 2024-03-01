//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCompiledExpressionSetToken`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCompiledExpressionSetToken<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCompiledExpressionSetToken"`: Name of this class.
    #[serde(default = "HkbCompiledExpressionSetToken::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc6aaccc8`: Unique value of this class.
    #[serde(default = "HkbCompiledExpressionSetToken::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCompiledExpressionSetTokenHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCompiledExpressionSetTokenHkParam<'a>>
}

impl HkbCompiledExpressionSetToken<'_> {
    /// Return `"hkbCompiledExpressionSetToken"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCompiledExpressionSetToken".into()
    }

    /// Return `"0xc6aaccc8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc6aaccc8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCompiledExpressionSetTokenHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum TokenType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(TokenType),
    /// # Information on fields in the original C++ class
    /// -   name:`"operator"`
    /// -   type: `enum Operator`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "operator")]
    Operator(Operator),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSetTokenHkParam<'de>, "@name",
    ("data" => Data(f64)),
    ("type" => Type(TokenType)),
    ("operator" => Operator(Operator)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "TOKEN_TYPE_NONE")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "OP_NOP")]
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
