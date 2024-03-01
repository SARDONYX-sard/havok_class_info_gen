//! A Rust structure that implements a serializer/deserializer corresponding to `hkClassMember`, a class defined in C++
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
/// -    size: 24
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkClassMember<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkClassMember"`: Name of this class.
    #[serde(default = "HkClassMember::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5c7ea4c2`: Unique value of this class.
    #[serde(default = "HkClassMember::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkClassMemberHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkClassMemberHkParam<'a>>
}

impl HkClassMember<'_> {
    /// Return `"hkClassMember"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkClassMember".into()
    }

    /// Return `"0x5c7ea4c2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5c7ea4c2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClassMemberHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"class"`
    /// -   type: `struct hkClass*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "class")]
    Class(Box<HkClass>),
    /// # Information on fields in the original C++ class
    /// -   name:`"enum"`
    /// -   type: `struct hkClassEnum*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Box<HkClassEnum>),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum Type`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Type),
    /// # Information on fields in the original C++ class
    /// -   name:`"subtype"`
    /// -   type: `enum Type`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subtype")]
    Subtype(Type),
    /// # Information on fields in the original C++ class
    /// -   name:`"cArraySize"`
    /// -   type: `hkInt16`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cArraySize")]
    CArraySize(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(FlagValues),
    /// # Information on fields in the original C++ class
    /// -   name:`"offset"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Box<HkCustomAttributes>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkClassMemberHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("class" => Class(Box<HkClass>)),
    ("enum" => Enum(Box<HkClassEnum>)),
    ("type" => Type(Type)),
    ("subtype" => Subtype(Type)),
    ("cArraySize" => CArraySize(i16)),
    ("flags" => Flags(FlagValues)),
    ("offset" => Offset(u16)),
    ("attributes" => Attributes(Box<HkCustomAttributes>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TYPE_VOID")]
    TypeVoid = 0,
    #[serde(rename = "TYPE_BOOL")]
    TypeBool = 1,
    #[serde(rename = "TYPE_CHAR")]
    TypeChar = 2,
    #[serde(rename = "TYPE_INT8")]
    TypeInt8 = 3,
    #[serde(rename = "TYPE_UINT8")]
    TypeUint8 = 4,
    #[serde(rename = "TYPE_INT16")]
    TypeInt16 = 5,
    #[serde(rename = "TYPE_UINT16")]
    TypeUint16 = 6,
    #[serde(rename = "TYPE_INT32")]
    TypeInt32 = 7,
    #[serde(rename = "TYPE_UINT32")]
    TypeUint32 = 8,
    #[serde(rename = "TYPE_INT64")]
    TypeInt64 = 9,
    #[serde(rename = "TYPE_UINT64")]
    TypeUint64 = 10,
    #[serde(rename = "TYPE_REAL")]
    TypeReal = 11,
    #[serde(rename = "TYPE_VECTOR4")]
    TypeVector4 = 12,
    #[serde(rename = "TYPE_QUATERNION")]
    TypeQuaternion = 13,
    #[serde(rename = "TYPE_MATRIX3")]
    TypeMatrix3 = 14,
    #[serde(rename = "TYPE_ROTATION")]
    TypeRotation = 15,
    #[serde(rename = "TYPE_QSTRANSFORM")]
    TypeQstransform = 16,
    #[serde(rename = "TYPE_MATRIX4")]
    TypeMatrix4 = 17,
    #[serde(rename = "TYPE_TRANSFORM")]
    TypeTransform = 18,
    #[serde(rename = "TYPE_ZERO")]
    TypeZero = 19,
    #[serde(rename = "TYPE_POINTER")]
    TypePointer = 20,
    #[serde(rename = "TYPE_FUNCTIONPOINTER")]
    TypeFunctionpointer = 21,
    #[serde(rename = "TYPE_ARRAY")]
    TypeArray = 22,
    #[serde(rename = "TYPE_INPLACEARRAY")]
    TypeInplacearray = 23,
    #[serde(rename = "TYPE_ENUM")]
    TypeEnum = 24,
    #[serde(rename = "TYPE_STRUCT")]
    TypeStruct = 25,
    #[serde(rename = "TYPE_SIMPLEARRAY")]
    TypeSimplearray = 26,
    #[serde(rename = "TYPE_HOMOGENEOUSARRAY")]
    TypeHomogeneousarray = 27,
    #[serde(rename = "TYPE_VARIANT")]
    TypeVariant = 28,
    #[serde(rename = "TYPE_CSTRING")]
    TypeCstring = 29,
    #[serde(rename = "TYPE_ULONG")]
    TypeUlong = 30,
    #[serde(rename = "TYPE_FLAGS")]
    TypeFlags = 31,
    #[serde(rename = "TYPE_HALF")]
    TypeHalf = 32,
    #[serde(rename = "TYPE_STRINGPTR")]
    TypeStringptr = 33,
    #[serde(rename = "TYPE_RELARRAY")]
    TypeRelarray = 34,
    #[serde(rename = "TYPE_MAX")]
    TypeMax = 35,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagValues {
    #[serde(rename = "FLAGS_NONE")]
    FlagsNone = 0,
    #[serde(rename = "ALIGN_8")]
    Align8 = 128,
    #[serde(rename = "ALIGN_16")]
    Align16 = 256,
    #[serde(rename = "NOT_OWNED")]
    NotOwned = 512,
    #[serde(rename = "SERIALIZE_IGNORED")]
    SerializeIgnored = 1024,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeprecatedFlagValues {
    #[serde(rename = "DEPRECATED_SIZE_8")]
    DeprecatedSize8 = 8,
    #[serde(rename = "DEPRECATED_ENUM_8")]
    DeprecatedEnum8 = 8,
    #[serde(rename = "DEPRECATED_SIZE_16")]
    DeprecatedSize16 = 16,
    #[serde(rename = "DEPRECATED_ENUM_16")]
    DeprecatedEnum16 = 16,
    #[serde(rename = "DEPRECATED_SIZE_32")]
    DeprecatedSize32 = 32,
    #[serde(rename = "DEPRECATED_ENUM_32")]
    DeprecatedEnum32 = 32,
}
