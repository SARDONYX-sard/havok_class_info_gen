//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkClass`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkClass`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x75585ef6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClass<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parent"`
    /// -   type: `struct hkClass*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parent")]
    Parent(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"objectSize"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectSize")]
    ObjectSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numImplementedInterfaces"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numImplementedInterfaces")]
    NumImplementedInterfaces(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"declaredEnums"`
    /// -   type: `hkSimpleArray&lt;struct hkClassEnum&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredEnums")]
    DeclaredEnums(HkArrayClass<HkClassEnum<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"declaredMembers"`
    /// -   type: `hkSimpleArray&lt;struct hkClassMember&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredMembers")]
    DeclaredMembers(HkArrayClass<HkClassMember>),
    /// # C++ Class Fields Info
    /// -   name:`"defaults"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaults", skip_serializing)]
    Defaults(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<FlagValues>),
    /// # C++ Class Fields Info
    /// -   name:`"describedVersion"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "describedVersion")]
    DescribedVersion(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkClass<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("objectSize" => ObjectSize(Primitive<i32>)),
    ("numImplementedInterfaces" => NumImplementedInterfaces(Primitive<i32>)),
    ("declaredEnums" => DeclaredEnums(HkArrayClass<HkClassEnum<'de>>)),
    ("declaredMembers" => DeclaredMembers(HkArrayClass<HkClassMember>)),
    ("defaults" => Defaults(Primitive<Cow<'de, str>>)),
    ("attributes" => Attributes(Primitive<Cow<'de, str>>)),
    ("flags" => Flags(Primitive<FlagValues>)),
    ("describedVersion" => DescribedVersion(Primitive<i32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignatureFlags {
    #[serde(rename = "SIGNATURE_LOCAL")]
    SignatureLocal = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagValues {
    #[serde(rename = "FLAGS_NONE")]
    FlagsNone = 0,
    #[serde(rename = "FLAGS_NOT_SERIALIZABLE")]
    FlagsNotSerializable = 1,
}
