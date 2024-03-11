//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryResourceHandle`, a class defined in C++
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
/// -    size: 28
/// -  vtable: true
/// -  parent: hkResourceHandle/`4e94146`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryResourceHandle<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryResourceHandle"`: The original C++ class name.
    #[serde(default = "HkMemoryResourceHandle::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xbffac086`: Unique value of this class.
    #[serde(default = "HkMemoryResourceHandle::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryResourceHandleHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryResourceHandleHkParam<'a>>
}

impl HkMemoryResourceHandle<'_> {
    /// Return `"hkMemoryResourceHandle"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryResourceHandle".into()
    }

    /// Return `"0xbffac086"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xbffac086".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceHandleHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variant")]
    Variant(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"references"`
    /// -   type: `hkArray&lt;struct hkMemoryResourceHandleExternalLink&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "references")]
    References(Vec<HkMemoryResourceHandleExternalLink>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceHandleHkParam<'de>, "@name",
    ("variant" => Variant(Cow<'a, str>)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("references" => References(Vec<HkMemoryResourceHandleExternalLink>)),
}
