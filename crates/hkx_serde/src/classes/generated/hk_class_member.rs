//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkClassMember`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkClassMember`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0x5c7ea4c2`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClassMember<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"class"`
    /// -   type: `struct hkClass*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "class")]
    Class(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"enum"`
    /// -   type: `struct hkClassEnum*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Type`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<Type>),
    /// # C++ Class Fields Info
    /// -   name:`"subtype"`
    /// -   type: `enum Type`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subtype")]
    Subtype(Primitive<Type>),
    /// # C++ Class Fields Info
    /// -   name:`"cArraySize"`
    /// -   type: `hkInt16`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cArraySize")]
    CArraySize(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<FlagValues>),
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkClassMember<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("class" => Class(Primitive<Cow<'de, str>>)),
    ("enum" => Enum(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<Type>)),
    ("subtype" => Subtype(Primitive<Type>)),
    ("cArraySize" => CArraySize(Primitive<i16>)),
    ("flags" => Flags(Primitive<FlagValues>)),
    ("offset" => Offset(Primitive<u16>)),
    ("attributes" => Attributes(Primitive<Cow<'de, str>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
