//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryResourceContainer`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkResourceContainer/`4e94146`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryResourceContainer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryResourceContainer"`: Name of this class.
    #[serde(default = "HkMemoryResourceContainer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4762f92a`: Unique value of this class.
    #[serde(default = "HkMemoryResourceContainer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryResourceContainerHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryResourceContainerHkParam<'a>>
}

impl HkMemoryResourceContainer<'_> {
    /// Return `"hkMemoryResourceContainer"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMemoryResourceContainer".into()
    }

    /// Return `"0x4762f92a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4762f92a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceContainerHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"parent"`
    /// -   type: `struct hkMemoryResourceContainer*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parent", skip_serializing)]
    Parent(Box<HkMemoryResourceContainer>),
    /// # Information on fields in the original C++ class
    /// -   name:`"resourceHandles"`
    /// -   type: `hkArray&lt;hkMemoryResourceHandle*&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resourceHandles")]
    ResourceHandles(Vec<Box<HkMemoryResourceHandle>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkMemoryResourceContainer*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(Vec<Box<HkMemoryResourceContainer>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceContainerHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("parent" => Parent(Box<HkMemoryResourceContainer>)),
    ("resourceHandles" => ResourceHandles(Vec<Box<HkMemoryResourceHandle>>)),
    ("children" => Children(Vec<Box<HkMemoryResourceContainer>>)),
}
