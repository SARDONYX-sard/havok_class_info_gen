//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGetHandleOnBoneModifier`
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

/// `hkbGetHandleOnBoneModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x50c34a17`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbGetHandleOnBoneModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_modifier: CStyleArray<[bool; 3]>,

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
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub handle_out: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub local_frame_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"ragdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub ragdoll_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    pub animation_bone_index: i16,
}

impl Serialize for HkbGetHandleOnBoneModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbGetHandleOnBoneModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbGetHandleOnBoneModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbGetHandleOnBoneModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbGetHandleOnBoneModifierVisitor<'a>>> for HkbGetHandleOnBoneModifier<'a> {
    fn from(_values: Vec<HkbGetHandleOnBoneModifierVisitor<'a>>) -> Self {
            let mut enable = None;
            let mut pad_modifier = None;
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
            let mut handle_out = None;
            let mut local_frame_name = None;
            let mut ragdoll_bone_index = None;
            let mut animation_bone_index = None;


        for _value in _values {
            match _value {
                HkbGetHandleOnBoneModifierVisitor::Enable(m) => enable = Some(m),
                HkbGetHandleOnBoneModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbGetHandleOnBoneModifierVisitor::UserData(m) => user_data = Some(m),
                HkbGetHandleOnBoneModifierVisitor::Name(m) => name = Some(m),
                HkbGetHandleOnBoneModifierVisitor::Id(m) => id = Some(m),
                HkbGetHandleOnBoneModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbGetHandleOnBoneModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbGetHandleOnBoneModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbGetHandleOnBoneModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbGetHandleOnBoneModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbGetHandleOnBoneModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbGetHandleOnBoneModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbGetHandleOnBoneModifierVisitor::HandleOut(m) => handle_out = Some(m),
                HkbGetHandleOnBoneModifierVisitor::LocalFrameName(m) => local_frame_name = Some(m),
                HkbGetHandleOnBoneModifierVisitor::RagdollBoneIndex(m) => ragdoll_bone_index = Some(m),
                HkbGetHandleOnBoneModifierVisitor::AnimationBoneIndex(m) => animation_bone_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enable: enable.unwrap_or_default().into_inner(),
            pad_modifier: pad_modifier.unwrap_or_default(),
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
            handle_out: handle_out.unwrap_or_default().into_inner(),
            local_frame_name: local_frame_name.unwrap_or_default().into_inner(),
            ragdoll_bone_index: ragdoll_bone_index.unwrap_or_default().into_inner(),
            animation_bone_index: animation_bone_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbGetHandleOnBoneModifier<'a>> for Vec<HkbGetHandleOnBoneModifierVisitor<'a>> {
    fn from(data: &HkbGetHandleOnBoneModifier<'a>) -> Self {
        vec![
            HkbGetHandleOnBoneModifierVisitor::Enable(data.enable.into()),
            HkbGetHandleOnBoneModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbGetHandleOnBoneModifierVisitor::UserData(data.user_data.into()),
            HkbGetHandleOnBoneModifierVisitor::Name(data.name.clone().into()),
            HkbGetHandleOnBoneModifierVisitor::Id(data.id.into()),
            HkbGetHandleOnBoneModifierVisitor::CloneState(data.clone_state.into()),
            HkbGetHandleOnBoneModifierVisitor::PadNode(data.pad_node.clone()),
            HkbGetHandleOnBoneModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbGetHandleOnBoneModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbGetHandleOnBoneModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbGetHandleOnBoneModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbGetHandleOnBoneModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbGetHandleOnBoneModifierVisitor::HandleOut(data.handle_out.clone().into()),
            HkbGetHandleOnBoneModifierVisitor::LocalFrameName(data.local_frame_name.clone().into()),
            HkbGetHandleOnBoneModifierVisitor::RagdollBoneIndex(data.ragdoll_bone_index.into()),
            HkbGetHandleOnBoneModifierVisitor::AnimationBoneIndex(data.animation_bone_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbGetHandleOnBoneModifier<'de> {
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
enum HkbGetHandleOnBoneModifierVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

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
    #[serde(rename = "handleOut")]
    HandleOut(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "ragdollBoneIndex")]
    RagdollBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetHandleOnBoneModifierVisitor<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
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
    ("handleOut" => HandleOut(Primitive<Cow<'de, str>>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
    ("ragdollBoneIndex" => RagdollBoneIndex(Primitive<i16>)),
    ("animationBoneIndex" => AnimationBoneIndex(Primitive<i16>)),
}
