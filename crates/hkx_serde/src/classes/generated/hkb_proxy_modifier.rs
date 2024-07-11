//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProxyModifier`
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

/// `hkbProxyModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8a41554f`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProxyModifier<'a> {
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
    /// -   name:`"proxyInfo"`
    /// -   type: `struct hkbProxyModifierProxyInfo`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub proxy_info: SingleClass<HkbProxyModifierProxyInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub linear_velocity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"horizontalGain"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub horizontal_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub vertical_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub max_horizontal_separation: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub max_vertical_separation: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacementError"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub vertical_displacement_error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacementErrorGain"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub vertical_displacement_error_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub max_vertical_displacement: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub min_vertical_displacement: f32,
    /// # C++ Class Fields Info
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub capsule_height: f32,
    /// # C++ Class Fields Info
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub capsule_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSlopeForRotation"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub max_slope_for_rotation: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"phantomType"`
    /// -   type: `enum PhantomType`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub phantom_type: PhantomType,
    /// # C++ Class Fields Info
    /// -   name:`"linearVelocityMode"`
    /// -   type: `enum LinearVelocityMode`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE`
    pub linear_velocity_mode: LinearVelocityMode,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreIncomingRotation"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE`
    pub ignore_incoming_rotation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreCollisionDuringRotation"`
    /// -   type: `hkBool`
    /// - offset: 195
    /// -  flags: `FLAGS_NONE`
    pub ignore_collision_during_rotation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreIncomingTranslation"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    pub ignore_incoming_translation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"includeDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 197
    /// -  flags: `FLAGS_NONE`
    pub include_downward_momentum: bool,
    /// # C++ Class Fields Info
    /// -   name:`"followWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 198
    /// -  flags: `FLAGS_NONE`
    pub follow_world_from_model: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 199
    /// -  flags: `FLAGS_NONE`
    pub is_touching_ground: bool,
    /// # C++ Class Fields Info
    /// -   name:`"characterProxy"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character_proxy: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub phantom: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"phantomShape"`
    /// -   type: `void*`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub phantom_shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"horizontalDisplacement"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub horizontal_displacement: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub vertical_displacement: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub timestep: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousFrameFollowWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub previous_frame_follow_world_from_model: bool,
}

impl Serialize for HkbProxyModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbProxyModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbProxyModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbProxyModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbProxyModifierVisitor<'a>>> for HkbProxyModifier<'a> {
    fn from(_values: Vec<HkbProxyModifierVisitor<'a>>) -> Self {
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
            let mut proxy_info = None;
            let mut linear_velocity = None;
            let mut horizontal_gain = None;
            let mut vertical_gain = None;
            let mut max_horizontal_separation = None;
            let mut max_vertical_separation = None;
            let mut vertical_displacement_error = None;
            let mut vertical_displacement_error_gain = None;
            let mut max_vertical_displacement = None;
            let mut min_vertical_displacement = None;
            let mut capsule_height = None;
            let mut capsule_radius = None;
            let mut max_slope_for_rotation = None;
            let mut collision_filter_info = None;
            let mut phantom_type = None;
            let mut linear_velocity_mode = None;
            let mut ignore_incoming_rotation = None;
            let mut ignore_collision_during_rotation = None;
            let mut ignore_incoming_translation = None;
            let mut include_downward_momentum = None;
            let mut follow_world_from_model = None;
            let mut is_touching_ground = None;
            let mut character_proxy = None;
            let mut phantom = None;
            let mut phantom_shape = None;
            let mut horizontal_displacement = None;
            let mut vertical_displacement = None;
            let mut timestep = None;
            let mut previous_frame_follow_world_from_model = None;


        for _value in _values {
            match _value {
                HkbProxyModifierVisitor::Enable(m) => enable = Some(m),
                HkbProxyModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbProxyModifierVisitor::UserData(m) => user_data = Some(m),
                HkbProxyModifierVisitor::Name(m) => name = Some(m),
                HkbProxyModifierVisitor::Id(m) => id = Some(m),
                HkbProxyModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbProxyModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbProxyModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbProxyModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbProxyModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbProxyModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbProxyModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbProxyModifierVisitor::ProxyInfo(m) => proxy_info = Some(m),
                HkbProxyModifierVisitor::LinearVelocity(m) => linear_velocity = Some(m),
                HkbProxyModifierVisitor::HorizontalGain(m) => horizontal_gain = Some(m),
                HkbProxyModifierVisitor::VerticalGain(m) => vertical_gain = Some(m),
                HkbProxyModifierVisitor::MaxHorizontalSeparation(m) => max_horizontal_separation = Some(m),
                HkbProxyModifierVisitor::MaxVerticalSeparation(m) => max_vertical_separation = Some(m),
                HkbProxyModifierVisitor::VerticalDisplacementError(m) => vertical_displacement_error = Some(m),
                HkbProxyModifierVisitor::VerticalDisplacementErrorGain(m) => vertical_displacement_error_gain = Some(m),
                HkbProxyModifierVisitor::MaxVerticalDisplacement(m) => max_vertical_displacement = Some(m),
                HkbProxyModifierVisitor::MinVerticalDisplacement(m) => min_vertical_displacement = Some(m),
                HkbProxyModifierVisitor::CapsuleHeight(m) => capsule_height = Some(m),
                HkbProxyModifierVisitor::CapsuleRadius(m) => capsule_radius = Some(m),
                HkbProxyModifierVisitor::MaxSlopeForRotation(m) => max_slope_for_rotation = Some(m),
                HkbProxyModifierVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbProxyModifierVisitor::PhantomType(m) => phantom_type = Some(m),
                HkbProxyModifierVisitor::LinearVelocityMode(m) => linear_velocity_mode = Some(m),
                HkbProxyModifierVisitor::IgnoreIncomingRotation(m) => ignore_incoming_rotation = Some(m),
                HkbProxyModifierVisitor::IgnoreCollisionDuringRotation(m) => ignore_collision_during_rotation = Some(m),
                HkbProxyModifierVisitor::IgnoreIncomingTranslation(m) => ignore_incoming_translation = Some(m),
                HkbProxyModifierVisitor::IncludeDownwardMomentum(m) => include_downward_momentum = Some(m),
                HkbProxyModifierVisitor::FollowWorldFromModel(m) => follow_world_from_model = Some(m),
                HkbProxyModifierVisitor::IsTouchingGround(m) => is_touching_ground = Some(m),
                HkbProxyModifierVisitor::CharacterProxy(m) => character_proxy = Some(m),
                HkbProxyModifierVisitor::Phantom(m) => phantom = Some(m),
                HkbProxyModifierVisitor::PhantomShape(m) => phantom_shape = Some(m),
                HkbProxyModifierVisitor::HorizontalDisplacement(m) => horizontal_displacement = Some(m),
                HkbProxyModifierVisitor::VerticalDisplacement(m) => vertical_displacement = Some(m),
                HkbProxyModifierVisitor::Timestep(m) => timestep = Some(m),
                HkbProxyModifierVisitor::PreviousFrameFollowWorldFromModel(m) => previous_frame_follow_world_from_model = Some(m),

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
            proxy_info: proxy_info.unwrap_or_default(),
            linear_velocity: linear_velocity.unwrap_or_default().into_inner(),
            horizontal_gain: horizontal_gain.unwrap_or_default().into_inner(),
            vertical_gain: vertical_gain.unwrap_or_default().into_inner(),
            max_horizontal_separation: max_horizontal_separation.unwrap_or_default().into_inner(),
            max_vertical_separation: max_vertical_separation.unwrap_or_default().into_inner(),
            vertical_displacement_error: vertical_displacement_error.unwrap_or_default().into_inner(),
            vertical_displacement_error_gain: vertical_displacement_error_gain.unwrap_or_default().into_inner(),
            max_vertical_displacement: max_vertical_displacement.unwrap_or_default().into_inner(),
            min_vertical_displacement: min_vertical_displacement.unwrap_or_default().into_inner(),
            capsule_height: capsule_height.unwrap_or_default().into_inner(),
            capsule_radius: capsule_radius.unwrap_or_default().into_inner(),
            max_slope_for_rotation: max_slope_for_rotation.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            phantom_type: phantom_type.unwrap_or_default().into_inner(),
            linear_velocity_mode: linear_velocity_mode.unwrap_or_default().into_inner(),
            ignore_incoming_rotation: ignore_incoming_rotation.unwrap_or_default().into_inner(),
            ignore_collision_during_rotation: ignore_collision_during_rotation.unwrap_or_default().into_inner(),
            ignore_incoming_translation: ignore_incoming_translation.unwrap_or_default().into_inner(),
            include_downward_momentum: include_downward_momentum.unwrap_or_default().into_inner(),
            follow_world_from_model: follow_world_from_model.unwrap_or_default().into_inner(),
            is_touching_ground: is_touching_ground.unwrap_or_default().into_inner(),
            character_proxy: character_proxy.unwrap_or_default().into_inner(),
            phantom: phantom.unwrap_or_default().into_inner(),
            phantom_shape: phantom_shape.unwrap_or_default().into_inner(),
            horizontal_displacement: horizontal_displacement.unwrap_or_default().into_inner(),
            vertical_displacement: vertical_displacement.unwrap_or_default().into_inner(),
            timestep: timestep.unwrap_or_default().into_inner(),
            previous_frame_follow_world_from_model: previous_frame_follow_world_from_model.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbProxyModifier<'a>> for Vec<HkbProxyModifierVisitor<'a>> {
    fn from(data: &HkbProxyModifier<'a>) -> Self {
        vec![
            HkbProxyModifierVisitor::Enable(data.enable.into()),
            HkbProxyModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbProxyModifierVisitor::UserData(data.user_data.into()),
            HkbProxyModifierVisitor::Name(data.name.clone().into()),
            HkbProxyModifierVisitor::Id(data.id.into()),
            HkbProxyModifierVisitor::CloneState(data.clone_state.into()),
            HkbProxyModifierVisitor::PadNode(data.pad_node.clone()),
            HkbProxyModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbProxyModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbProxyModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbProxyModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbProxyModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbProxyModifierVisitor::ProxyInfo(data.proxy_info.clone()),
            HkbProxyModifierVisitor::LinearVelocity(data.linear_velocity.into()),
            HkbProxyModifierVisitor::HorizontalGain(data.horizontal_gain.into()),
            HkbProxyModifierVisitor::VerticalGain(data.vertical_gain.into()),
            HkbProxyModifierVisitor::MaxHorizontalSeparation(data.max_horizontal_separation.into()),
            HkbProxyModifierVisitor::MaxVerticalSeparation(data.max_vertical_separation.into()),
            HkbProxyModifierVisitor::VerticalDisplacementError(data.vertical_displacement_error.into()),
            HkbProxyModifierVisitor::VerticalDisplacementErrorGain(data.vertical_displacement_error_gain.into()),
            HkbProxyModifierVisitor::MaxVerticalDisplacement(data.max_vertical_displacement.into()),
            HkbProxyModifierVisitor::MinVerticalDisplacement(data.min_vertical_displacement.into()),
            HkbProxyModifierVisitor::CapsuleHeight(data.capsule_height.into()),
            HkbProxyModifierVisitor::CapsuleRadius(data.capsule_radius.into()),
            HkbProxyModifierVisitor::MaxSlopeForRotation(data.max_slope_for_rotation.into()),
            HkbProxyModifierVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbProxyModifierVisitor::PhantomType(data.phantom_type.clone().into()),
            HkbProxyModifierVisitor::LinearVelocityMode(data.linear_velocity_mode.clone().into()),
            HkbProxyModifierVisitor::IgnoreIncomingRotation(data.ignore_incoming_rotation.into()),
            HkbProxyModifierVisitor::IgnoreCollisionDuringRotation(data.ignore_collision_during_rotation.into()),
            HkbProxyModifierVisitor::IgnoreIncomingTranslation(data.ignore_incoming_translation.into()),
            HkbProxyModifierVisitor::IncludeDownwardMomentum(data.include_downward_momentum.into()),
            HkbProxyModifierVisitor::FollowWorldFromModel(data.follow_world_from_model.into()),
            HkbProxyModifierVisitor::IsTouchingGround(data.is_touching_ground.into()),
            HkbProxyModifierVisitor::CharacterProxy(data.character_proxy.clone().into()),
            HkbProxyModifierVisitor::Phantom(data.phantom.clone().into()),
            HkbProxyModifierVisitor::PhantomShape(data.phantom_shape.clone().into()),
            HkbProxyModifierVisitor::HorizontalDisplacement(data.horizontal_displacement.into()),
            HkbProxyModifierVisitor::VerticalDisplacement(data.vertical_displacement.into()),
            HkbProxyModifierVisitor::Timestep(data.timestep.into()),
            HkbProxyModifierVisitor::PreviousFrameFollowWorldFromModel(data.previous_frame_follow_world_from_model.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbProxyModifier<'de> {
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
enum HkbProxyModifierVisitor<'a> {
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
    #[serde(rename = "proxyInfo")]
    ProxyInfo(SingleClass<HkbProxyModifierProxyInfo>),
    /// Visitor fields
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "horizontalGain")]
    HorizontalGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "verticalGain")]
    VerticalGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "verticalDisplacementError")]
    VerticalDisplacementError(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "verticalDisplacementErrorGain")]
    VerticalDisplacementErrorGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxVerticalDisplacement")]
    MaxVerticalDisplacement(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minVerticalDisplacement")]
    MinVerticalDisplacement(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxSlopeForRotation")]
    MaxSlopeForRotation(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "phantomType")]
    PhantomType(Primitive<PhantomType>),
    /// Visitor fields
    #[serde(rename = "linearVelocityMode")]
    LinearVelocityMode(Primitive<LinearVelocityMode>),
    /// Visitor fields
    #[serde(rename = "ignoreIncomingRotation")]
    IgnoreIncomingRotation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "ignoreCollisionDuringRotation")]
    IgnoreCollisionDuringRotation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "ignoreIncomingTranslation")]
    IgnoreIncomingTranslation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "includeDownwardMomentum")]
    IncludeDownwardMomentum(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "followWorldFromModel")]
    FollowWorldFromModel(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "characterProxy", skip_serializing)]
    CharacterProxy(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantom", skip_serializing)]
    Phantom(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantomShape", skip_serializing)]
    PhantomShape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "horizontalDisplacement", skip_serializing)]
    HorizontalDisplacement(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "verticalDisplacement", skip_serializing)]
    VerticalDisplacement(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousFrameFollowWorldFromModel", skip_serializing)]
    PreviousFrameFollowWorldFromModel(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifierVisitor<'de>, "@name",
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
    ("proxyInfo" => ProxyInfo(SingleClass<HkbProxyModifierProxyInfo>)),
    ("linearVelocity" => LinearVelocity(Primitive<Vector4<f32>>)),
    ("horizontalGain" => HorizontalGain(Primitive<f32>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("verticalDisplacementError" => VerticalDisplacementError(Primitive<f32>)),
    ("verticalDisplacementErrorGain" => VerticalDisplacementErrorGain(Primitive<f32>)),
    ("maxVerticalDisplacement" => MaxVerticalDisplacement(Primitive<f32>)),
    ("minVerticalDisplacement" => MinVerticalDisplacement(Primitive<f32>)),
    ("capsuleHeight" => CapsuleHeight(Primitive<f32>)),
    ("capsuleRadius" => CapsuleRadius(Primitive<f32>)),
    ("maxSlopeForRotation" => MaxSlopeForRotation(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("phantomType" => PhantomType(Primitive<PhantomType>)),
    ("linearVelocityMode" => LinearVelocityMode(Primitive<LinearVelocityMode>)),
    ("ignoreIncomingRotation" => IgnoreIncomingRotation(Primitive<bool>)),
    ("ignoreCollisionDuringRotation" => IgnoreCollisionDuringRotation(Primitive<bool>)),
    ("ignoreIncomingTranslation" => IgnoreIncomingTranslation(Primitive<bool>)),
    ("includeDownwardMomentum" => IncludeDownwardMomentum(Primitive<bool>)),
    ("followWorldFromModel" => FollowWorldFromModel(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
    ("characterProxy" => CharacterProxy(Primitive<Cow<'de, str>>)),
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("phantomShape" => PhantomShape(Primitive<Cow<'de, str>>)),
    ("horizontalDisplacement" => HorizontalDisplacement(Primitive<Vector4<f32>>)),
    ("verticalDisplacement" => VerticalDisplacement(Primitive<f32>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("previousFrameFollowWorldFromModel" => PreviousFrameFollowWorldFromModel(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum PhantomType {
    #[serde(rename = "PHANTOM_TYPE_SIMPLE")]
    #[default]
    PhantomTypeSimple = 0,
    #[serde(rename = "PHANTOM_TYPE_CACHING")]
    PhantomTypeCaching = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum LinearVelocityMode {
    #[serde(rename = "LINEAR_VELOCITY_MODE_WORLD")]
    #[default]
    LinearVelocityModeWorld = 0,
    #[serde(rename = "LINEAR_VELOCITY_MODE_MODEL")]
    LinearVelocityModeModel = 1,
}
