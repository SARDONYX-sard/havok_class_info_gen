//! A Rust structure that implements a serializer/deserializer corresponding to `hkClass`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 48
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkClass<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkClass"`: The original C++ class name.
    #[serde(default = "HkClass::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x75585ef6`: Unique value of this class.
    #[serde(default = "HkClass::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkClassHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkClassHkParam<'a>>
}

impl HkClass<'_> {
    /// Return `"hkClass"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkClass".into()
    }

    /// Return `"0x75585ef6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x75585ef6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClassHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"parent"`
    /// -   type: `struct hkClass*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parent")]
    Parent(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"objectSize"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectSize")]
    ObjectSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"numImplementedInterfaces"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numImplementedInterfaces")]
    NumImplementedInterfaces(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"declaredEnums"`
    /// -   type: `hkSimpleArray&lt;struct hkClassEnum&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredEnums")]
    DeclaredEnums(Vec<HkClassEnum>),
    /// # Field information in the original C++ class
    /// -   name:`"declaredMembers"`
    /// -   type: `hkSimpleArray&lt;struct hkClassMember&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredMembers")]
    DeclaredMembers(Vec<HkClassMember>),
    /// # Field information in the original C++ class
    /// -   name:`"defaults"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaults", skip_serializing)]
    Defaults(()),
    /// # Field information in the original C++ class
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(FlagValues),
    /// # Field information in the original C++ class
    /// -   name:`"describedVersion"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "describedVersion")]
    DescribedVersion(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkClassHkParam<'de>, "@name",
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("parent" => Parent(Cow<'a, str>)),
    ("objectSize" => ObjectSize(Primitive<i32>)),
    ("numImplementedInterfaces" => NumImplementedInterfaces(Primitive<i32>)),
    ("declaredEnums" => DeclaredEnums(Vec<HkClassEnum>)),
    ("declaredMembers" => DeclaredMembers(Vec<HkClassMember>)),
    ("defaults" => Defaults(())),
    ("attributes" => Attributes(Cow<'a, str>)),
    ("flags" => Flags(FlagValues)),
    ("describedVersion" => DescribedVersion(Primitive<i32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SignatureFlags {
    #[serde(rename = "SIGNATURE_LOCAL")]
    SignatureLocal = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagValues {
    #[serde(rename = "FLAGS_NONE")]
    FlagsNone = 0,
    #[serde(rename = "FLAGS_NOT_SERIALIZABLE")]
    FlagsNotSerializable = 1,
}
