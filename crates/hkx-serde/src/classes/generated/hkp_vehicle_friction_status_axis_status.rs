//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleFrictionStatusAxisStatus`, a class defined in C++
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
/// -    size: 36
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleFrictionStatusAxisStatus<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleFrictionStatusAxisStatus"`: Name of this class.
    #[serde(default = "HkpVehicleFrictionStatusAxisStatus::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe70e2bb4`: Unique value of this class.
    #[serde(default = "HkpVehicleFrictionStatusAxisStatus::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleFrictionStatusAxisStatusHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleFrictionStatusAxisStatusHkParam<'a>>
}

impl HkpVehicleFrictionStatusAxisStatus<'_> {
    /// Return `"hkpVehicleFrictionStatusAxisStatus"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleFrictionStatusAxisStatus".into()
    }

    /// Return `"0xe70e2bb4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe70e2bb4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionStatusAxisStatusHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"forward_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forward_slip_velocity")]
    ForwardSlipVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"side_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "side_slip_velocity")]
    SideSlipVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"skid_energy_density"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skid_energy_density")]
    SkidEnergyDensity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"side_force"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "side_force")]
    SideForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"delayed_forward_impulse"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayed_forward_impulse")]
    DelayedForwardImpulse(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sideRhs"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideRhs")]
    SideRhs(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"forwardRhs"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardRhs")]
    ForwardRhs(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"relativeSideForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeSideForce")]
    RelativeSideForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"relativeForwardForce"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeForwardForce")]
    RelativeForwardForce(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatusAxisStatusHkParam<'de>, "@name",
    ("forward_slip_velocity" => ForwardSlipVelocity(f64)),
    ("side_slip_velocity" => SideSlipVelocity(f64)),
    ("skid_energy_density" => SkidEnergyDensity(f64)),
    ("side_force" => SideForce(f64)),
    ("delayed_forward_impulse" => DelayedForwardImpulse(f64)),
    ("sideRhs" => SideRhs(f64)),
    ("forwardRhs" => ForwardRhs(f64)),
    ("relativeSideForce" => RelativeSideForce(f64)),
    ("relativeForwardForce" => RelativeForwardForce(f64)),
}
