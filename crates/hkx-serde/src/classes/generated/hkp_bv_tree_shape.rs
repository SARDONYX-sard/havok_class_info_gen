//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBvTreeShape`, a class defined in C++
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
/// -  parent: hkpShape/`666490a1`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBvTreeShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBvTreeShape"`: The original C++ class name.
    #[serde(default = "HkpBvTreeShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa823d623`: Unique value of this class.
    #[serde(default = "HkpBvTreeShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBvTreeShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBvTreeShapeHkParam<'a>>
}

impl HkpBvTreeShape<'_> {
    /// Return `"hkpBvTreeShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpBvTreeShape".into()
    }

    /// Return `"0xa823d623"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa823d623".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBvTreeShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bvTreeType")]
    BvTreeType(BvTreeType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBvTreeShapeHkParam<'de>, "@name",
    ("bvTreeType" => BvTreeType(BvTreeType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BvTreeType {
    #[serde(rename = "BVTREE_MOPP")]
    BvtreeMopp = 0,
    #[serde(rename = "BVTREE_TRISAMPLED_HEIGHTFIELD")]
    BvtreeTrisampledHeightfield = 1,
    #[serde(rename = "BVTREE_USER")]
    BvtreeUser = 2,
    #[serde(rename = "BVTREE_MAX")]
    BvtreeMax = 3,
}
