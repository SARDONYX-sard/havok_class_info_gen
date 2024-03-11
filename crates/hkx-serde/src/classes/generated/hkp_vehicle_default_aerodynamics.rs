//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultAerodynamics`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkpVehicleAerodynamics/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultAerodynamics<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultAerodynamics"`: The original C++ class name.
    #[serde(default = "HkpVehicleDefaultAerodynamics::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x42fc5bbd`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultAerodynamics::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultAerodynamicsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultAerodynamicsHkParam<'a>>
}

impl HkpVehicleDefaultAerodynamics<'_> {
    /// Return `"hkpVehicleDefaultAerodynamics"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleDefaultAerodynamics".into()
    }

    /// Return `"0x42fc5bbd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x42fc5bbd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultAerodynamicsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"airDensity"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "airDensity")]
    AirDensity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"frontalArea"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frontalArea")]
    FrontalArea(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"dragCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dragCoefficient")]
    DragCoefficient(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"liftCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "liftCoefficient")]
    LiftCoefficient(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"extraGravityws"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraGravityws")]
    ExtraGravityws(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAerodynamicsHkParam<'de>, "@name",
    ("airDensity" => AirDensity(Primitive<f32>)),
    ("frontalArea" => FrontalArea(Primitive<f32>)),
    ("dragCoefficient" => DragCoefficient(Primitive<f32>)),
    ("liftCoefficient" => LiftCoefficient(Primitive<f32>)),
    ("extraGravityws" => ExtraGravityws(Vector4<f32>)),
}
