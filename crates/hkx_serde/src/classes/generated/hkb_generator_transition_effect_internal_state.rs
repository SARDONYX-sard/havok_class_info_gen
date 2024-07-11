//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorTransitionEffectInternalState`
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

/// `hkbGeneratorTransitionEffectInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd6692b5d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbGeneratorTransitionEffectInternalState {
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
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub time_in_transition: f32,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub effective_blend_in_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub effective_blend_out_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum ToGeneratorState`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub to_generator_state: ToGeneratorState,
    /// # C++ Class Fields Info
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    pub echo_transition_generator: bool,
    /// # C++ Class Fields Info
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    pub echo_to_generator: bool,
    /// # C++ Class Fields Info
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    pub just_activated: bool,
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub update_active_nodes: bool,
    /// # C++ Class Fields Info
    /// -   name:`"stage"`
    /// -   type: `enum Stage`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    pub stage: Stage,
}

impl Serialize for HkbGeneratorTransitionEffectInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbGeneratorTransitionEffectInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbGeneratorTransitionEffectInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbGeneratorTransitionEffectInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbGeneratorTransitionEffectInternalStateVisitor>> for HkbGeneratorTransitionEffectInternalState {
    fn from(_values: Vec<HkbGeneratorTransitionEffectInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut time_in_transition = None;
            let mut duration = None;
            let mut effective_blend_in_duration = None;
            let mut effective_blend_out_duration = None;
            let mut to_generator_state = None;
            let mut echo_transition_generator = None;
            let mut echo_to_generator = None;
            let mut just_activated = None;
            let mut update_active_nodes = None;
            let mut stage = None;


        for _value in _values {
            match _value {
                HkbGeneratorTransitionEffectInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::TimeInTransition(m) => time_in_transition = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::Duration(m) => duration = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::EffectiveBlendInDuration(m) => effective_blend_in_duration = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::EffectiveBlendOutDuration(m) => effective_blend_out_duration = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::ToGeneratorState(m) => to_generator_state = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::EchoTransitionGenerator(m) => echo_transition_generator = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::EchoToGenerator(m) => echo_to_generator = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::JustActivated(m) => just_activated = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::UpdateActiveNodes(m) => update_active_nodes = Some(m),
                HkbGeneratorTransitionEffectInternalStateVisitor::Stage(m) => stage = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            time_in_transition: time_in_transition.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            effective_blend_in_duration: effective_blend_in_duration.unwrap_or_default().into_inner(),
            effective_blend_out_duration: effective_blend_out_duration.unwrap_or_default().into_inner(),
            to_generator_state: to_generator_state.unwrap_or_default().into_inner(),
            echo_transition_generator: echo_transition_generator.unwrap_or_default().into_inner(),
            echo_to_generator: echo_to_generator.unwrap_or_default().into_inner(),
            just_activated: just_activated.unwrap_or_default().into_inner(),
            update_active_nodes: update_active_nodes.unwrap_or_default().into_inner(),
            stage: stage.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbGeneratorTransitionEffectInternalState> for Vec<HkbGeneratorTransitionEffectInternalStateVisitor> {
    fn from(data: &HkbGeneratorTransitionEffectInternalState) -> Self {
        vec![
            HkbGeneratorTransitionEffectInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::TimeInTransition(data.time_in_transition.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::Duration(data.duration.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::EffectiveBlendInDuration(data.effective_blend_in_duration.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::EffectiveBlendOutDuration(data.effective_blend_out_duration.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::ToGeneratorState(data.to_generator_state.clone().into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::EchoTransitionGenerator(data.echo_transition_generator.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::EchoToGenerator(data.echo_to_generator.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::JustActivated(data.just_activated.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::UpdateActiveNodes(data.update_active_nodes.into()),
            HkbGeneratorTransitionEffectInternalStateVisitor::Stage(data.stage.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbGeneratorTransitionEffectInternalState {
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
enum HkbGeneratorTransitionEffectInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "timeInTransition")]
    TimeInTransition(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "effectiveBlendInDuration")]
    EffectiveBlendInDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "effectiveBlendOutDuration")]
    EffectiveBlendOutDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "toGeneratorState")]
    ToGeneratorState(Primitive<ToGeneratorState>),
    /// Visitor fields
    #[serde(rename = "echoTransitionGenerator")]
    EchoTransitionGenerator(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "echoToGenerator")]
    EchoToGenerator(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "justActivated")]
    JustActivated(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "updateActiveNodes")]
    UpdateActiveNodes(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "stage")]
    Stage(Primitive<Stage>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffectInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(Primitive<f32>)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(Primitive<f32>)),
    ("toGeneratorState" => ToGeneratorState(Primitive<ToGeneratorState>)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(Primitive<bool>)),
    ("echoToGenerator" => EchoToGenerator(Primitive<bool>)),
    ("justActivated" => JustActivated(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stage" => Stage(Primitive<Stage>)),
}
