//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkModifier`, a class defined in C++
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
/// -    size: 208
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkModifier"`: Name of this class.
    #[serde(default = "HkbFootIkModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xed8966c0`: Unique value of this class.
    #[serde(default = "HkbFootIkModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkModifierHkParam<'a>>
}

impl HkbFootIkModifier<'_> {
    /// Return `"hkbFootIkModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbFootIkModifier".into()
    }

    /// Return `"0xed8966c0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xed8966c0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gains")]
    Gains(HkbFootIkGains),
    /// # Information on fields in the original C++ class
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierLeg&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(Vec<HkbFootIkModifierLeg>),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"errorOut"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOut")]
    ErrorOut(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"useTrackData"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useTrackData")]
    UseTrackData(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 182
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"alignMode"`
    /// -   type: `enum AlignMode`
    /// - offset: 183
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignMode")]
    AlignMode(AlignMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"internalLegData"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierInternalLegData&gt;`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalLegData", skip_serializing)]
    InternalLegData(Vec<HkbFootIkModifierInternalLegData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"prevIsFootIkEnabled"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "prevIsFootIkEnabled", skip_serializing)]
    PrevIsFootIkEnabled(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isSetUp"`
    /// -   type: `hkBool`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isSetUp", skip_serializing)]
    IsSetUp(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isGroundPositionValid"`
    /// -   type: `hkBool`
    /// - offset: 201
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isGroundPositionValid", skip_serializing)]
    IsGroundPositionValid(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierHkParam<'de>, "@name",
    ("gains" => Gains(HkbFootIkGains)),
    ("legs" => Legs(Vec<HkbFootIkModifierLeg>)),
    ("raycastDistanceUp" => RaycastDistanceUp(f64)),
    ("raycastDistanceDown" => RaycastDistanceDown(f64)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(f64)),
    ("errorOut" => ErrorOut(f64)),
    ("errorOutTranslation" => ErrorOutTranslation(cgmath::Vector4<f32>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(cgmath::Quaternion<f32>)),
    ("verticalOffset" => VerticalOffset(f64)),
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("forwardAlignFraction" => ForwardAlignFraction(f64)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(f64)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(f64)),
    ("useTrackData" => UseTrackData(bool)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(bool)),
    ("useCharacterUpVector" => UseCharacterUpVector(bool)),
    ("alignMode" => AlignMode(AlignMode)),
    ("internalLegData" => InternalLegData(Vec<HkbFootIkModifierInternalLegData>)),
    ("prevIsFootIkEnabled" => PrevIsFootIkEnabled(f64)),
    ("isSetUp" => IsSetUp(bool)),
    ("isGroundPositionValid" => IsGroundPositionValid(bool)),
    ("timeStep" => TimeStep(f64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AlignMode {
    #[serde(rename = "ALIGN_MODE_FORWARD_RIGHT")]
    AlignModeForwardRight = 0,
    #[serde(rename = "ALIGN_MODE_FORWARD")]
    AlignModeForward = 1,
}
