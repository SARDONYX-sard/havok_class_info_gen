//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBallSocketConstraintAtom`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBallSocketConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBallSocketConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpBallSocketConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe70e4dfa`: Unique value of this class.
    #[serde(default = "HkpBallSocketConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBallSocketConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBallSocketConstraintAtomHkParam<'a>>
}

impl HkpBallSocketConstraintAtom<'_> {
    /// Return `"hkpBallSocketConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpBallSocketConstraintAtom".into()
    }

    /// Return `"0xe70e4dfa"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe70e4dfa".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallSocketConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"solvingMethod"`
    /// -   type: `enum SolvingMethod`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solvingMethod")]
    SolvingMethod(SolvingMethod),
    /// # Field information in the original C++ class
    /// -   name:`"bodiesToNotify"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodiesToNotify")]
    BodiesToNotify(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"velocityStabilizationFactor"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityStabilizationFactor")]
    VelocityStabilizationFactor(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"maxImpulse"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"inertiaStabilizationFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaStabilizationFactor")]
    InertiaStabilizationFactor(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketConstraintAtomHkParam<'de>, "@name",
    ("solvingMethod" => SolvingMethod(SolvingMethod)),
    ("bodiesToNotify" => BodiesToNotify(Primitive<u8>)),
    ("velocityStabilizationFactor" => VelocityStabilizationFactor(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<f32>)),
    ("inertiaStabilizationFactor" => InertiaStabilizationFactor(Primitive<f32>)),
}
