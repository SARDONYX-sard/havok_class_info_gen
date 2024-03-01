//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBreakableConstraintData`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBreakableConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBreakableConstraintData"`: Name of this class.
    #[serde(default = "HkpBreakableConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7d6310c8`: Unique value of this class.
    #[serde(default = "HkpBreakableConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBreakableConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBreakableConstraintDataHkParam<'a>>
}

impl HkpBreakableConstraintData<'_> {
    /// Return `"hkpBreakableConstraintData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpBreakableConstraintData".into()
    }

    /// Return `"0x7d6310c8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7d6310c8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBreakableConstraintDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintData")]
    ConstraintData(Box<HkpConstraintData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"childRuntimeSize"`
    /// -   type: `hkUint16`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childRuntimeSize")]
    ChildRuntimeSize(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"childNumSolverResults"`
    /// -   type: `hkUint16`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childNumSolverResults")]
    ChildNumSolverResults(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverResultLimit"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverResultLimit")]
    SolverResultLimit(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"removeWhenBroken"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "removeWhenBroken")]
    RemoveWhenBroken(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"revertBackVelocityOnBreak"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "revertBackVelocityOnBreak")]
    RevertBackVelocityOnBreak(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBreakableConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("constraintData" => ConstraintData(Box<HkpConstraintData>)),
    ("childRuntimeSize" => ChildRuntimeSize(u16)),
    ("childNumSolverResults" => ChildNumSolverResults(u16)),
    ("solverResultLimit" => SolverResultLimit(f64)),
    ("removeWhenBroken" => RemoveWhenBroken(bool)),
    ("revertBackVelocityOnBreak" => RevertBackVelocityOnBreak(bool)),
}