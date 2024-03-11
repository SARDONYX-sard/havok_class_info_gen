//! A Rust structure that implements a serializer/deserializer corresponding to `hkxTriangleSelectionChannel`, a class defined in C++
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
/// -    size: 20
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxTriangleSelectionChannel<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxTriangleSelectionChannel"`: The original C++ class name.
    #[serde(default = "HkxTriangleSelectionChannel::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa02cfca9`: Unique value of this class.
    #[serde(default = "HkxTriangleSelectionChannel::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxTriangleSelectionChannelHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxTriangleSelectionChannelHkParam<'a>>
}

impl HkxTriangleSelectionChannel<'_> {
    /// Return `"hkxTriangleSelectionChannel"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxTriangleSelectionChannel".into()
    }

    /// Return `"0xa02cfca9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa02cfca9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxTriangleSelectionChannelHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"selectedTriangles"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedTriangles")]
    SelectedTriangles(Vec<Primitive<i32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxTriangleSelectionChannelHkParam<'de>, "@name",
    ("selectedTriangles" => SelectedTriangles(Vec<Primitive<i32>>)),
}
