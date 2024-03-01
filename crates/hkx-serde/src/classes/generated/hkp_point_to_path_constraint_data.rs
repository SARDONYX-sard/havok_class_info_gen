//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPointToPathConstraintData`, a class defined in C++
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
/// -    size: 176
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPointToPathConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPointToPathConstraintData"`: Name of this class.
    #[serde(default = "HkpPointToPathConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8e7cb5da`: Unique value of this class.
    #[serde(default = "HkpPointToPathConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPointToPathConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPointToPathConstraintDataHkParam<'a>>
}

impl HkpPointToPathConstraintData<'_> {
    /// Return `"hkpPointToPathConstraintData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPointToPathConstraintData".into()
    }

    /// Return `"0x8e7cb5da"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8e7cb5da".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPointToPathConstraintDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # Information on fields in the original C++ class
    /// -   name:`"path"`
    /// -   type: `struct hkpParametricCurve*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "path")]
    Path(Box<HkpParametricCurve>),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxFrictionForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionForce")]
    MaxFrictionForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"angularConstrainedDOF"`
    /// -   type: `enum OrientationConstraintType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularConstrainedDOF")]
    AngularConstrainedDof(OrientationConstraintType),
    /// # Information on fields in the original C++ class
    /// -   name:`"transform_OS_KS"`
    /// -   type: `hkTransform[2]`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform_OS_KS")]
    TransformOsKs([cgmath::Matrix4<f32>; 2]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPointToPathConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("path" => Path(Box<HkpParametricCurve>)),
    ("maxFrictionForce" => MaxFrictionForce(f64)),
    ("angularConstrainedDOF" => AngularConstrainedDof(OrientationConstraintType)),
    ("transform_OS_KS" => TransformOsKs([cgmath::Matrix4<f32>; 2])),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OrientationConstraintType {
    #[serde(rename = "CONSTRAIN_ORIENTATION_INVALID")]
    ConstrainOrientationInvalid = 0,
    #[serde(rename = "CONSTRAIN_ORIENTATION_NONE")]
    ConstrainOrientationNone = 1,
    #[serde(rename = "CONSTRAIN_ORIENTATION_ALLOW_SPIN")]
    ConstrainOrientationAllowSpin = 2,
    #[serde(rename = "CONSTRAIN_ORIENTATION_TO_PATH")]
    ConstrainOrientationToPath = 3,
    #[serde(rename = "CONSTRAIN_ORIENTATION_MAX_ID")]
    ConstrainOrientationMaxId = 4,
}
