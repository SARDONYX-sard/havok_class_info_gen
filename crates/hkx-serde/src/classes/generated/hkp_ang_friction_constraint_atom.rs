//! A Rust structure that implements a serializer/deserializer corresponding to `hkpAngFrictionConstraintAtom`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpAngFrictionConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpAngFrictionConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpAngFrictionConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf313aa80`: Unique value of this class.
    #[serde(default = "HkpAngFrictionConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpAngFrictionConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpAngFrictionConstraintAtomHkParam<'a>>
}

impl HkpAngFrictionConstraintAtom<'_> {
    /// Return `"hkpAngFrictionConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpAngFrictionConstraintAtom".into()
    }

    /// Return `"0xf313aa80"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf313aa80".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngFrictionConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"firstFrictionAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "firstFrictionAxis")]
    FirstFrictionAxis(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"numFrictionAxes"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numFrictionAxes")]
    NumFrictionAxes(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"maxFrictionTorque"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionTorque")]
    MaxFrictionTorque(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngFrictionConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("firstFrictionAxis" => FirstFrictionAxis(Primitive<u8>)),
    ("numFrictionAxes" => NumFrictionAxes(Primitive<u8>)),
    ("maxFrictionTorque" => MaxFrictionTorque(Primitive<f32>)),
}
