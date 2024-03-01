//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleLinearCastWheelCollideWheelState`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleLinearCastWheelCollideWheelState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleLinearCastWheelCollideWheelState"`: Name of this class.
    #[serde(default = "HkpVehicleLinearCastWheelCollideWheelState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2a9acf98`: Unique value of this class.
    #[serde(default = "HkpVehicleLinearCastWheelCollideWheelState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastWheelCollideWheelStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleLinearCastWheelCollideWheelStateHkParam<'a>>
}

impl HkpVehicleLinearCastWheelCollideWheelState<'_> {
    /// Return `"hkpVehicleLinearCastWheelCollideWheelState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleLinearCastWheelCollideWheelState".into()
    }

    /// Return `"0x2a9acf98"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2a9acf98".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastWheelCollideWheelStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Box<HkpAabbPhantom>),
    /// # Information on fields in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Box<HkpShape>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"to"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "to")]
    To(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideWheelStateHkParam<'de>, "@name",
    ("phantom" => Phantom(Box<HkpAabbPhantom>)),
    ("shape" => Shape(Box<HkpShape>)),
    ("transform" => Transform(cgmath::Matrix4<f32>)),
    ("to" => To(cgmath::Vector4<f32>)),
}
