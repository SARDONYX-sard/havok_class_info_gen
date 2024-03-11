//! A Rust structure that implements a serializer/deserializer corresponding to `hkxNodeSelectionSet`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkxAttributeHolder/`7468cc44`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxNodeSelectionSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxNodeSelectionSet"`: The original C++ class name.
    #[serde(default = "HkxNodeSelectionSet::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd753fc4d`: Unique value of this class.
    #[serde(default = "HkxNodeSelectionSet::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxNodeSelectionSetHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxNodeSelectionSetHkParam<'a>>
}

impl HkxNodeSelectionSet<'_> {
    /// Return `"hkxNodeSelectionSet"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxNodeSelectionSet".into()
    }

    /// Return `"0xd753fc4d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd753fc4d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxNodeSelectionSetHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"selectedNodes"`
    /// -   type: `hkArray&lt;hkxNode*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedNodes")]
    SelectedNodes(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxNodeSelectionSetHkParam<'de>, "@name",
    ("selectedNodes" => SelectedNodes(Vec<Cow<'a, str>>)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
}
