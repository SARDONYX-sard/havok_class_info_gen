//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTwistLimitConstraintAtom`, a class defined in C++
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
pub struct HkpTwistLimitConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTwistLimitConstraintAtom"`: Name of this class.
    #[serde(default = "HkpTwistLimitConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7c9b1052`: Unique value of this class.
    #[serde(default = "HkpTwistLimitConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTwistLimitConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTwistLimitConstraintAtomHkParam<'a>>
}

impl HkpTwistLimitConstraintAtom<'_> {
    /// Return `"hkpTwistLimitConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTwistLimitConstraintAtom".into()
    }

    /// Return `"0x7c9b1052"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7c9b1052".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTwistLimitConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"twistAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAxis")]
    TwistAxis(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"refAxis"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refAxis")]
    RefAxis(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAngle")]
    MinAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngle")]
    MaxAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTwistLimitConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(u8)),
    ("twistAxis" => TwistAxis(u8)),
    ("refAxis" => RefAxis(u8)),
    ("minAngle" => MinAngle(f64)),
    ("maxAngle" => MaxAngle(f64)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(f64)),
}
