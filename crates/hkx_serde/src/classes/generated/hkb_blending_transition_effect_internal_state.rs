//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendingTransitionEffectInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbBlendingTransitionEffectInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xb18c70c2`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBlendingTransitionEffectInternalState {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub character_pose_at_beginning_of_transition: HkArrayMatrix3<QsTransform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub time_remaining: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub time_in_transition: f32,
    /// # C++ Class Fields Info
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub apply_self_transition: bool,
    /// # C++ Class Fields Info
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    pub initialize_character_pose: bool,
}

impl Serialize for HkbBlendingTransitionEffectInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBlendingTransitionEffectInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBlendingTransitionEffectInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBlendingTransitionEffectInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbBlendingTransitionEffectInternalStateVisitor>> for HkbBlendingTransitionEffectInternalState {
    fn from(_values: Vec<HkbBlendingTransitionEffectInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_pose_at_beginning_of_transition = None;
            let mut time_remaining = None;
            let mut time_in_transition = None;
            let mut apply_self_transition = None;
            let mut initialize_character_pose = None;


        for _value in _values {
            match _value {
                HkbBlendingTransitionEffectInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::CharacterPoseAtBeginningOfTransition(m) => character_pose_at_beginning_of_transition = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::TimeRemaining(m) => time_remaining = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::TimeInTransition(m) => time_in_transition = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::ApplySelfTransition(m) => apply_self_transition = Some(m),
                HkbBlendingTransitionEffectInternalStateVisitor::InitializeCharacterPose(m) => initialize_character_pose = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_pose_at_beginning_of_transition: character_pose_at_beginning_of_transition.unwrap_or_default(),
            time_remaining: time_remaining.unwrap_or_default().into_inner(),
            time_in_transition: time_in_transition.unwrap_or_default().into_inner(),
            apply_self_transition: apply_self_transition.unwrap_or_default().into_inner(),
            initialize_character_pose: initialize_character_pose.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbBlendingTransitionEffectInternalState> for Vec<HkbBlendingTransitionEffectInternalStateVisitor> {
    fn from(data: &HkbBlendingTransitionEffectInternalState) -> Self {
        vec![
            HkbBlendingTransitionEffectInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBlendingTransitionEffectInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbBlendingTransitionEffectInternalStateVisitor::CharacterPoseAtBeginningOfTransition(data.character_pose_at_beginning_of_transition.clone()),
            HkbBlendingTransitionEffectInternalStateVisitor::TimeRemaining(data.time_remaining.into()),
            HkbBlendingTransitionEffectInternalStateVisitor::TimeInTransition(data.time_in_transition.into()),
            HkbBlendingTransitionEffectInternalStateVisitor::ApplySelfTransition(data.apply_self_transition.into()),
            HkbBlendingTransitionEffectInternalStateVisitor::InitializeCharacterPose(data.initialize_character_pose.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBlendingTransitionEffectInternalState {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkbBlendingTransitionEffectInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "characterPoseAtBeginningOfTransition")]
    CharacterPoseAtBeginningOfTransition(HkArrayMatrix3<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "timeRemaining")]
    TimeRemaining(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeInTransition")]
    TimeInTransition(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "applySelfTransition")]
    ApplySelfTransition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "initializeCharacterPose")]
    InitializeCharacterPose(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffectInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayMatrix3<QsTransform<f32>>)),
    ("timeRemaining" => TimeRemaining(Primitive<f32>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("applySelfTransition" => ApplySelfTransition(Primitive<bool>)),
    ("initializeCharacterPose" => InitializeCharacterPose(Primitive<bool>)),
}
