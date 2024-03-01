//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSampledHeightFieldShape`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkpHeightFieldShape/`e7eca7eb`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSampledHeightFieldShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSampledHeightFieldShape"`: Name of this class.
    #[serde(default = "HkpSampledHeightFieldShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x11213421`: Unique value of this class.
    #[serde(default = "HkpSampledHeightFieldShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSampledHeightFieldShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSampledHeightFieldShapeHkParam<'a>>
}

impl HkpSampledHeightFieldShape<'_> {
    /// Return `"hkpSampledHeightFieldShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpSampledHeightFieldShape".into()
    }

    /// Return `"0x11213421"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x11213421".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSampledHeightFieldShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"xRes"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xRes")]
    XRes(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"zRes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "zRes")]
    ZRes(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"heightCenter"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightCenter")]
    HeightCenter(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"useProjectionBasedHeight"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useProjectionBasedHeight")]
    UseProjectionBasedHeight(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"heightfieldType"`
    /// -   type: `enum HeightFieldType`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightfieldType")]
    HeightfieldType(HeightFieldType),
    /// # Information on fields in the original C++ class
    /// -   name:`"intToFloatScale"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "intToFloatScale")]
    IntToFloatScale(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatToIntScale"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntScale")]
    FloatToIntScale(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatToIntOffsetFloorCorrected"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntOffsetFloorCorrected")]
    FloatToIntOffsetFloorCorrected(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"extents"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extents")]
    Extents(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSampledHeightFieldShapeHkParam<'de>, "@name",
    ("xRes" => XRes(i32)),
    ("zRes" => ZRes(i32)),
    ("heightCenter" => HeightCenter(f64)),
    ("useProjectionBasedHeight" => UseProjectionBasedHeight(bool)),
    ("heightfieldType" => HeightfieldType(HeightFieldType)),
    ("intToFloatScale" => IntToFloatScale(cgmath::Vector4<f32>)),
    ("floatToIntScale" => FloatToIntScale(cgmath::Vector4<f32>)),
    ("floatToIntOffsetFloorCorrected" => FloatToIntOffsetFloorCorrected(cgmath::Vector4<f32>)),
    ("extents" => Extents(cgmath::Vector4<f32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HeightFieldType {
    #[serde(rename = "HEIGHTFIELD_STORAGE")]
    HeightfieldStorage = 0,
    #[serde(rename = "HEIGHTFIELD_COMPRESSED")]
    HeightfieldCompressed = 1,
    #[serde(rename = "HEIGHTFIELD_USER")]
    HeightfieldUser = 2,
    #[serde(rename = "HEIGHTFIELD_MAX_ID")]
    HeightfieldMaxId = 3,
}
