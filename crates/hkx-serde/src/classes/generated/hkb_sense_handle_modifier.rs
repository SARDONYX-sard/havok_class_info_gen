//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSenseHandleModifier`, a class defined in C++
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
/// -    size: 160
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSenseHandleModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSenseHandleModifier"`: The original C++ class name.
    #[serde(default = "HkbSenseHandleModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2a064d99`: Unique value of this class.
    #[serde(default = "HkbSenseHandleModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSenseHandleModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSenseHandleModifierHkParam<'a>>
}

impl HkbSenseHandleModifier<'_> {
    /// Return `"hkbSenseHandleModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbSenseHandleModifier".into()
    }

    /// Return `"0x2a064d99"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2a064d99".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSenseHandleModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "handle", skip_serializing)]
    Handle(HkbHandle),
    /// # Field information in the original C++ class
    /// -   name:`"sensorLocalOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorLocalOffset")]
    SensorLocalOffset(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ranges"`
    /// -   type: `hkArray&lt;struct hkbSenseHandleModifierRange&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ranges")]
    Ranges(Vec<HkbSenseHandleModifierRange>),
    /// # Field information in the original C++ class
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleOut")]
    HandleOut(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"handleIn"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleIn")]
    HandleIn(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"sensorLocalFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorLocalFrameName")]
    SensorLocalFrameName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDistance")]
    MinDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistance")]
    MaxDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"distanceOut"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distanceOut")]
    DistanceOut(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"sensorRagdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorRagdollBoneIndex")]
    SensorRagdollBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"sensorAnimationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorAnimationBoneIndex")]
    SensorAnimationBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"sensingMode"`
    /// -   type: `enum SensingMode`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensingMode")]
    SensingMode(SensingMode),
    /// # Field information in the original C++ class
    /// -   name:`"extrapolateSensorPosition"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolateSensorPosition")]
    ExtrapolateSensorPosition(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"keepFirstSensedHandle"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepFirstSensedHandle")]
    KeepFirstSensedHandle(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"foundHandleOut"`
    /// -   type: `hkBool`
    /// - offset: 147
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "foundHandleOut")]
    FoundHandleOut(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rangeIndexForEventToSendNextUpdate"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rangeIndexForEventToSendNextUpdate", skip_serializing)]
    RangeIndexForEventToSendNextUpdate(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifierHkParam<'de>, "@name",
    ("handle" => Handle(HkbHandle)),
    ("sensorLocalOffset" => SensorLocalOffset(Vector4<f32>)),
    ("ranges" => Ranges(Vec<HkbSenseHandleModifierRange>)),
    ("handleOut" => HandleOut(Cow<'a, str>)),
    ("handleIn" => HandleIn(Cow<'a, str>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'a, str>>)),
    ("sensorLocalFrameName" => SensorLocalFrameName(Primitive<Cow<'a, str>>)),
    ("minDistance" => MinDistance(Primitive<f32>)),
    ("maxDistance" => MaxDistance(Primitive<f32>)),
    ("distanceOut" => DistanceOut(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("sensorRagdollBoneIndex" => SensorRagdollBoneIndex(Primitive<i16>)),
    ("sensorAnimationBoneIndex" => SensorAnimationBoneIndex(Primitive<i16>)),
    ("sensingMode" => SensingMode(SensingMode)),
    ("extrapolateSensorPosition" => ExtrapolateSensorPosition(Primitive<bool>)),
    ("keepFirstSensedHandle" => KeepFirstSensedHandle(Primitive<bool>)),
    ("foundHandleOut" => FoundHandleOut(Primitive<bool>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("rangeIndexForEventToSendNextUpdate" => RangeIndexForEventToSendNextUpdate(Primitive<i32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SensingMode {
    #[serde(rename = "SENSE_IN_NEARBY_RIGID_BODIES")]
    SenseInNearbyRigidBodies = 0,
    #[serde(rename = "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER")]
    SenseInRigidBodiesOutsideThisCharacter = 1,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_RIGID_BODIES")]
    SenseInOtherCharacterRigidBodies = 2,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_RIGID_BODIES")]
    SenseInThisCharacterRigidBodies = 3,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES")]
    SenseInGivenCharacterRigidBodies = 4,
    #[serde(rename = "SENSE_IN_GIVEN_RIGID_BODY")]
    SenseInGivenRigidBody = 5,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_SKELETON")]
    SenseInOtherCharacterSkeleton = 6,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_SKELETON")]
    SenseInThisCharacterSkeleton = 7,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_SKELETON")]
    SenseInGivenCharacterSkeleton = 8,
    #[serde(rename = "SENSE_IN_GIVEN_LOCAL_FRAME_GROUP")]
    SenseInGivenLocalFrameGroup = 9,
}
