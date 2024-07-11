//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendingTransitionEffect`
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

/// `hkbBlendingTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkbTransitionEffect`/`0x945da157`
/// - signature: `0xfd8584fe`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBlendingTransitionEffect<'a> {
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub self_transition_mode: SelfTransitionMode,
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    pub event_mode: EventMode,
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub default_event_mode: (),

    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: i16,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub clone_state: (),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_node: CStyleArray<[bool; 1]>,

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variable_binding_set: Cow<'a, str>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub cached_bindables: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub are_bindables_cached: bool,

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
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorStartTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub to_generator_start_time_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagBits`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub flags: FlagBits,
    /// # C++ Class Fields Info
    /// -   name:`"endMode"`
    /// -   type: `enum EndMode`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    pub end_mode: EndMode,
    /// # C++ Class Fields Info
    /// -   name:`"blendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 55
    /// -  flags: `FLAGS_NONE`
    pub blend_curve: BlendCurve,
    /// # C++ Class Fields Info
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub from_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub to_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray<void>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character_pose_at_beginning_of_transition: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_remaining: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_in_transition: f32,
    /// # C++ Class Fields Info
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub apply_self_transition: bool,
    /// # C++ Class Fields Info
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub initialize_character_pose: bool,
}

impl Serialize for HkbBlendingTransitionEffect<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBlendingTransitionEffectVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBlendingTransitionEffect<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBlendingTransitionEffectVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBlendingTransitionEffectVisitor<'a>>> for HkbBlendingTransitionEffect<'a> {
    fn from(_values: Vec<HkbBlendingTransitionEffectVisitor<'a>>) -> Self {
            let mut self_transition_mode = None;
            let mut event_mode = None;
            let mut default_event_mode = None;
            let mut user_data = None;
            let mut name = None;
            let mut id = None;
            let mut clone_state = None;
            let mut pad_node = None;
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut duration = None;
            let mut to_generator_start_time_fraction = None;
            let mut flags = None;
            let mut end_mode = None;
            let mut blend_curve = None;
            let mut from_generator = None;
            let mut to_generator = None;
            let mut character_pose_at_beginning_of_transition = None;
            let mut time_remaining = None;
            let mut time_in_transition = None;
            let mut apply_self_transition = None;
            let mut initialize_character_pose = None;


        for _value in _values {
            match _value {
                HkbBlendingTransitionEffectVisitor::SelfTransitionMode(m) => self_transition_mode = Some(m),
                HkbBlendingTransitionEffectVisitor::EventMode(m) => event_mode = Some(m),
                HkbBlendingTransitionEffectVisitor::DefaultEventMode(m) => default_event_mode = Some(m),
                HkbBlendingTransitionEffectVisitor::UserData(m) => user_data = Some(m),
                HkbBlendingTransitionEffectVisitor::Name(m) => name = Some(m),
                HkbBlendingTransitionEffectVisitor::Id(m) => id = Some(m),
                HkbBlendingTransitionEffectVisitor::CloneState(m) => clone_state = Some(m),
                HkbBlendingTransitionEffectVisitor::PadNode(m) => pad_node = Some(m),
                HkbBlendingTransitionEffectVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbBlendingTransitionEffectVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbBlendingTransitionEffectVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbBlendingTransitionEffectVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBlendingTransitionEffectVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBlendingTransitionEffectVisitor::Duration(m) => duration = Some(m),
                HkbBlendingTransitionEffectVisitor::ToGeneratorStartTimeFraction(m) => to_generator_start_time_fraction = Some(m),
                HkbBlendingTransitionEffectVisitor::Flags(m) => flags = Some(m),
                HkbBlendingTransitionEffectVisitor::EndMode(m) => end_mode = Some(m),
                HkbBlendingTransitionEffectVisitor::BlendCurve(m) => blend_curve = Some(m),
                HkbBlendingTransitionEffectVisitor::FromGenerator(m) => from_generator = Some(m),
                HkbBlendingTransitionEffectVisitor::ToGenerator(m) => to_generator = Some(m),
                HkbBlendingTransitionEffectVisitor::CharacterPoseAtBeginningOfTransition(m) => character_pose_at_beginning_of_transition = Some(m),
                HkbBlendingTransitionEffectVisitor::TimeRemaining(m) => time_remaining = Some(m),
                HkbBlendingTransitionEffectVisitor::TimeInTransition(m) => time_in_transition = Some(m),
                HkbBlendingTransitionEffectVisitor::ApplySelfTransition(m) => apply_self_transition = Some(m),
                HkbBlendingTransitionEffectVisitor::InitializeCharacterPose(m) => initialize_character_pose = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            self_transition_mode: self_transition_mode.unwrap_or_default().into_inner(),
            event_mode: event_mode.unwrap_or_default().into_inner(),
            default_event_mode: default_event_mode.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),
            clone_state: clone_state.unwrap_or_default().into_inner(),
            pad_node: pad_node.unwrap_or_default(),
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            to_generator_start_time_fraction: to_generator_start_time_fraction.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),
            end_mode: end_mode.unwrap_or_default().into_inner(),
            blend_curve: blend_curve.unwrap_or_default().into_inner(),
            from_generator: from_generator.unwrap_or_default().into_inner(),
            to_generator: to_generator.unwrap_or_default().into_inner(),
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
impl<'a> From<&HkbBlendingTransitionEffect<'a>> for Vec<HkbBlendingTransitionEffectVisitor<'a>> {
    fn from(data: &HkbBlendingTransitionEffect<'a>) -> Self {
        vec![
            HkbBlendingTransitionEffectVisitor::SelfTransitionMode(data.self_transition_mode.clone().into()),
            HkbBlendingTransitionEffectVisitor::EventMode(data.event_mode.clone().into()),
            HkbBlendingTransitionEffectVisitor::DefaultEventMode(data.default_event_mode.into()),
            HkbBlendingTransitionEffectVisitor::UserData(data.user_data.into()),
            HkbBlendingTransitionEffectVisitor::Name(data.name.clone().into()),
            HkbBlendingTransitionEffectVisitor::Id(data.id.into()),
            HkbBlendingTransitionEffectVisitor::CloneState(data.clone_state.into()),
            HkbBlendingTransitionEffectVisitor::PadNode(data.pad_node.clone()),
            HkbBlendingTransitionEffectVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbBlendingTransitionEffectVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbBlendingTransitionEffectVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbBlendingTransitionEffectVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBlendingTransitionEffectVisitor::ReferenceCount(data.reference_count.into()),
            HkbBlendingTransitionEffectVisitor::Duration(data.duration.into()),
            HkbBlendingTransitionEffectVisitor::ToGeneratorStartTimeFraction(data.to_generator_start_time_fraction.into()),
            HkbBlendingTransitionEffectVisitor::Flags(data.flags.clone().into()),
            HkbBlendingTransitionEffectVisitor::EndMode(data.end_mode.clone().into()),
            HkbBlendingTransitionEffectVisitor::BlendCurve(data.blend_curve.clone().into()),
            HkbBlendingTransitionEffectVisitor::FromGenerator(data.from_generator.clone().into()),
            HkbBlendingTransitionEffectVisitor::ToGenerator(data.to_generator.clone().into()),
            HkbBlendingTransitionEffectVisitor::CharacterPoseAtBeginningOfTransition(data.character_pose_at_beginning_of_transition.clone()),
            HkbBlendingTransitionEffectVisitor::TimeRemaining(data.time_remaining.into()),
            HkbBlendingTransitionEffectVisitor::TimeInTransition(data.time_in_transition.into()),
            HkbBlendingTransitionEffectVisitor::ApplySelfTransition(data.apply_self_transition.into()),
            HkbBlendingTransitionEffectVisitor::InitializeCharacterPose(data.initialize_character_pose.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBlendingTransitionEffect<'de> {
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
enum HkbBlendingTransitionEffectVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// Visitor fields
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// Visitor fields
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<()>),

    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// Visitor fields
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "toGeneratorStartTimeFraction")]
    ToGeneratorStartTimeFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<FlagBits>),
    /// Visitor fields
    #[serde(rename = "endMode")]
    EndMode(Primitive<EndMode>),
    /// Visitor fields
    #[serde(rename = "blendCurve")]
    BlendCurve(Primitive<BlendCurve>),
    /// Visitor fields
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "characterPoseAtBeginningOfTransition", skip_serializing)]
    CharacterPoseAtBeginningOfTransition(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "timeRemaining", skip_serializing)]
    TimeRemaining(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "applySelfTransition", skip_serializing)]
    ApplySelfTransition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "initializeCharacterPose", skip_serializing)]
    InitializeCharacterPose(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffectVisitor<'de>, "@name",
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<()>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("duration" => Duration(Primitive<f32>)),
    ("toGeneratorStartTimeFraction" => ToGeneratorStartTimeFraction(Primitive<f32>)),
    ("flags" => Flags(Primitive<FlagBits>)),
    ("endMode" => EndMode(Primitive<EndMode>)),
    ("blendCurve" => BlendCurve(Primitive<BlendCurve>)),
    ("fromGenerator" => FromGenerator(Primitive<Cow<'de, str>>)),
    ("toGenerator" => ToGenerator(Primitive<Cow<'de, str>>)),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayRef<()>)),
    ("timeRemaining" => TimeRemaining(Primitive<f32>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("applySelfTransition" => ApplySelfTransition(Primitive<bool>)),
    ("initializeCharacterPose" => InitializeCharacterPose(Primitive<bool>)),
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FlagBits: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_NONE = 0;
        const FLAG_IGNORE_FROM_WORLD_FROM_MODEL = 1;
        const FLAG_SYNC = 2;
        const FLAG_IGNORE_TO_WORLD_FROM_MODEL = 4;
    }
}

impl Default for FlagBits {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for FlagBits {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for FlagBits {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for FlagBits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> serde::Deserialize<'de> for FlagBits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(Self::NULL);
                };
                let mut flags = Self::empty();
                for token in s.split('|') {
                    match token.trim() {
                        "FLAG_NONE" => flags |= Self::FLAG_NONE,
                        "FLAG_IGNORE_FROM_WORLD_FROM_MODEL" => flags |= Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL,
                        "FLAG_SYNC" => flags |= Self::FLAG_SYNC,
                        "FLAG_IGNORE_TO_WORLD_FROM_MODEL" => flags |= Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected FlagBits flags but got '{unknown}'",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected FlagBits flags or integer, but got '{unknown}'"
                                )))
                            }
                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}

impl core::fmt::Display for FlagBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl FlagBits {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_NONE) {
            flags.push("FLAG_NONE");
        }
        if self.contains(Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL) {
            flags.push("FLAG_IGNORE_FROM_WORLD_FROM_MODEL");
        }
        if self.contains(Self::FLAG_SYNC) {
            flags.push("FLAG_SYNC");
        }
        if self.contains(Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL) {
            flags.push("FLAG_IGNORE_TO_WORLD_FROM_MODEL");
        }

        flags.join("|").into()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EndMode {
    #[serde(rename = "END_MODE_NONE")]
    #[default]
    EndModeNone = 0,
    #[serde(rename = "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR")]
    EndModeTransitionUntilEndOfFromGenerator = 1,
    #[serde(rename = "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR")]
    EndModeCapDurationAtEndOfFromGenerator = 2,
}
