//! A Rust structure that implements a serializer/deserializer corresponding to `hkbPoweredRagdollControlData`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbPoweredRagdollControlData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbPoweredRagdollControlData"`: Name of this class.
    #[serde(default = "HkbPoweredRagdollControlData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf5ba21b`: Unique value of this class.
    #[serde(default = "HkbPoweredRagdollControlData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbPoweredRagdollControlDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbPoweredRagdollControlDataHkParam<'a>>
}

impl HkbPoweredRagdollControlData<'_> {
    /// Return `"hkbPoweredRagdollControlData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbPoweredRagdollControlData".into()
    }

    /// Return `"0xf5ba21b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf5ba21b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoweredRagdollControlDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "maxForce")]
    MaxForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"proportionalRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "proportionalRecoveryVelocity")]
    ProportionalRecoveryVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"constantRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constantRecoveryVelocity")]
    ConstantRecoveryVelocity(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoweredRagdollControlDataHkParam<'de>, "@name",
    ("maxForce" => MaxForce(f64)),
    ("tau" => Tau(f64)),
    ("damping" => Damping(f64)),
    ("proportionalRecoveryVelocity" => ProportionalRecoveryVelocity(f64)),
    ("constantRecoveryVelocity" => ConstantRecoveryVelocity(f64)),
}
