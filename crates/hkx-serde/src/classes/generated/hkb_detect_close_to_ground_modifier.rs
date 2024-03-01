//! A Rust structure that implements a serializer/deserializer corresponding to `hkbDetectCloseToGroundModifier`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbDetectCloseToGroundModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbDetectCloseToGroundModifier"`: Name of this class.
    #[serde(default = "HkbDetectCloseToGroundModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x981687b2`: Unique value of this class.
    #[serde(default = "HkbDetectCloseToGroundModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbDetectCloseToGroundModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbDetectCloseToGroundModifierHkParam<'a>>
}

impl HkbDetectCloseToGroundModifier<'_> {
    /// Return `"hkbDetectCloseToGroundModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbDetectCloseToGroundModifier".into()
    }

    /// Return `"0x981687b2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x981687b2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDetectCloseToGroundModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"closeToGroundEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closeToGroundEvent")]
    CloseToGroundEvent(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"closeToGroundHeight"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closeToGroundHeight")]
    CloseToGroundHeight(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"animBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 66
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animBoneIndex")]
    AnimBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"isCloseToGround"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isCloseToGround", skip_serializing)]
    IsCloseToGround(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbDetectCloseToGroundModifierHkParam<'de>, "@name",
    ("closeToGroundEvent" => CloseToGroundEvent(HkbEventProperty)),
    ("closeToGroundHeight" => CloseToGroundHeight(f64)),
    ("raycastDistanceDown" => RaycastDistanceDown(f64)),
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("boneIndex" => BoneIndex(i16)),
    ("animBoneIndex" => AnimBoneIndex(i16)),
    ("isCloseToGround" => IsCloseToGround(bool)),
}
