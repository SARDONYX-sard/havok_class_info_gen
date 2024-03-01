//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPoweredChainMapper`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPoweredChainMapper<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPoweredChainMapper"`: Name of this class.
    #[serde(default = "HkpPoweredChainMapper::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7a77ef5`: Unique value of this class.
    #[serde(default = "HkpPoweredChainMapper::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPoweredChainMapperHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPoweredChainMapperHkParam<'a>>
}

impl HkpPoweredChainMapper<'_> {
    /// Return `"hkpPoweredChainMapper"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPoweredChainMapper".into()
    }

    /// Return `"0x7a77ef5"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7a77ef5".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapperHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"links"`
    /// -   type: `hkArray&lt;struct hkpPoweredChainMapperLinkInfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "links")]
    Links(Vec<HkpPoweredChainMapperLinkInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targets"`
    /// -   type: `hkArray&lt;struct hkpPoweredChainMapperTarget&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targets")]
    Targets(Vec<HkpPoweredChainMapperTarget>),
    /// # Information on fields in the original C++ class
    /// -   name:`"chains"`
    /// -   type: `hkArray&lt;hkpConstraintChainInstance*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chains")]
    Chains(Vec<Box<HkpConstraintChainInstance>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperHkParam<'de>, "@name",
    ("links" => Links(Vec<HkpPoweredChainMapperLinkInfo>)),
    ("targets" => Targets(Vec<HkpPoweredChainMapperTarget>)),
    ("chains" => Chains(Vec<Box<HkpConstraintChainInstance>>)),
}
