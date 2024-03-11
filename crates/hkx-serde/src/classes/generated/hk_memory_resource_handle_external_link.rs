//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryResourceHandleExternalLink`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryResourceHandleExternalLink<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryResourceHandleExternalLink"`: The original C++ class name.
    #[serde(default = "HkMemoryResourceHandleExternalLink::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3144d17c`: Unique value of this class.
    #[serde(default = "HkMemoryResourceHandleExternalLink::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryResourceHandleExternalLinkHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryResourceHandleExternalLinkHkParam<'a>>
}

impl HkMemoryResourceHandleExternalLink<'_> {
    /// Return `"hkMemoryResourceHandleExternalLink"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryResourceHandleExternalLink".into()
    }

    /// Return `"0x3144d17c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3144d17c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceHandleExternalLinkHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"memberName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memberName")]
    MemberName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"externalId"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "externalId")]
    ExternalId(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceHandleExternalLinkHkParam<'de>, "@name",
    ("memberName" => MemberName(Primitive<Cow<'a, str>>)),
    ("externalId" => ExternalId(Primitive<Cow<'a, str>>)),
}
