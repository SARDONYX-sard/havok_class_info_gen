//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultVelocityDamper`, a class defined in C++
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
/// -  parent: hkpVehicleVelocityDamper/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultVelocityDamper<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultVelocityDamper"`: The original C++ class name.
    #[serde(default = "HkpVehicleDefaultVelocityDamper::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x741b8d9e`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultVelocityDamper::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultVelocityDamperHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultVelocityDamperHkParam<'a>>
}

impl HkpVehicleDefaultVelocityDamper<'_> {
    /// Return `"hkpVehicleDefaultVelocityDamper"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleDefaultVelocityDamper".into()
    }

    /// Return `"0x741b8d9e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x741b8d9e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultVelocityDamperHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"normalSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalSpinDamping")]
    NormalSpinDamping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionSpinDamping")]
    CollisionSpinDamping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionThreshold"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionThreshold")]
    CollisionThreshold(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultVelocityDamperHkParam<'de>, "@name",
    ("normalSpinDamping" => NormalSpinDamping(Primitive<f32>)),
    ("collisionSpinDamping" => CollisionSpinDamping(Primitive<f32>)),
    ("collisionThreshold" => CollisionThreshold(Primitive<f32>)),
}
