//! A Rust structure that implements a serializer/deserializer corresponding to `hkpLinearParametricCurve`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkpParametricCurve/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpLinearParametricCurve<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpLinearParametricCurve"`: The original C++ class name.
    #[serde(default = "HkpLinearParametricCurve::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd7b3be03`: Unique value of this class.
    #[serde(default = "HkpLinearParametricCurve::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpLinearParametricCurveHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpLinearParametricCurveHkParam<'a>>
}

impl HkpLinearParametricCurve<'_> {
    /// Return `"hkpLinearParametricCurve"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpLinearParametricCurve".into()
    }

    /// Return `"0xd7b3be03"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd7b3be03".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinearParametricCurveHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"smoothingFactor"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "smoothingFactor")]
    SmoothingFactor(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"closedLoop"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closedLoop")]
    ClosedLoop(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"dirNotParallelToTangentAlongWholePath"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dirNotParallelToTangentAlongWholePath")]
    DirNotParallelToTangentAlongWholePath(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"points"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "points")]
    Points(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"distance"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distance")]
    Distance(Vec<Primitive<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinearParametricCurveHkParam<'de>, "@name",
    ("smoothingFactor" => SmoothingFactor(Primitive<f32>)),
    ("closedLoop" => ClosedLoop(Primitive<bool>)),
    ("dirNotParallelToTangentAlongWholePath" => DirNotParallelToTangentAlongWholePath(Vector4<f32>)),
    ("points" => Points(Vec<Vector4<f32>>)),
    ("distance" => Distance(Vec<Primitive<f32>>)),
}
