//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStorageSampledHeightFieldShape`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkpSampledHeightFieldShape/`11213421`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStorageSampledHeightFieldShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStorageSampledHeightFieldShape"`: Name of this class.
    #[serde(default = "HkpStorageSampledHeightFieldShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x15ff414b`: Unique value of this class.
    #[serde(default = "HkpStorageSampledHeightFieldShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStorageSampledHeightFieldShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStorageSampledHeightFieldShapeHkParam<'a>>
}

impl HkpStorageSampledHeightFieldShape<'_> {
    /// Return `"hkpStorageSampledHeightFieldShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpStorageSampledHeightFieldShape".into()
    }

    /// Return `"0x15ff414b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x15ff414b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageSampledHeightFieldShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"storage"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storage")]
    Storage(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"triangleFlip"`
    /// -   type: `hkBool`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleFlip")]
    TriangleFlip(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageSampledHeightFieldShapeHkParam<'de>, "@name",
    ("storage" => Storage(Vec<f64>)),
    ("triangleFlip" => TriangleFlip(bool)),
}
