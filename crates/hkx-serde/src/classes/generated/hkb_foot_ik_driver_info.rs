//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkDriverInfo`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkDriverInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkDriverInfo"`: Name of this class.
    #[serde(default = "HkbFootIkDriverInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc6a09dbf`: Unique value of this class.
    #[serde(default = "HkbFootIkDriverInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkDriverInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkDriverInfoHkParam<'a>>
}

impl HkbFootIkDriverInfo<'_> {
    /// Return `"hkbFootIkDriverInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbFootIkDriverInfo".into()
    }

    /// Return `"0xc6a09dbf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc6a09dbf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkDriverInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkDriverInfoLeg&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(Vec<HkbFootIkDriverInfoLeg>),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isQuadrupedNarrow"`
    /// -   type: `hkBool`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isQuadrupedNarrow")]
    IsQuadrupedNarrow(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkDriverInfoHkParam<'de>, "@name",
    ("legs" => Legs(Vec<HkbFootIkDriverInfoLeg>)),
    ("raycastDistanceUp" => RaycastDistanceUp(f64)),
    ("raycastDistanceDown" => RaycastDistanceDown(f64)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(f64)),
    ("verticalOffset" => VerticalOffset(f64)),
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("forwardAlignFraction" => ForwardAlignFraction(f64)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(f64)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(f64)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(bool)),
    ("useCharacterUpVector" => UseCharacterUpVector(bool)),
    ("isQuadrupedNarrow" => IsQuadrupedNarrow(bool)),
}