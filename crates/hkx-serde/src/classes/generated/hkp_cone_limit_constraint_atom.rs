//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConeLimitConstraintAtom`, a class defined in C++
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
pub struct HkpConeLimitConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConeLimitConstraintAtom"`: Name of this class.
    #[serde(default = "HkpConeLimitConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf19443c8`: Unique value of this class.
    #[serde(default = "HkpConeLimitConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConeLimitConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConeLimitConstraintAtomHkParam<'a>>
}

impl HkpConeLimitConstraintAtom<'_> {
    /// Return `"hkpConeLimitConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConeLimitConstraintAtom".into()
    }

    /// Return `"0xf19443c8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf19443c8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConeLimitConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"twistAxisInA"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAxisInA")]
    TwistAxisInA(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"refAxisInB"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refAxisInB")]
    RefAxisInB(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"angleMeasurementMode"`
    /// -   type: `enum MeasurementMode`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angleMeasurementMode")]
    AngleMeasurementMode(MeasurementMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"memOffsetToAngleOffset"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToAngleOffset")]
    MemOffsetToAngleOffset(u8),
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
    HkpConeLimitConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(u8)),
    ("twistAxisInA" => TwistAxisInA(u8)),
    ("refAxisInB" => RefAxisInB(u8)),
    ("angleMeasurementMode" => AngleMeasurementMode(MeasurementMode)),
    ("memOffsetToAngleOffset" => MemOffsetToAngleOffset(u8)),
    ("minAngle" => MinAngle(f64)),
    ("maxAngle" => MaxAngle(f64)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(f64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MeasurementMode {
    #[serde(rename = "ZERO_WHEN_VECTORS_ALIGNED")]
    ZeroWhenVectorsAligned = 0,
    #[serde(rename = "ZERO_WHEN_VECTORS_PERPENDICULAR")]
    ZeroWhenVectorsPerpendicular = 1,
}
