//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipGenerator`
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

/// `hkbClipGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x333b85b9`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbClipGenerator<'a> {
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
    /// -   name:`"animationName"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub animation_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"triggers"`
    /// -   type: `struct hkbClipTriggerArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub triggers: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"cropStartAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub crop_start_amount_local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cropEndAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub crop_end_amount_local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startTime"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub start_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub playback_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enforcedDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub enforced_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"userControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub user_controlled_time_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub animation_binding_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum PlaybackMode`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    pub mode: PlaybackMode,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkInt8`
    /// - offset: 75
    /// -  flags: `FLAGS_NONE`
    pub flags: i8,
    /// # C++ Class Fields Info
    /// -   name:`"animDatas"`
    /// -   type: `hkArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub anim_datas: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"animationControl"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub animation_control: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"originalTriggers"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub original_triggers: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mapperData"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mapper_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"binding"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub binding: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredAnimation"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mirrored_animation: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub extracted_motion: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"echos"`
    /// -   type: `hkArray<void>`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub echos: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub previous_user_controlled_time_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub buffer_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub echo_buffer_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub at_end: bool,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub ignore_start_time: bool,
    /// # C++ Class Fields Info
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub ping_pong_backward: bool,
}

impl Serialize for HkbClipGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbClipGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbClipGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbClipGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbClipGeneratorVisitor<'a>>> for HkbClipGenerator<'a> {
    fn from(_values: Vec<HkbClipGeneratorVisitor<'a>>) -> Self {
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
            let mut animation_name = None;
            let mut triggers = None;
            let mut crop_start_amount_local_time = None;
            let mut crop_end_amount_local_time = None;
            let mut start_time = None;
            let mut playback_speed = None;
            let mut enforced_duration = None;
            let mut user_controlled_time_fraction = None;
            let mut animation_binding_index = None;
            let mut mode = None;
            let mut flags = None;
            let mut anim_datas = None;
            let mut animation_control = None;
            let mut original_triggers = None;
            let mut mapper_data = None;
            let mut binding = None;
            let mut mirrored_animation = None;
            let mut extracted_motion = None;
            let mut echos = None;
            let mut local_time = None;
            let mut time = None;
            let mut previous_user_controlled_time_fraction = None;
            let mut buffer_size = None;
            let mut echo_buffer_size = None;
            let mut at_end = None;
            let mut ignore_start_time = None;
            let mut ping_pong_backward = None;


        for _value in _values {
            match _value {
                HkbClipGeneratorVisitor::UserData(m) => user_data = Some(m),
                HkbClipGeneratorVisitor::Name(m) => name = Some(m),
                HkbClipGeneratorVisitor::Id(m) => id = Some(m),
                HkbClipGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                HkbClipGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                HkbClipGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbClipGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbClipGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbClipGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbClipGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbClipGeneratorVisitor::AnimationName(m) => animation_name = Some(m),
                HkbClipGeneratorVisitor::Triggers(m) => triggers = Some(m),
                HkbClipGeneratorVisitor::CropStartAmountLocalTime(m) => crop_start_amount_local_time = Some(m),
                HkbClipGeneratorVisitor::CropEndAmountLocalTime(m) => crop_end_amount_local_time = Some(m),
                HkbClipGeneratorVisitor::StartTime(m) => start_time = Some(m),
                HkbClipGeneratorVisitor::PlaybackSpeed(m) => playback_speed = Some(m),
                HkbClipGeneratorVisitor::EnforcedDuration(m) => enforced_duration = Some(m),
                HkbClipGeneratorVisitor::UserControlledTimeFraction(m) => user_controlled_time_fraction = Some(m),
                HkbClipGeneratorVisitor::AnimationBindingIndex(m) => animation_binding_index = Some(m),
                HkbClipGeneratorVisitor::Mode(m) => mode = Some(m),
                HkbClipGeneratorVisitor::Flags(m) => flags = Some(m),
                HkbClipGeneratorVisitor::AnimDatas(m) => anim_datas = Some(m),
                HkbClipGeneratorVisitor::AnimationControl(m) => animation_control = Some(m),
                HkbClipGeneratorVisitor::OriginalTriggers(m) => original_triggers = Some(m),
                HkbClipGeneratorVisitor::MapperData(m) => mapper_data = Some(m),
                HkbClipGeneratorVisitor::Binding(m) => binding = Some(m),
                HkbClipGeneratorVisitor::MirroredAnimation(m) => mirrored_animation = Some(m),
                HkbClipGeneratorVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkbClipGeneratorVisitor::Echos(m) => echos = Some(m),
                HkbClipGeneratorVisitor::LocalTime(m) => local_time = Some(m),
                HkbClipGeneratorVisitor::Time(m) => time = Some(m),
                HkbClipGeneratorVisitor::PreviousUserControlledTimeFraction(m) => previous_user_controlled_time_fraction = Some(m),
                HkbClipGeneratorVisitor::BufferSize(m) => buffer_size = Some(m),
                HkbClipGeneratorVisitor::EchoBufferSize(m) => echo_buffer_size = Some(m),
                HkbClipGeneratorVisitor::AtEnd(m) => at_end = Some(m),
                HkbClipGeneratorVisitor::IgnoreStartTime(m) => ignore_start_time = Some(m),
                HkbClipGeneratorVisitor::PingPongBackward(m) => ping_pong_backward = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
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
            animation_name: animation_name.unwrap_or_default().into_inner(),
            triggers: triggers.unwrap_or_default().into_inner(),
            crop_start_amount_local_time: crop_start_amount_local_time.unwrap_or_default().into_inner(),
            crop_end_amount_local_time: crop_end_amount_local_time.unwrap_or_default().into_inner(),
            start_time: start_time.unwrap_or_default().into_inner(),
            playback_speed: playback_speed.unwrap_or_default().into_inner(),
            enforced_duration: enforced_duration.unwrap_or_default().into_inner(),
            user_controlled_time_fraction: user_controlled_time_fraction.unwrap_or_default().into_inner(),
            animation_binding_index: animation_binding_index.unwrap_or_default().into_inner(),
            mode: mode.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),
            anim_datas: anim_datas.unwrap_or_default(),
            animation_control: animation_control.unwrap_or_default().into_inner(),
            original_triggers: original_triggers.unwrap_or_default().into_inner(),
            mapper_data: mapper_data.unwrap_or_default().into_inner(),
            binding: binding.unwrap_or_default().into_inner(),
            mirrored_animation: mirrored_animation.unwrap_or_default().into_inner(),
            extracted_motion: extracted_motion.unwrap_or_default().into_inner(),
            echos: echos.unwrap_or_default(),
            local_time: local_time.unwrap_or_default().into_inner(),
            time: time.unwrap_or_default().into_inner(),
            previous_user_controlled_time_fraction: previous_user_controlled_time_fraction.unwrap_or_default().into_inner(),
            buffer_size: buffer_size.unwrap_or_default().into_inner(),
            echo_buffer_size: echo_buffer_size.unwrap_or_default().into_inner(),
            at_end: at_end.unwrap_or_default().into_inner(),
            ignore_start_time: ignore_start_time.unwrap_or_default().into_inner(),
            ping_pong_backward: ping_pong_backward.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbClipGenerator<'a>> for Vec<HkbClipGeneratorVisitor<'a>> {
    fn from(data: &HkbClipGenerator<'a>) -> Self {
        vec![
            HkbClipGeneratorVisitor::UserData(data.user_data.into()),
            HkbClipGeneratorVisitor::Name(data.name.clone().into()),
            HkbClipGeneratorVisitor::Id(data.id.into()),
            HkbClipGeneratorVisitor::CloneState(data.clone_state.into()),
            HkbClipGeneratorVisitor::PadNode(data.pad_node.clone()),
            HkbClipGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbClipGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbClipGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbClipGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbClipGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            HkbClipGeneratorVisitor::AnimationName(data.animation_name.clone().into()),
            HkbClipGeneratorVisitor::Triggers(data.triggers.clone().into()),
            HkbClipGeneratorVisitor::CropStartAmountLocalTime(data.crop_start_amount_local_time.into()),
            HkbClipGeneratorVisitor::CropEndAmountLocalTime(data.crop_end_amount_local_time.into()),
            HkbClipGeneratorVisitor::StartTime(data.start_time.into()),
            HkbClipGeneratorVisitor::PlaybackSpeed(data.playback_speed.into()),
            HkbClipGeneratorVisitor::EnforcedDuration(data.enforced_duration.into()),
            HkbClipGeneratorVisitor::UserControlledTimeFraction(data.user_controlled_time_fraction.into()),
            HkbClipGeneratorVisitor::AnimationBindingIndex(data.animation_binding_index.into()),
            HkbClipGeneratorVisitor::Mode(data.mode.clone().into()),
            HkbClipGeneratorVisitor::Flags(data.flags.into()),
            HkbClipGeneratorVisitor::AnimDatas(data.anim_datas.clone()),
            HkbClipGeneratorVisitor::AnimationControl(data.animation_control.clone().into()),
            HkbClipGeneratorVisitor::OriginalTriggers(data.original_triggers.clone().into()),
            HkbClipGeneratorVisitor::MapperData(data.mapper_data.clone().into()),
            HkbClipGeneratorVisitor::Binding(data.binding.clone().into()),
            HkbClipGeneratorVisitor::MirroredAnimation(data.mirrored_animation.clone().into()),
            HkbClipGeneratorVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkbClipGeneratorVisitor::Echos(data.echos.clone()),
            HkbClipGeneratorVisitor::LocalTime(data.local_time.into()),
            HkbClipGeneratorVisitor::Time(data.time.into()),
            HkbClipGeneratorVisitor::PreviousUserControlledTimeFraction(data.previous_user_controlled_time_fraction.into()),
            HkbClipGeneratorVisitor::BufferSize(data.buffer_size.into()),
            HkbClipGeneratorVisitor::EchoBufferSize(data.echo_buffer_size.into()),
            HkbClipGeneratorVisitor::AtEnd(data.at_end.into()),
            HkbClipGeneratorVisitor::IgnoreStartTime(data.ignore_start_time.into()),
            HkbClipGeneratorVisitor::PingPongBackward(data.ping_pong_backward.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbClipGenerator<'de> {
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
enum HkbClipGeneratorVisitor<'a> {
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
    #[serde(rename = "animationName")]
    AnimationName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "triggers")]
    Triggers(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cropStartAmountLocalTime")]
    CropStartAmountLocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cropEndAmountLocalTime")]
    CropEndAmountLocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startTime")]
    StartTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "enforcedDuration")]
    EnforcedDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "userControlledTimeFraction")]
    UserControlledTimeFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "animationBindingIndex")]
    AnimationBindingIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "mode")]
    Mode(Primitive<PlaybackMode>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "animDatas", skip_serializing)]
    AnimDatas(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "animationControl", skip_serializing)]
    AnimationControl(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "originalTriggers", skip_serializing)]
    OriginalTriggers(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mapperData", skip_serializing)]
    MapperData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "binding", skip_serializing)]
    Binding(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mirroredAnimation", skip_serializing)]
    MirroredAnimation(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "extractedMotion", skip_serializing)]
    ExtractedMotion(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "echos", skip_serializing)]
    Echos(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "localTime", skip_serializing)]
    LocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousUserControlledTimeFraction", skip_serializing)]
    PreviousUserControlledTimeFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bufferSize", skip_serializing)]
    BufferSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "echoBufferSize", skip_serializing)]
    EchoBufferSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "atEnd", skip_serializing)]
    AtEnd(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "ignoreStartTime", skip_serializing)]
    IgnoreStartTime(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "pingPongBackward", skip_serializing)]
    PingPongBackward(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorVisitor<'de>, "@name",
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
    ("animationName" => AnimationName(Primitive<Cow<'de, str>>)),
    ("triggers" => Triggers(Primitive<Cow<'de, str>>)),
    ("cropStartAmountLocalTime" => CropStartAmountLocalTime(Primitive<f32>)),
    ("cropEndAmountLocalTime" => CropEndAmountLocalTime(Primitive<f32>)),
    ("startTime" => StartTime(Primitive<f32>)),
    ("playbackSpeed" => PlaybackSpeed(Primitive<f32>)),
    ("enforcedDuration" => EnforcedDuration(Primitive<f32>)),
    ("userControlledTimeFraction" => UserControlledTimeFraction(Primitive<f32>)),
    ("animationBindingIndex" => AnimationBindingIndex(Primitive<i16>)),
    ("mode" => Mode(Primitive<PlaybackMode>)),
    ("flags" => Flags(Primitive<i8>)),
    ("animDatas" => AnimDatas(HkArrayRef<()>)),
    ("animationControl" => AnimationControl(Primitive<Cow<'de, str>>)),
    ("originalTriggers" => OriginalTriggers(Primitive<Cow<'de, str>>)),
    ("mapperData" => MapperData(Primitive<Cow<'de, str>>)),
    ("binding" => Binding(Primitive<Cow<'de, str>>)),
    ("mirroredAnimation" => MirroredAnimation(Primitive<Cow<'de, str>>)),
    ("extractedMotion" => ExtractedMotion(Primitive<QsTransform<f32>>)),
    ("echos" => Echos(HkArrayRef<()>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("time" => Time(Primitive<f32>)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(Primitive<f32>)),
    ("bufferSize" => BufferSize(Primitive<i32>)),
    ("echoBufferSize" => EchoBufferSize(Primitive<i32>)),
    ("atEnd" => AtEnd(Primitive<bool>)),
    ("ignoreStartTime" => IgnoreStartTime(Primitive<bool>)),
    ("pingPongBackward" => PingPongBackward(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum PlaybackMode {
    #[serde(rename = "MODE_SINGLE_PLAY")]
    #[default]
    ModeSinglePlay = 0,
    #[serde(rename = "MODE_LOOPING")]
    ModeLooping = 1,
    #[serde(rename = "MODE_USER_CONTROLLED")]
    ModeUserControlled = 2,
    #[serde(rename = "MODE_PING_PONG")]
    ModePingPong = 3,
    #[serde(rename = "MODE_COUNT")]
    ModeCount = 4,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ClipFlags {
    #[serde(rename = "FLAG_CONTINUE_MOTION_AT_END")]
    #[default]
    FlagContinueMotionAtEnd = 1,
    #[serde(rename = "FLAG_SYNC_HALF_CYCLE_IN_PING_PONG_MODE")]
    FlagSyncHalfCycleInPingPongMode = 2,
    #[serde(rename = "FLAG_MIRROR")]
    FlagMirror = 4,
    #[serde(rename = "FLAG_FORCE_DENSE_POSE")]
    FlagForceDensePose = 8,
    #[serde(rename = "FLAG_DONT_CONVERT_ANNOTATIONS_TO_TRIGGERS")]
    FlagDontConvertAnnotationsToTriggers = 16,
    #[serde(rename = "FLAG_IGNORE_MOTION")]
    FlagIgnoreMotion = 32,
}
