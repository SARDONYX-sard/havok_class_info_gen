//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkModifier`
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

/// `hkbFootIkModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xed8966c0`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkModifier<'a> {
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
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub gains: SingleClass<HkbFootIkGains>,
    /// # C++ Class Fields Info
    /// -   name:`"legs"`
    /// -   type: `hkArray<struct hkbFootIkModifierLeg>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub legs: HkArrayClass<HkbFootIkModifierLeg<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub raycast_distance_up: f32,
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub raycast_distance_down: f32,
    /// # C++ Class Fields Info
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub original_ground_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"errorOut"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub error_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub error_out_translation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub align_with_ground_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub vertical_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub forward_align_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub sideways_align_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub sideways_sample_width: f32,
    /// # C++ Class Fields Info
    /// -   name:`"useTrackData"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub use_track_data: bool,
    /// # C++ Class Fields Info
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    pub lock_feet_when_planted: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 182
    /// -  flags: `FLAGS_NONE`
    pub use_character_up_vector: bool,
    /// # C++ Class Fields Info
    /// -   name:`"alignMode"`
    /// -   type: `enum AlignMode`
    /// - offset: 183
    /// -  flags: `FLAGS_NONE`
    pub align_mode: AlignMode,
    /// # C++ Class Fields Info
    /// -   name:`"internalLegData"`
    /// -   type: `hkArray<struct hkbFootIkModifierInternalLegData>`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub internal_leg_data: HkArrayClass<HkbFootIkModifierInternalLegData<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"prevIsFootIkEnabled"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub prev_is_foot_ik_enabled: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isSetUp"`
    /// -   type: `hkBool`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_set_up: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isGroundPositionValid"`
    /// -   type: `hkBool`
    /// - offset: 201
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_ground_position_valid: bool,
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_step: f32,
}

impl Serialize for HkbFootIkModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbFootIkModifierVisitor<'a>>> for HkbFootIkModifier<'a> {
    fn from(_values: Vec<HkbFootIkModifierVisitor<'a>>) -> Self {
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
            let mut gains = None;
            let mut legs = None;
            let mut raycast_distance_up = None;
            let mut raycast_distance_down = None;
            let mut original_ground_height_ms = None;
            let mut error_out = None;
            let mut error_out_translation = None;
            let mut align_with_ground_rotation = None;
            let mut vertical_offset = None;
            let mut collision_filter_info = None;
            let mut forward_align_fraction = None;
            let mut sideways_align_fraction = None;
            let mut sideways_sample_width = None;
            let mut use_track_data = None;
            let mut lock_feet_when_planted = None;
            let mut use_character_up_vector = None;
            let mut align_mode = None;
            let mut internal_leg_data = None;
            let mut prev_is_foot_ik_enabled = None;
            let mut is_set_up = None;
            let mut is_ground_position_valid = None;
            let mut time_step = None;


        for _value in _values {
            match _value {
                HkbFootIkModifierVisitor::Enable(m) => enable = Some(m),
                HkbFootIkModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbFootIkModifierVisitor::UserData(m) => user_data = Some(m),
                HkbFootIkModifierVisitor::Name(m) => name = Some(m),
                HkbFootIkModifierVisitor::Id(m) => id = Some(m),
                HkbFootIkModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbFootIkModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbFootIkModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbFootIkModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbFootIkModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbFootIkModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbFootIkModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbFootIkModifierVisitor::Gains(m) => gains = Some(m),
                HkbFootIkModifierVisitor::Legs(m) => legs = Some(m),
                HkbFootIkModifierVisitor::RaycastDistanceUp(m) => raycast_distance_up = Some(m),
                HkbFootIkModifierVisitor::RaycastDistanceDown(m) => raycast_distance_down = Some(m),
                HkbFootIkModifierVisitor::OriginalGroundHeightMs(m) => original_ground_height_ms = Some(m),
                HkbFootIkModifierVisitor::ErrorOut(m) => error_out = Some(m),
                HkbFootIkModifierVisitor::ErrorOutTranslation(m) => error_out_translation = Some(m),
                HkbFootIkModifierVisitor::AlignWithGroundRotation(m) => align_with_ground_rotation = Some(m),
                HkbFootIkModifierVisitor::VerticalOffset(m) => vertical_offset = Some(m),
                HkbFootIkModifierVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbFootIkModifierVisitor::ForwardAlignFraction(m) => forward_align_fraction = Some(m),
                HkbFootIkModifierVisitor::SidewaysAlignFraction(m) => sideways_align_fraction = Some(m),
                HkbFootIkModifierVisitor::SidewaysSampleWidth(m) => sideways_sample_width = Some(m),
                HkbFootIkModifierVisitor::UseTrackData(m) => use_track_data = Some(m),
                HkbFootIkModifierVisitor::LockFeetWhenPlanted(m) => lock_feet_when_planted = Some(m),
                HkbFootIkModifierVisitor::UseCharacterUpVector(m) => use_character_up_vector = Some(m),
                HkbFootIkModifierVisitor::AlignMode(m) => align_mode = Some(m),
                HkbFootIkModifierVisitor::InternalLegData(m) => internal_leg_data = Some(m),
                HkbFootIkModifierVisitor::PrevIsFootIkEnabled(m) => prev_is_foot_ik_enabled = Some(m),
                HkbFootIkModifierVisitor::IsSetUp(m) => is_set_up = Some(m),
                HkbFootIkModifierVisitor::IsGroundPositionValid(m) => is_ground_position_valid = Some(m),
                HkbFootIkModifierVisitor::TimeStep(m) => time_step = Some(m),

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
            gains: gains.unwrap_or_default(),
            legs: legs.unwrap_or_default(),
            raycast_distance_up: raycast_distance_up.unwrap_or_default().into_inner(),
            raycast_distance_down: raycast_distance_down.unwrap_or_default().into_inner(),
            original_ground_height_ms: original_ground_height_ms.unwrap_or_default().into_inner(),
            error_out: error_out.unwrap_or_default().into_inner(),
            error_out_translation: error_out_translation.unwrap_or_default().into_inner(),
            align_with_ground_rotation: align_with_ground_rotation.unwrap_or_default().into_inner(),
            vertical_offset: vertical_offset.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            forward_align_fraction: forward_align_fraction.unwrap_or_default().into_inner(),
            sideways_align_fraction: sideways_align_fraction.unwrap_or_default().into_inner(),
            sideways_sample_width: sideways_sample_width.unwrap_or_default().into_inner(),
            use_track_data: use_track_data.unwrap_or_default().into_inner(),
            lock_feet_when_planted: lock_feet_when_planted.unwrap_or_default().into_inner(),
            use_character_up_vector: use_character_up_vector.unwrap_or_default().into_inner(),
            align_mode: align_mode.unwrap_or_default().into_inner(),
            internal_leg_data: internal_leg_data.unwrap_or_default(),
            prev_is_foot_ik_enabled: prev_is_foot_ik_enabled.unwrap_or_default().into_inner(),
            is_set_up: is_set_up.unwrap_or_default().into_inner(),
            is_ground_position_valid: is_ground_position_valid.unwrap_or_default().into_inner(),
            time_step: time_step.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbFootIkModifier<'a>> for Vec<HkbFootIkModifierVisitor<'a>> {
    fn from(data: &HkbFootIkModifier<'a>) -> Self {
        vec![
            HkbFootIkModifierVisitor::Enable(data.enable.into()),
            HkbFootIkModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbFootIkModifierVisitor::UserData(data.user_data.into()),
            HkbFootIkModifierVisitor::Name(data.name.clone().into()),
            HkbFootIkModifierVisitor::Id(data.id.into()),
            HkbFootIkModifierVisitor::CloneState(data.clone_state.into()),
            HkbFootIkModifierVisitor::PadNode(data.pad_node.clone()),
            HkbFootIkModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbFootIkModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbFootIkModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbFootIkModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbFootIkModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbFootIkModifierVisitor::Gains(data.gains.clone()),
            HkbFootIkModifierVisitor::Legs(data.legs.clone()),
            HkbFootIkModifierVisitor::RaycastDistanceUp(data.raycast_distance_up.into()),
            HkbFootIkModifierVisitor::RaycastDistanceDown(data.raycast_distance_down.into()),
            HkbFootIkModifierVisitor::OriginalGroundHeightMs(data.original_ground_height_ms.into()),
            HkbFootIkModifierVisitor::ErrorOut(data.error_out.into()),
            HkbFootIkModifierVisitor::ErrorOutTranslation(data.error_out_translation.into()),
            HkbFootIkModifierVisitor::AlignWithGroundRotation(data.align_with_ground_rotation.clone().into()),
            HkbFootIkModifierVisitor::VerticalOffset(data.vertical_offset.into()),
            HkbFootIkModifierVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbFootIkModifierVisitor::ForwardAlignFraction(data.forward_align_fraction.into()),
            HkbFootIkModifierVisitor::SidewaysAlignFraction(data.sideways_align_fraction.into()),
            HkbFootIkModifierVisitor::SidewaysSampleWidth(data.sideways_sample_width.into()),
            HkbFootIkModifierVisitor::UseTrackData(data.use_track_data.into()),
            HkbFootIkModifierVisitor::LockFeetWhenPlanted(data.lock_feet_when_planted.into()),
            HkbFootIkModifierVisitor::UseCharacterUpVector(data.use_character_up_vector.into()),
            HkbFootIkModifierVisitor::AlignMode(data.align_mode.clone().into()),
            HkbFootIkModifierVisitor::InternalLegData(data.internal_leg_data.clone()),
            HkbFootIkModifierVisitor::PrevIsFootIkEnabled(data.prev_is_foot_ik_enabled.into()),
            HkbFootIkModifierVisitor::IsSetUp(data.is_set_up.into()),
            HkbFootIkModifierVisitor::IsGroundPositionValid(data.is_ground_position_valid.into()),
            HkbFootIkModifierVisitor::TimeStep(data.time_step.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkModifier<'de> {
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
enum HkbFootIkModifierVisitor<'a> {
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
    #[serde(rename = "gains")]
    Gains(SingleClass<HkbFootIkGains>),
    /// Visitor fields
    #[serde(rename = "legs")]
    Legs(HkArrayClass<HkbFootIkModifierLeg<'a>>),
    /// Visitor fields
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "errorOut")]
    ErrorOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "useTrackData")]
    UseTrackData(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "alignMode")]
    AlignMode(Primitive<AlignMode>),
    /// Visitor fields
    #[serde(rename = "internalLegData", skip_serializing)]
    InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData<'a>>),
    /// Visitor fields
    #[serde(rename = "prevIsFootIkEnabled", skip_serializing)]
    PrevIsFootIkEnabled(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isSetUp", skip_serializing)]
    IsSetUp(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isGroundPositionValid", skip_serializing)]
    IsGroundPositionValid(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierVisitor<'de>, "@name",
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
    ("gains" => Gains(SingleClass<HkbFootIkGains>)),
    ("legs" => Legs(HkArrayClass<HkbFootIkModifierLeg<'de>>)),
    ("raycastDistanceUp" => RaycastDistanceUp(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(Primitive<f32>)),
    ("errorOut" => ErrorOut(Primitive<f32>)),
    ("errorOutTranslation" => ErrorOutTranslation(Primitive<Vector4<f32>>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Primitive<Quaternion<f32>>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("forwardAlignFraction" => ForwardAlignFraction(Primitive<f32>)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(Primitive<f32>)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(Primitive<f32>)),
    ("useTrackData" => UseTrackData(Primitive<bool>)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(Primitive<bool>)),
    ("useCharacterUpVector" => UseCharacterUpVector(Primitive<bool>)),
    ("alignMode" => AlignMode(Primitive<AlignMode>)),
    ("internalLegData" => InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData<'de>>)),
    ("prevIsFootIkEnabled" => PrevIsFootIkEnabled(Primitive<f32>)),
    ("isSetUp" => IsSetUp(Primitive<bool>)),
    ("isGroundPositionValid" => IsGroundPositionValid(Primitive<bool>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AlignMode {
    #[serde(rename = "ALIGN_MODE_FORWARD_RIGHT")]
    #[default]
    AlignModeForwardRight = 0,
    #[serde(rename = "ALIGN_MODE_FORWARD")]
    AlignModeForward = 1,
}
