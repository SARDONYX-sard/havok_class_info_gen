//! A Rust structure that implements a serializer/deserializer corresponding to `hkaKeyFrameHierarchyUtilityControlData`, a class defined in C++
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
/// -    size: 48
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaKeyFrameHierarchyUtilityControlData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaKeyFrameHierarchyUtilityControlData"`: Name of this class.
    #[serde(default = "HkaKeyFrameHierarchyUtilityControlData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa3d0ac71`: Unique value of this class.
    #[serde(default = "HkaKeyFrameHierarchyUtilityControlData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaKeyFrameHierarchyUtilityControlDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaKeyFrameHierarchyUtilityControlDataHkParam<'a>>
}

impl HkaKeyFrameHierarchyUtilityControlData<'_> {
    /// Return `"hkaKeyFrameHierarchyUtilityControlData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaKeyFrameHierarchyUtilityControlData".into()
    }

    /// Return `"0xa3d0ac71"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa3d0ac71".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaKeyFrameHierarchyUtilityControlDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"hierarchyGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hierarchyGain")]
    HierarchyGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"velocityDamping"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityDamping")]
    VelocityDamping(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"accelerationGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "accelerationGain")]
    AccelerationGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"velocityGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityGain")]
    VelocityGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"positionGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionGain")]
    PositionGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"positionMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionMaxLinearVelocity")]
    PositionMaxLinearVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"positionMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionMaxAngularVelocity")]
    PositionMaxAngularVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapGain"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapGain")]
    SnapGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxLinearVelocity")]
    SnapMaxLinearVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxAngularVelocity")]
    SnapMaxAngularVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapMaxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxLinearDistance")]
    SnapMaxLinearDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapMaxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxAngularDistance")]
    SnapMaxAngularDistance(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaKeyFrameHierarchyUtilityControlDataHkParam<'de>, "@name",
    ("hierarchyGain" => HierarchyGain(f64)),
    ("velocityDamping" => VelocityDamping(f64)),
    ("accelerationGain" => AccelerationGain(f64)),
    ("velocityGain" => VelocityGain(f64)),
    ("positionGain" => PositionGain(f64)),
    ("positionMaxLinearVelocity" => PositionMaxLinearVelocity(f64)),
    ("positionMaxAngularVelocity" => PositionMaxAngularVelocity(f64)),
    ("snapGain" => SnapGain(f64)),
    ("snapMaxLinearVelocity" => SnapMaxLinearVelocity(f64)),
    ("snapMaxAngularVelocity" => SnapMaxAngularVelocity(f64)),
    ("snapMaxLinearDistance" => SnapMaxLinearDistance(f64)),
    ("snapMaxAngularDistance" => SnapMaxAngularDistance(f64)),
}
