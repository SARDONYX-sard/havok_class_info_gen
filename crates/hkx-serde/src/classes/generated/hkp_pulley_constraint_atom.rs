//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPulleyConstraintAtom`, a class defined in C++
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
/// -    size: 64
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPulleyConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPulleyConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpPulleyConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x94a08848`: Unique value of this class.
    #[serde(default = "HkpPulleyConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPulleyConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPulleyConstraintAtomHkParam<'a>>
}

impl HkpPulleyConstraintAtom<'_> {
    /// Return `"hkpPulleyConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpPulleyConstraintAtom".into()
    }

    /// Return `"0x94a08848"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x94a08848".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"fixedPivotAinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedPivotAinWorld")]
    FixedPivotAinWorld(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fixedPivotBinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedPivotBinWorld")]
    FixedPivotBinWorld(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ropeLength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ropeLength")]
    RopeLength(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"leverageOnBodyB"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leverageOnBodyB")]
    LeverageOnBodyB(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintAtomHkParam<'de>, "@name",
    ("fixedPivotAinWorld" => FixedPivotAinWorld(Vector4<f32>)),
    ("fixedPivotBinWorld" => FixedPivotBinWorld(Vector4<f32>)),
    ("ropeLength" => RopeLength(Primitive<f32>)),
    ("leverageOnBodyB" => LeverageOnBodyB(Primitive<f32>)),
}
