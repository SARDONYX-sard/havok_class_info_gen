//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSSynchronizedClipGenerator`
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

/// `BSSynchronizedClipGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xd83bea64`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsSynchronizedClipGenerator<'a> {
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
    /// -   name:`"pClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_clip_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"SyncAnimPrefix"`
    /// -   type: `char*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub sync_anim_prefix: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"bSyncClipIgnoreMarkPlacement"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub b_sync_clip_ignore_mark_placement: bool,
    /// # C++ Class Fields Info
    /// -   name:`"fGetToMarkTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub f_get_to_mark_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fMarkErrorThreshold"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub f_mark_error_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bLeadCharacter"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub b_lead_character: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bReorientSupportChar"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    pub b_reorient_support_char: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bApplyMotionFromRoot"`
    /// -   type: `hkBool`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    pub b_apply_motion_from_root: bool,
    /// # C++ Class Fields Info
    /// -   name:`"pSyncScene"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_sync_scene: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"StartMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub start_mark_ws: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"EndMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub end_mark_ws: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"StartMarkMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub start_mark_ms: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"fCurrentLerp"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub f_current_lerp: f32,
    /// # C++ Class Fields Info
    /// -   name:`"pLocalSyncBinding"`
    /// -   type: `void*`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_local_sync_binding: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pEventMap"`
    /// -   type: `void*`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_event_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sAnimationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    pub s_animation_binding_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"bAtMark"`
    /// -   type: `hkBool`
    /// - offset: 238
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_at_mark: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bAllCharactersInScene"`
    /// -   type: `hkBool`
    /// - offset: 239
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_all_characters_in_scene: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bAllCharactersAtMarks"`
    /// -   type: `hkBool`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_all_characters_at_marks: bool,
}

impl Serialize for BsSynchronizedClipGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsSynchronizedClipGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsSynchronizedClipGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsSynchronizedClipGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsSynchronizedClipGeneratorVisitor<'a>>> for BsSynchronizedClipGenerator<'a> {
    fn from(_values: Vec<BsSynchronizedClipGeneratorVisitor<'a>>) -> Self {
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
            let mut p_clip_generator = None;
            let mut sync_anim_prefix = None;
            let mut b_sync_clip_ignore_mark_placement = None;
            let mut f_get_to_mark_time = None;
            let mut f_mark_error_threshold = None;
            let mut b_lead_character = None;
            let mut b_reorient_support_char = None;
            let mut b_apply_motion_from_root = None;
            let mut p_sync_scene = None;
            let mut start_mark_ws = None;
            let mut end_mark_ws = None;
            let mut start_mark_ms = None;
            let mut f_current_lerp = None;
            let mut p_local_sync_binding = None;
            let mut p_event_map = None;
            let mut s_animation_binding_index = None;
            let mut b_at_mark = None;
            let mut b_all_characters_in_scene = None;
            let mut b_all_characters_at_marks = None;


        for _value in _values {
            match _value {
                BsSynchronizedClipGeneratorVisitor::UserData(m) => user_data = Some(m),
                BsSynchronizedClipGeneratorVisitor::Name(m) => name = Some(m),
                BsSynchronizedClipGeneratorVisitor::Id(m) => id = Some(m),
                BsSynchronizedClipGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                BsSynchronizedClipGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                BsSynchronizedClipGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsSynchronizedClipGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsSynchronizedClipGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsSynchronizedClipGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsSynchronizedClipGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsSynchronizedClipGeneratorVisitor::PClipGenerator(m) => p_clip_generator = Some(m),
                BsSynchronizedClipGeneratorVisitor::SyncAnimPrefix(m) => sync_anim_prefix = Some(m),
                BsSynchronizedClipGeneratorVisitor::BSyncClipIgnoreMarkPlacement(m) => b_sync_clip_ignore_mark_placement = Some(m),
                BsSynchronizedClipGeneratorVisitor::FGetToMarkTime(m) => f_get_to_mark_time = Some(m),
                BsSynchronizedClipGeneratorVisitor::FMarkErrorThreshold(m) => f_mark_error_threshold = Some(m),
                BsSynchronizedClipGeneratorVisitor::BLeadCharacter(m) => b_lead_character = Some(m),
                BsSynchronizedClipGeneratorVisitor::BReorientSupportChar(m) => b_reorient_support_char = Some(m),
                BsSynchronizedClipGeneratorVisitor::BApplyMotionFromRoot(m) => b_apply_motion_from_root = Some(m),
                BsSynchronizedClipGeneratorVisitor::PSyncScene(m) => p_sync_scene = Some(m),
                BsSynchronizedClipGeneratorVisitor::StartMarkWs(m) => start_mark_ws = Some(m),
                BsSynchronizedClipGeneratorVisitor::EndMarkWs(m) => end_mark_ws = Some(m),
                BsSynchronizedClipGeneratorVisitor::StartMarkMs(m) => start_mark_ms = Some(m),
                BsSynchronizedClipGeneratorVisitor::FCurrentLerp(m) => f_current_lerp = Some(m),
                BsSynchronizedClipGeneratorVisitor::PLocalSyncBinding(m) => p_local_sync_binding = Some(m),
                BsSynchronizedClipGeneratorVisitor::PEventMap(m) => p_event_map = Some(m),
                BsSynchronizedClipGeneratorVisitor::SAnimationBindingIndex(m) => s_animation_binding_index = Some(m),
                BsSynchronizedClipGeneratorVisitor::BAtMark(m) => b_at_mark = Some(m),
                BsSynchronizedClipGeneratorVisitor::BAllCharactersInScene(m) => b_all_characters_in_scene = Some(m),
                BsSynchronizedClipGeneratorVisitor::BAllCharactersAtMarks(m) => b_all_characters_at_marks = Some(m),

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
            p_clip_generator: p_clip_generator.unwrap_or_default().into_inner(),
            sync_anim_prefix: sync_anim_prefix.unwrap_or_default().into_inner(),
            b_sync_clip_ignore_mark_placement: b_sync_clip_ignore_mark_placement.unwrap_or_default().into_inner(),
            f_get_to_mark_time: f_get_to_mark_time.unwrap_or_default().into_inner(),
            f_mark_error_threshold: f_mark_error_threshold.unwrap_or_default().into_inner(),
            b_lead_character: b_lead_character.unwrap_or_default().into_inner(),
            b_reorient_support_char: b_reorient_support_char.unwrap_or_default().into_inner(),
            b_apply_motion_from_root: b_apply_motion_from_root.unwrap_or_default().into_inner(),
            p_sync_scene: p_sync_scene.unwrap_or_default().into_inner(),
            start_mark_ws: start_mark_ws.unwrap_or_default().into_inner(),
            end_mark_ws: end_mark_ws.unwrap_or_default().into_inner(),
            start_mark_ms: start_mark_ms.unwrap_or_default().into_inner(),
            f_current_lerp: f_current_lerp.unwrap_or_default().into_inner(),
            p_local_sync_binding: p_local_sync_binding.unwrap_or_default().into_inner(),
            p_event_map: p_event_map.unwrap_or_default().into_inner(),
            s_animation_binding_index: s_animation_binding_index.unwrap_or_default().into_inner(),
            b_at_mark: b_at_mark.unwrap_or_default().into_inner(),
            b_all_characters_in_scene: b_all_characters_in_scene.unwrap_or_default().into_inner(),
            b_all_characters_at_marks: b_all_characters_at_marks.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsSynchronizedClipGenerator<'a>> for Vec<BsSynchronizedClipGeneratorVisitor<'a>> {
    fn from(data: &BsSynchronizedClipGenerator<'a>) -> Self {
        vec![
            BsSynchronizedClipGeneratorVisitor::UserData(data.user_data.into()),
            BsSynchronizedClipGeneratorVisitor::Name(data.name.clone().into()),
            BsSynchronizedClipGeneratorVisitor::Id(data.id.into()),
            BsSynchronizedClipGeneratorVisitor::CloneState(data.clone_state.into()),
            BsSynchronizedClipGeneratorVisitor::PadNode(data.pad_node.clone()),
            BsSynchronizedClipGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsSynchronizedClipGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            BsSynchronizedClipGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsSynchronizedClipGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsSynchronizedClipGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            BsSynchronizedClipGeneratorVisitor::PClipGenerator(data.p_clip_generator.clone().into()),
            BsSynchronizedClipGeneratorVisitor::SyncAnimPrefix(data.sync_anim_prefix.clone().into()),
            BsSynchronizedClipGeneratorVisitor::BSyncClipIgnoreMarkPlacement(data.b_sync_clip_ignore_mark_placement.into()),
            BsSynchronizedClipGeneratorVisitor::FGetToMarkTime(data.f_get_to_mark_time.into()),
            BsSynchronizedClipGeneratorVisitor::FMarkErrorThreshold(data.f_mark_error_threshold.into()),
            BsSynchronizedClipGeneratorVisitor::BLeadCharacter(data.b_lead_character.into()),
            BsSynchronizedClipGeneratorVisitor::BReorientSupportChar(data.b_reorient_support_char.into()),
            BsSynchronizedClipGeneratorVisitor::BApplyMotionFromRoot(data.b_apply_motion_from_root.into()),
            BsSynchronizedClipGeneratorVisitor::PSyncScene(data.p_sync_scene.clone().into()),
            BsSynchronizedClipGeneratorVisitor::StartMarkWs(data.start_mark_ws.clone().into()),
            BsSynchronizedClipGeneratorVisitor::EndMarkWs(data.end_mark_ws.clone().into()),
            BsSynchronizedClipGeneratorVisitor::StartMarkMs(data.start_mark_ms.clone().into()),
            BsSynchronizedClipGeneratorVisitor::FCurrentLerp(data.f_current_lerp.into()),
            BsSynchronizedClipGeneratorVisitor::PLocalSyncBinding(data.p_local_sync_binding.clone().into()),
            BsSynchronizedClipGeneratorVisitor::PEventMap(data.p_event_map.clone().into()),
            BsSynchronizedClipGeneratorVisitor::SAnimationBindingIndex(data.s_animation_binding_index.into()),
            BsSynchronizedClipGeneratorVisitor::BAtMark(data.b_at_mark.into()),
            BsSynchronizedClipGeneratorVisitor::BAllCharactersInScene(data.b_all_characters_in_scene.into()),
            BsSynchronizedClipGeneratorVisitor::BAllCharactersAtMarks(data.b_all_characters_at_marks.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsSynchronizedClipGenerator<'de> {
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
enum BsSynchronizedClipGeneratorVisitor<'a> {
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
    #[serde(rename = "pClipGenerator")]
    PClipGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "SyncAnimPrefix")]
    SyncAnimPrefix(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "bSyncClipIgnoreMarkPlacement")]
    BSyncClipIgnoreMarkPlacement(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "fGetToMarkTime")]
    FGetToMarkTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fMarkErrorThreshold")]
    FMarkErrorThreshold(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bLeadCharacter")]
    BLeadCharacter(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bReorientSupportChar")]
    BReorientSupportChar(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bApplyMotionFromRoot")]
    BApplyMotionFromRoot(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "pSyncScene", skip_serializing)]
    PSyncScene(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "StartMarkWS", skip_serializing)]
    StartMarkWs(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "EndMarkWS", skip_serializing)]
    EndMarkWs(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "StartMarkMS", skip_serializing)]
    StartMarkMs(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "fCurrentLerp", skip_serializing)]
    FCurrentLerp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "pLocalSyncBinding", skip_serializing)]
    PLocalSyncBinding(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pEventMap", skip_serializing)]
    PEventMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sAnimationBindingIndex")]
    SAnimationBindingIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "bAtMark", skip_serializing)]
    BAtMark(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bAllCharactersInScene", skip_serializing)]
    BAllCharactersInScene(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bAllCharactersAtMarks", skip_serializing)]
    BAllCharactersAtMarks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsSynchronizedClipGeneratorVisitor<'de>, "@name",
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
    ("pClipGenerator" => PClipGenerator(Primitive<Cow<'de, str>>)),
    ("SyncAnimPrefix" => SyncAnimPrefix(Primitive<Cow<'de, str>>)),
    ("bSyncClipIgnoreMarkPlacement" => BSyncClipIgnoreMarkPlacement(Primitive<bool>)),
    ("fGetToMarkTime" => FGetToMarkTime(Primitive<f32>)),
    ("fMarkErrorThreshold" => FMarkErrorThreshold(Primitive<f32>)),
    ("bLeadCharacter" => BLeadCharacter(Primitive<bool>)),
    ("bReorientSupportChar" => BReorientSupportChar(Primitive<bool>)),
    ("bApplyMotionFromRoot" => BApplyMotionFromRoot(Primitive<bool>)),
    ("pSyncScene" => PSyncScene(Primitive<Cow<'de, str>>)),
    ("StartMarkWS" => StartMarkWs(Primitive<QsTransform<f32>>)),
    ("EndMarkWS" => EndMarkWs(Primitive<QsTransform<f32>>)),
    ("StartMarkMS" => StartMarkMs(Primitive<QsTransform<f32>>)),
    ("fCurrentLerp" => FCurrentLerp(Primitive<f32>)),
    ("pLocalSyncBinding" => PLocalSyncBinding(Primitive<Cow<'de, str>>)),
    ("pEventMap" => PEventMap(Primitive<Cow<'de, str>>)),
    ("sAnimationBindingIndex" => SAnimationBindingIndex(Primitive<i16>)),
    ("bAtMark" => BAtMark(Primitive<bool>)),
    ("bAllCharactersInScene" => BAllCharactersInScene(Primitive<bool>)),
    ("bAllCharactersAtMarks" => BAllCharactersAtMarks(Primitive<bool>)),
}
