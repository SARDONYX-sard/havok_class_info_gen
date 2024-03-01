//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleInstance`, a class defined in C++
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
/// -    size: 212
/// -  vtable: true
/// -  parent: hkpUnaryAction/`895532c0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleInstance<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleInstance"`: Name of this class.
    #[serde(default = "HkpVehicleInstance::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x877bb579`: Unique value of this class.
    #[serde(default = "HkpVehicleInstance::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleInstanceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleInstanceHkParam<'a>>
}

impl HkpVehicleInstance<'_> {
    /// Return `"hkpVehicleInstance"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleInstance".into()
    }

    /// Return `"0x877bb579"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x877bb579".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleInstanceHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `struct hkpVehicleData*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Box<HkpVehicleData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"driverInput"`
    /// -   type: `struct hkpVehicleDriverInput*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "driverInput")]
    DriverInput(Box<HkpVehicleDriverInput>),
    /// # Information on fields in the original C++ class
    /// -   name:`"steering"`
    /// -   type: `struct hkpVehicleSteering*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steering")]
    Steering(Box<HkpVehicleSteering>),
    /// # Information on fields in the original C++ class
    /// -   name:`"engine"`
    /// -   type: `struct hkpVehicleEngine*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "engine")]
    Engine(Box<HkpVehicleEngine>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transmission"`
    /// -   type: `struct hkpVehicleTransmission*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transmission")]
    Transmission(Box<HkpVehicleTransmission>),
    /// # Information on fields in the original C++ class
    /// -   name:`"brake"`
    /// -   type: `struct hkpVehicleBrake*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "brake")]
    Brake(Box<HkpVehicleBrake>),
    /// # Information on fields in the original C++ class
    /// -   name:`"suspension"`
    /// -   type: `struct hkpVehicleSuspension*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspension")]
    Suspension(Box<HkpVehicleSuspension>),
    /// # Information on fields in the original C++ class
    /// -   name:`"aerodynamics"`
    /// -   type: `struct hkpVehicleAerodynamics*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aerodynamics")]
    Aerodynamics(Box<HkpVehicleAerodynamics>),
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelCollide"`
    /// -   type: `struct hkpVehicleWheelCollide*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollide")]
    WheelCollide(Box<HkpVehicleWheelCollide>),
    /// # Information on fields in the original C++ class
    /// -   name:`"tyreMarks"`
    /// -   type: `struct hkpTyremarksInfo*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tyreMarks")]
    TyreMarks(Box<HkpTyremarksInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"velocityDamper"`
    /// -   type: `struct hkpVehicleVelocityDamper*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityDamper")]
    VelocityDamper(Box<HkpVehicleVelocityDamper>),
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelsInfo"`
    /// -   type: `hkArray&lt;struct hkpVehicleInstanceWheelInfo&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsInfo")]
    WheelsInfo(Vec<HkpVehicleInstanceWheelInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"frictionStatus"`
    /// -   type: `struct hkpVehicleFrictionStatus`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionStatus")]
    FrictionStatus(HkpVehicleFrictionStatus),
    /// # Information on fields in the original C++ class
    /// -   name:`"deviceStatus"`
    /// -   type: `struct hkpVehicleDriverInputStatus*`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deviceStatus")]
    DeviceStatus(Box<HkpVehicleDriverInputStatus>),
    /// # Information on fields in the original C++ class
    /// -   name:`"isFixed"`
    /// -   type: `hkArray&lt;hkBool&gt;`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isFixed")]
    IsFixed(Vec<bool>),
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelsTimeSinceMaxPedalInput"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsTimeSinceMaxPedalInput")]
    WheelsTimeSinceMaxPedalInput(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"tryingToReverse"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tryingToReverse")]
    TryingToReverse(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"torque"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torque")]
    Torque(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"rpm"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rpm")]
    Rpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"mainSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mainSteeringAngle")]
    MainSteeringAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelsSteeringAngle"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsSteeringAngle")]
    WheelsSteeringAngle(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"isReversing"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isReversing")]
    IsReversing(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentGear"`
    /// -   type: `hkInt8`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentGear")]
    CurrentGear(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"delayed"`
    /// -   type: `hkBool`
    /// - offset: 206
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayed")]
    Delayed(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"clutchDelayCountdown"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchDelayCountdown")]
    ClutchDelayCountdown(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstanceHkParam<'de>, "@name",
    ("data" => Data(Box<HkpVehicleData>)),
    ("driverInput" => DriverInput(Box<HkpVehicleDriverInput>)),
    ("steering" => Steering(Box<HkpVehicleSteering>)),
    ("engine" => Engine(Box<HkpVehicleEngine>)),
    ("transmission" => Transmission(Box<HkpVehicleTransmission>)),
    ("brake" => Brake(Box<HkpVehicleBrake>)),
    ("suspension" => Suspension(Box<HkpVehicleSuspension>)),
    ("aerodynamics" => Aerodynamics(Box<HkpVehicleAerodynamics>)),
    ("wheelCollide" => WheelCollide(Box<HkpVehicleWheelCollide>)),
    ("tyreMarks" => TyreMarks(Box<HkpTyremarksInfo>)),
    ("velocityDamper" => VelocityDamper(Box<HkpVehicleVelocityDamper>)),
    ("wheelsInfo" => WheelsInfo(Vec<HkpVehicleInstanceWheelInfo>)),
    ("frictionStatus" => FrictionStatus(HkpVehicleFrictionStatus)),
    ("deviceStatus" => DeviceStatus(Box<HkpVehicleDriverInputStatus>)),
    ("isFixed" => IsFixed(Vec<bool>)),
    ("wheelsTimeSinceMaxPedalInput" => WheelsTimeSinceMaxPedalInput(f64)),
    ("tryingToReverse" => TryingToReverse(bool)),
    ("torque" => Torque(f64)),
    ("rpm" => Rpm(f64)),
    ("mainSteeringAngle" => MainSteeringAngle(f64)),
    ("wheelsSteeringAngle" => WheelsSteeringAngle(Vec<f64>)),
    ("isReversing" => IsReversing(bool)),
    ("currentGear" => CurrentGear(i8)),
    ("delayed" => Delayed(bool)),
    ("clutchDelayCountdown" => ClutchDelayCountdown(f64)),
}
