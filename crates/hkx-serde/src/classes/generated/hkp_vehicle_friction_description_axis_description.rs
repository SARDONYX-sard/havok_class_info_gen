//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleFrictionDescriptionAxisDescription`, a class defined in C++
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
/// -    size: 100
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleFrictionDescriptionAxisDescription<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleFrictionDescriptionAxisDescription"`: The original C++ class name.
    #[serde(default = "HkpVehicleFrictionDescriptionAxisDescription::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x59ce153f`: Unique value of this class.
    #[serde(default = "HkpVehicleFrictionDescriptionAxisDescription::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleFrictionDescriptionAxisDescriptionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleFrictionDescriptionAxisDescriptionHkParam<'a>>
}

impl HkpVehicleFrictionDescriptionAxisDescription<'_> {
    /// Return `"hkpVehicleFrictionDescriptionAxisDescription"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleFrictionDescriptionAxisDescription".into()
    }

    /// Return `"0x59ce153f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x59ce153f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionDescriptionAxisDescriptionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"frictionCircleYtab"`
    /// -   type: `hkReal[16]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionCircleYtab")]
    FrictionCircleYtab([Primitive<f32>; 16]),
    /// # Field information in the original C++ class
    /// -   name:`"xStep"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xStep")]
    XStep(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"xStart"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xStart")]
    XStart(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelSurfaceInertia"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSurfaceInertia")]
    WheelSurfaceInertia(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelSurfaceInertiaInv"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSurfaceInertiaInv")]
    WheelSurfaceInertiaInv(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelChassisMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelChassisMassRatio")]
    WheelChassisMassRatio(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelRadius"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelRadius")]
    WheelRadius(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelRadiusInv"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelRadiusInv")]
    WheelRadiusInv(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelDownForceFactor"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDownForceFactor")]
    WheelDownForceFactor(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelDownForceSumFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDownForceSumFactor")]
    WheelDownForceSumFactor(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescriptionAxisDescriptionHkParam<'de>, "@name",
    ("frictionCircleYtab" => FrictionCircleYtab([Primitive<f32>; 16])),
    ("xStep" => XStep(Primitive<f32>)),
    ("xStart" => XStart(Primitive<f32>)),
    ("wheelSurfaceInertia" => WheelSurfaceInertia(Primitive<f32>)),
    ("wheelSurfaceInertiaInv" => WheelSurfaceInertiaInv(Primitive<f32>)),
    ("wheelChassisMassRatio" => WheelChassisMassRatio(Primitive<f32>)),
    ("wheelRadius" => WheelRadius(Primitive<f32>)),
    ("wheelRadiusInv" => WheelRadiusInv(Primitive<f32>)),
    ("wheelDownForceFactor" => WheelDownForceFactor(Primitive<f32>)),
    ("wheelDownForceSumFactor" => WheelDownForceSumFactor(Primitive<f32>)),
}
