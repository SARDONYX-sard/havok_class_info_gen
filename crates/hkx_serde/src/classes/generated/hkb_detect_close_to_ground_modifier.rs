//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDetectCloseToGroundModifier`
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

/// `hkbDetectCloseToGroundModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x981687b2`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbDetectCloseToGroundModifier<'a> {
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
    /// -   name:`"closeToGroundEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub close_to_ground_event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"closeToGroundHeight"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub close_to_ground_height: f32,
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub raycast_distance_down: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"animBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 66
    /// -  flags: `FLAGS_NONE`
    pub anim_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"isCloseToGround"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_close_to_ground: bool,
}

impl Serialize for HkbDetectCloseToGroundModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbDetectCloseToGroundModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbDetectCloseToGroundModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbDetectCloseToGroundModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbDetectCloseToGroundModifierVisitor<'a>>> for HkbDetectCloseToGroundModifier<'a> {
    fn from(_values: Vec<HkbDetectCloseToGroundModifierVisitor<'a>>) -> Self {
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
            let mut close_to_ground_event = None;
            let mut close_to_ground_height = None;
            let mut raycast_distance_down = None;
            let mut collision_filter_info = None;
            let mut bone_index = None;
            let mut anim_bone_index = None;
            let mut is_close_to_ground = None;


        for _value in _values {
            match _value {
                HkbDetectCloseToGroundModifierVisitor::Enable(m) => enable = Some(m),
                HkbDetectCloseToGroundModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbDetectCloseToGroundModifierVisitor::UserData(m) => user_data = Some(m),
                HkbDetectCloseToGroundModifierVisitor::Name(m) => name = Some(m),
                HkbDetectCloseToGroundModifierVisitor::Id(m) => id = Some(m),
                HkbDetectCloseToGroundModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbDetectCloseToGroundModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbDetectCloseToGroundModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbDetectCloseToGroundModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbDetectCloseToGroundModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbDetectCloseToGroundModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbDetectCloseToGroundModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbDetectCloseToGroundModifierVisitor::CloseToGroundEvent(m) => close_to_ground_event = Some(m),
                HkbDetectCloseToGroundModifierVisitor::CloseToGroundHeight(m) => close_to_ground_height = Some(m),
                HkbDetectCloseToGroundModifierVisitor::RaycastDistanceDown(m) => raycast_distance_down = Some(m),
                HkbDetectCloseToGroundModifierVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbDetectCloseToGroundModifierVisitor::BoneIndex(m) => bone_index = Some(m),
                HkbDetectCloseToGroundModifierVisitor::AnimBoneIndex(m) => anim_bone_index = Some(m),
                HkbDetectCloseToGroundModifierVisitor::IsCloseToGround(m) => is_close_to_ground = Some(m),

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
            close_to_ground_event: close_to_ground_event.unwrap_or_default(),
            close_to_ground_height: close_to_ground_height.unwrap_or_default().into_inner(),
            raycast_distance_down: raycast_distance_down.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            bone_index: bone_index.unwrap_or_default().into_inner(),
            anim_bone_index: anim_bone_index.unwrap_or_default().into_inner(),
            is_close_to_ground: is_close_to_ground.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbDetectCloseToGroundModifier<'a>> for Vec<HkbDetectCloseToGroundModifierVisitor<'a>> {
    fn from(data: &HkbDetectCloseToGroundModifier<'a>) -> Self {
        vec![
            HkbDetectCloseToGroundModifierVisitor::Enable(data.enable.into()),
            HkbDetectCloseToGroundModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbDetectCloseToGroundModifierVisitor::UserData(data.user_data.into()),
            HkbDetectCloseToGroundModifierVisitor::Name(data.name.clone().into()),
            HkbDetectCloseToGroundModifierVisitor::Id(data.id.into()),
            HkbDetectCloseToGroundModifierVisitor::CloneState(data.clone_state.into()),
            HkbDetectCloseToGroundModifierVisitor::PadNode(data.pad_node.clone()),
            HkbDetectCloseToGroundModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbDetectCloseToGroundModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbDetectCloseToGroundModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbDetectCloseToGroundModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbDetectCloseToGroundModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbDetectCloseToGroundModifierVisitor::CloseToGroundEvent(data.close_to_ground_event.clone()),
            HkbDetectCloseToGroundModifierVisitor::CloseToGroundHeight(data.close_to_ground_height.into()),
            HkbDetectCloseToGroundModifierVisitor::RaycastDistanceDown(data.raycast_distance_down.into()),
            HkbDetectCloseToGroundModifierVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbDetectCloseToGroundModifierVisitor::BoneIndex(data.bone_index.into()),
            HkbDetectCloseToGroundModifierVisitor::AnimBoneIndex(data.anim_bone_index.into()),
            HkbDetectCloseToGroundModifierVisitor::IsCloseToGround(data.is_close_to_ground.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbDetectCloseToGroundModifier<'de> {
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
enum HkbDetectCloseToGroundModifierVisitor<'a> {
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
    #[serde(rename = "closeToGroundEvent")]
    CloseToGroundEvent(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "closeToGroundHeight")]
    CloseToGroundHeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "animBoneIndex")]
    AnimBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "isCloseToGround", skip_serializing)]
    IsCloseToGround(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDetectCloseToGroundModifierVisitor<'de>, "@name",
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
    ("closeToGroundEvent" => CloseToGroundEvent(SingleClass<HkbEventProperty<'de>>)),
    ("closeToGroundHeight" => CloseToGroundHeight(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("animBoneIndex" => AnimBoneIndex(Primitive<i16>)),
    ("isCloseToGround" => IsCloseToGround(Primitive<bool>)),
}
