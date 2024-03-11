//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkModifier`, a class defined in C++
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

    /// `"hkbFootIkModifier"`: The original C++ class name.
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
    /// Return `"hkbFootIkModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkModifier".into()
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
    /// # Field information in the original C++ class
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gains")]
    Gains(HkbFootIkGains),
    /// # Field information in the original C++ class
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierLeg&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(Vec<HkbFootIkModifierLeg>),
    /// # Field information in the original C++ class
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"errorOut"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOut")]
    ErrorOut(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"useTrackData"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useTrackData")]
    UseTrackData(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 182
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"alignMode"`
    /// -   type: `enum AlignMode`
    /// - offset: 183
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignMode")]
    AlignMode(AlignMode),
    /// # Field information in the original C++ class
    /// -   name:`"internalLegData"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierInternalLegData&gt;`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalLegData", skip_serializing)]
    InternalLegData(Vec<HkbFootIkModifierInternalLegData>),
    /// # Field information in the original C++ class
    /// -   name:`"prevIsFootIkEnabled"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "prevIsFootIkEnabled", skip_serializing)]
    PrevIsFootIkEnabled(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"isSetUp"`
    /// -   type: `hkBool`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isSetUp", skip_serializing)]
    IsSetUp(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isGroundPositionValid"`
    /// -   type: `hkBool`
    /// - offset: 201
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isGroundPositionValid", skip_serializing)]
    IsGroundPositionValid(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierHkParam<'de>, "@name",
    ("gains" => Gains(HkbFootIkGains)),
    ("legs" => Legs(Vec<HkbFootIkModifierLeg>)),
    ("raycastDistanceUp" => RaycastDistanceUp(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(Primitive<f32>)),
    ("errorOut" => ErrorOut(Primitive<f32>)),
    ("errorOutTranslation" => ErrorOutTranslation(Vector4<f32>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Quaternion<f32>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("forwardAlignFraction" => ForwardAlignFraction(Primitive<f32>)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(Primitive<f32>)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(Primitive<f32>)),
    ("useTrackData" => UseTrackData(Primitive<bool>)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(Primitive<bool>)),
    ("useCharacterUpVector" => UseCharacterUpVector(Primitive<bool>)),
    ("alignMode" => AlignMode(AlignMode)),
    ("internalLegData" => InternalLegData(Vec<HkbFootIkModifierInternalLegData>)),
    ("prevIsFootIkEnabled" => PrevIsFootIkEnabled(Primitive<f32>)),
    ("isSetUp" => IsSetUp(Primitive<bool>)),
    ("isGroundPositionValid" => IsGroundPositionValid(Primitive<bool>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AlignMode {
    #[serde(rename = "ALIGN_MODE_FORWARD_RIGHT")]
    AlignModeForwardRight = 0,
    #[serde(rename = "ALIGN_MODE_FORWARD")]
    AlignModeForward = 1,
}
