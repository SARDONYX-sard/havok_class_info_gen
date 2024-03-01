//! A Rust structure that implements a serializer/deserializer corresponding to `hkClass`, a class defined in C++
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

    /// `"hkClass"`: Name of this class.
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
    /// Return `"hkClass"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkClass".into()
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
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"parent"`
    /// -   type: `struct hkClass*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parent")]
    Parent(Box<HkClass>),
    /// # Information on fields in the original C++ class
    /// -   name:`"objectSize"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectSize")]
    ObjectSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numImplementedInterfaces"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numImplementedInterfaces")]
    NumImplementedInterfaces(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"declaredEnums"`
    /// -   type: `hkSimpleArray&lt;struct hkClassEnum&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredEnums")]
    DeclaredEnums(Vec<HkClassEnum>),
    /// # Information on fields in the original C++ class
    /// -   name:`"declaredMembers"`
    /// -   type: `hkSimpleArray&lt;struct hkClassMember&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "declaredMembers")]
    DeclaredMembers(Vec<HkClassMember>),
    /// # Information on fields in the original C++ class
    /// -   name:`"defaults"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaults", skip_serializing)]
    Defaults(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Box<HkCustomAttributes>),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(FlagValues),
    /// # Information on fields in the original C++ class
    /// -   name:`"describedVersion"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "describedVersion")]
    DescribedVersion(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkClassHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("parent" => Parent(Box<HkClass>)),
    ("objectSize" => ObjectSize(i32)),
    ("numImplementedInterfaces" => NumImplementedInterfaces(i32)),
    ("declaredEnums" => DeclaredEnums(Vec<HkClassEnum>)),
    ("declaredMembers" => DeclaredMembers(Vec<HkClassMember>)),
    ("defaults" => Defaults(())),
    ("attributes" => Attributes(Box<HkCustomAttributes>)),
    ("flags" => Flags(FlagValues)),
    ("describedVersion" => DescribedVersion(i32)),
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
