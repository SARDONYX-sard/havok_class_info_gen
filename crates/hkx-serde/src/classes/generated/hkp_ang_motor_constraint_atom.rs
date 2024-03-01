//! A Rust structure that implements a serializer/deserializer corresponding to `hkpAngMotorConstraintAtom`, a class defined in C++
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
/// -    size: 20
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpAngMotorConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpAngMotorConstraintAtom"`: Name of this class.
    #[serde(default = "HkpAngMotorConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x81f087ff`: Unique value of this class.
    #[serde(default = "HkpAngMotorConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpAngMotorConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpAngMotorConstraintAtomHkParam<'a>>
}

impl HkpAngMotorConstraintAtom<'_> {
    /// Return `"hkpAngMotorConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpAngMotorConstraintAtom".into()
    }

    /// Return `"0x81f087ff"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x81f087ff".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngMotorConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"motorAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motorAxis")]
    MotorAxis(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializedOffset")]
    InitializedOffset(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousTargetAngleOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousTargetAngleOffset")]
    PreviousTargetAngleOffset(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"correspondingAngLimitSolverResultOffset"`
    /// -   type: `hkInt16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "correspondingAngLimitSolverResultOffset")]
    CorrespondingAngLimitSolverResultOffset(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetAngle")]
    TargetAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"motor"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor")]
    Motor(Box<HkpConstraintMotor>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngMotorConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(bool)),
    ("motorAxis" => MotorAxis(u8)),
    ("initializedOffset" => InitializedOffset(i16)),
    ("previousTargetAngleOffset" => PreviousTargetAngleOffset(i16)),
    ("correspondingAngLimitSolverResultOffset" => CorrespondingAngLimitSolverResultOffset(i16)),
    ("targetAngle" => TargetAngle(f64)),
    ("motor" => Motor(Box<HkpConstraintMotor>)),
}
