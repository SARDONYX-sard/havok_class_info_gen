//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleData`, a class defined in C++
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
/// -    size: 416
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleData"`: Name of this class.
    #[serde(default = "HkpVehicleData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x173feb43`: Unique value of this class.
    #[serde(default = "HkpVehicleData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDataHkParam<'a>>
}

impl HkpVehicleData<'_> {
    /// Return `"hkpVehicleData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleData".into()
    }

    /// Return `"0x173feb43"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x173feb43".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numWheels"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numWheels")]
    NumWheels(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisOrientation"`
    /// -   type: `hkRotation`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisOrientation")]
    ChassisOrientation(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"torqueRollFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueRollFactor")]
    TorqueRollFactor(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"torquePitchFactor"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torquePitchFactor")]
    TorquePitchFactor(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"torqueYawFactor"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueYawFactor")]
    TorqueYawFactor(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"extraTorqueFactor"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraTorqueFactor")]
    ExtraTorqueFactor(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxVelocityForPositionalFriction"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVelocityForPositionalFriction")]
    MaxVelocityForPositionalFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisUnitInertiaYaw"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaYaw")]
    ChassisUnitInertiaYaw(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisUnitInertiaRoll"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaRoll")]
    ChassisUnitInertiaRoll(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisUnitInertiaPitch"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaPitch")]
    ChassisUnitInertiaPitch(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"frictionEqualizer"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionEqualizer")]
    FrictionEqualizer(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"normalClippingAngleCos"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalClippingAngleCos")]
    NormalClippingAngleCos(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxFrictionSolverMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionSolverMassRatio")]
    MaxFrictionSolverMassRatio(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelParams"`
    /// -   type: `hkArray&lt;struct hkpVehicleDataWheelComponentParams&gt;`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelParams")]
    WheelParams(Vec<HkpVehicleDataWheelComponentParams>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numWheelsPerAxle"`
    /// -   type: `hkArray&lt;hkInt8&gt;`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numWheelsPerAxle")]
    NumWheelsPerAxle(Vec<i8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"frictionDescription"`
    /// -   type: `struct hkpVehicleFrictionDescription`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionDescription")]
    FrictionDescription(HkpVehicleFrictionDescription),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisFrictionInertiaInvDiag"`
    /// -   type: `hkVector4`
    /// - offset: 384
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisFrictionInertiaInvDiag")]
    ChassisFrictionInertiaInvDiag(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"alreadyInitialised"`
    /// -   type: `hkBool`
    /// - offset: 400
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alreadyInitialised")]
    AlreadyInitialised(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDataHkParam<'de>, "@name",
    ("gravity" => Gravity(cgmath::Vector4<f32>)),
    ("numWheels" => NumWheels(i8)),
    ("chassisOrientation" => ChassisOrientation(cgmath::Matrix4<f32>)),
    ("torqueRollFactor" => TorqueRollFactor(f64)),
    ("torquePitchFactor" => TorquePitchFactor(f64)),
    ("torqueYawFactor" => TorqueYawFactor(f64)),
    ("extraTorqueFactor" => ExtraTorqueFactor(f64)),
    ("maxVelocityForPositionalFriction" => MaxVelocityForPositionalFriction(f64)),
    ("chassisUnitInertiaYaw" => ChassisUnitInertiaYaw(f64)),
    ("chassisUnitInertiaRoll" => ChassisUnitInertiaRoll(f64)),
    ("chassisUnitInertiaPitch" => ChassisUnitInertiaPitch(f64)),
    ("frictionEqualizer" => FrictionEqualizer(f64)),
    ("normalClippingAngleCos" => NormalClippingAngleCos(f64)),
    ("maxFrictionSolverMassRatio" => MaxFrictionSolverMassRatio(f64)),
    ("wheelParams" => WheelParams(Vec<HkpVehicleDataWheelComponentParams>)),
    ("numWheelsPerAxle" => NumWheelsPerAxle(Vec<i8>)),
    ("frictionDescription" => FrictionDescription(HkpVehicleFrictionDescription)),
    ("chassisFrictionInertiaInvDiag" => ChassisFrictionInertiaInvDiag(cgmath::Vector4<f32>)),
    ("alreadyInitialised" => AlreadyInitialised(bool)),
}
