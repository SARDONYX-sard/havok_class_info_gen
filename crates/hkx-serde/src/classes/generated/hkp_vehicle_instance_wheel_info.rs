//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleInstanceWheelInfo`, a class defined in C++
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
/// -    size: 224
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleInstanceWheelInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleInstanceWheelInfo"`: Name of this class.
    #[serde(default = "HkpVehicleInstanceWheelInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x99f693f0`: Unique value of this class.
    #[serde(default = "HkpVehicleInstanceWheelInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleInstanceWheelInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleInstanceWheelInfoHkParam<'a>>
}

impl HkpVehicleInstanceWheelInfo<'_> {
    /// Return `"hkpVehicleInstanceWheelInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleInstanceWheelInfo".into()
    }

    /// Return `"0x99f693f0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x99f693f0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleInstanceWheelInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"contactPoint"`
    /// -   type: `struct hkContactPoint`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPoint")]
    ContactPoint(HkContactPoint),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactFriction"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactFriction")]
    ContactFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactBody"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactBody", skip_serializing)]
    ContactBody(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactShapeKey"`
    /// -   type: `hkUint32[8]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactShapeKey")]
    ContactShapeKey([u32; 8]),
    /// # Information on fields in the original C++ class
    /// -   name:`"hardPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardPointWs")]
    HardPointWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rayEndPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rayEndPointWs")]
    RayEndPointWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentSuspensionLength"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentSuspensionLength")]
    CurrentSuspensionLength(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"suspensionDirectionWs"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspensionDirectionWs")]
    SuspensionDirectionWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"spinAxisChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAxisChassisSpace")]
    SpinAxisChassisSpace(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"spinAxisWs"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAxisWs")]
    SpinAxisWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"steeringOrientationChassisSpace"`
    /// -   type: `hkQuaternion`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steeringOrientationChassisSpace")]
    SteeringOrientationChassisSpace(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"spinVelocity"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinVelocity")]
    SpinVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"spinAngle"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAngle")]
    SpinAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"skidEnergyDensity"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skidEnergyDensity")]
    SkidEnergyDensity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sideForce"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideForce")]
    SideForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"forwardSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardSlipVelocity")]
    ForwardSlipVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sideSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideSlipVelocity")]
    SideSlipVelocity(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstanceWheelInfoHkParam<'de>, "@name",
    ("contactPoint" => ContactPoint(HkContactPoint)),
    ("contactFriction" => ContactFriction(f64)),
    ("contactBody" => ContactBody(())),
    ("contactShapeKey" => ContactShapeKey([u32; 8])),
    ("hardPointWs" => HardPointWs(cgmath::Vector4<f32>)),
    ("rayEndPointWs" => RayEndPointWs(cgmath::Vector4<f32>)),
    ("currentSuspensionLength" => CurrentSuspensionLength(f64)),
    ("suspensionDirectionWs" => SuspensionDirectionWs(cgmath::Vector4<f32>)),
    ("spinAxisChassisSpace" => SpinAxisChassisSpace(cgmath::Vector4<f32>)),
    ("spinAxisWs" => SpinAxisWs(cgmath::Vector4<f32>)),
    ("steeringOrientationChassisSpace" => SteeringOrientationChassisSpace(cgmath::Quaternion<f32>)),
    ("spinVelocity" => SpinVelocity(f64)),
    ("spinAngle" => SpinAngle(f64)),
    ("skidEnergyDensity" => SkidEnergyDensity(f64)),
    ("sideForce" => SideForce(f64)),
    ("forwardSlipVelocity" => ForwardSlipVelocity(f64)),
    ("sideSlipVelocity" => SideSlipVelocity(f64)),
}
