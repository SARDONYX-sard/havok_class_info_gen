//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRagdollMotorConstraintAtom`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRagdollMotorConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRagdollMotorConstraintAtom"`: Name of this class.
    #[serde(default = "HkpRagdollMotorConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x71013826`: Unique value of this class.
    #[serde(default = "HkpRagdollMotorConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRagdollMotorConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRagdollMotorConstraintAtomHkParam<'a>>
}

impl HkpRagdollMotorConstraintAtom<'_> {
    /// Return `"hkpRagdollMotorConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpRagdollMotorConstraintAtom".into()
    }

    /// Return `"0x71013826"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x71013826".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollMotorConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializedOffset")]
    InitializedOffset(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousTargetAnglesOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousTargetAnglesOffset")]
    PreviousTargetAnglesOffset(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"target_bRca"`
    /// -   type: `hkMatrix3`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "target_bRca")]
    TargetBRca(cgmath::Matrix3<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(Box<HkpConstraintMotor>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollMotorConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(bool)),
    ("initializedOffset" => InitializedOffset(i16)),
    ("previousTargetAnglesOffset" => PreviousTargetAnglesOffset(i16)),
    ("target_bRca" => TargetBRca(cgmath::Matrix3<f32>)),
    ("motors" => Motors(Box<HkpConstraintMotor>)),
}
