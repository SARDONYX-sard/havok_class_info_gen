//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerModifier`
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

/// `hkbCharacterControllerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xf675d6fb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterControllerModifier<'a> {
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
    /// -   name:`"controlData"`
    /// -   type: `struct hkbCharacterControllerControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub control_data: SingleClass<HkbCharacterControllerControlData>,
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub initial_velocity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocityCoordinates"`
    /// -   type: `enum InitialVelocityCoordinates`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub initial_velocity_coordinates: InitialVelocityCoordinates,
    /// # C++ Class Fields Info
    /// -   name:`"motionMode"`
    /// -   type: `enum MotionMode`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    pub motion_mode: MotionMode,
    /// # C++ Class Fields Info
    /// -   name:`"forceDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    pub force_downward_momentum: bool,
    /// # C++ Class Fields Info
    /// -   name:`"applyGravity"`
    /// -   type: `hkBool`
    /// - offset: 99
    /// -  flags: `FLAGS_NONE`
    pub apply_gravity: bool,
    /// # C++ Class Fields Info
    /// -   name:`"setInitialVelocity"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub set_initial_velocity: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 101
    /// -  flags: `FLAGS_NONE`
    pub is_touching_ground: bool,
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub gravity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub timestep: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_initial_velocity_added: bool,
}

impl Serialize for HkbCharacterControllerModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterControllerModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterControllerModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterControllerModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterControllerModifierVisitor<'a>>> for HkbCharacterControllerModifier<'a> {
    fn from(_values: Vec<HkbCharacterControllerModifierVisitor<'a>>) -> Self {
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
            let mut control_data = None;
            let mut initial_velocity = None;
            let mut initial_velocity_coordinates = None;
            let mut motion_mode = None;
            let mut force_downward_momentum = None;
            let mut apply_gravity = None;
            let mut set_initial_velocity = None;
            let mut is_touching_ground = None;
            let mut gravity = None;
            let mut timestep = None;
            let mut is_initial_velocity_added = None;


        for _value in _values {
            match _value {
                HkbCharacterControllerModifierVisitor::Enable(m) => enable = Some(m),
                HkbCharacterControllerModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbCharacterControllerModifierVisitor::UserData(m) => user_data = Some(m),
                HkbCharacterControllerModifierVisitor::Name(m) => name = Some(m),
                HkbCharacterControllerModifierVisitor::Id(m) => id = Some(m),
                HkbCharacterControllerModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbCharacterControllerModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbCharacterControllerModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbCharacterControllerModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbCharacterControllerModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbCharacterControllerModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterControllerModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterControllerModifierVisitor::ControlData(m) => control_data = Some(m),
                HkbCharacterControllerModifierVisitor::InitialVelocity(m) => initial_velocity = Some(m),
                HkbCharacterControllerModifierVisitor::InitialVelocityCoordinates(m) => initial_velocity_coordinates = Some(m),
                HkbCharacterControllerModifierVisitor::MotionMode(m) => motion_mode = Some(m),
                HkbCharacterControllerModifierVisitor::ForceDownwardMomentum(m) => force_downward_momentum = Some(m),
                HkbCharacterControllerModifierVisitor::ApplyGravity(m) => apply_gravity = Some(m),
                HkbCharacterControllerModifierVisitor::SetInitialVelocity(m) => set_initial_velocity = Some(m),
                HkbCharacterControllerModifierVisitor::IsTouchingGround(m) => is_touching_ground = Some(m),
                HkbCharacterControllerModifierVisitor::Gravity(m) => gravity = Some(m),
                HkbCharacterControllerModifierVisitor::Timestep(m) => timestep = Some(m),
                HkbCharacterControllerModifierVisitor::IsInitialVelocityAdded(m) => is_initial_velocity_added = Some(m),

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
            control_data: control_data.unwrap_or_default(),
            initial_velocity: initial_velocity.unwrap_or_default().into_inner(),
            initial_velocity_coordinates: initial_velocity_coordinates.unwrap_or_default().into_inner(),
            motion_mode: motion_mode.unwrap_or_default().into_inner(),
            force_downward_momentum: force_downward_momentum.unwrap_or_default().into_inner(),
            apply_gravity: apply_gravity.unwrap_or_default().into_inner(),
            set_initial_velocity: set_initial_velocity.unwrap_or_default().into_inner(),
            is_touching_ground: is_touching_ground.unwrap_or_default().into_inner(),
            gravity: gravity.unwrap_or_default().into_inner(),
            timestep: timestep.unwrap_or_default().into_inner(),
            is_initial_velocity_added: is_initial_velocity_added.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterControllerModifier<'a>> for Vec<HkbCharacterControllerModifierVisitor<'a>> {
    fn from(data: &HkbCharacterControllerModifier<'a>) -> Self {
        vec![
            HkbCharacterControllerModifierVisitor::Enable(data.enable.into()),
            HkbCharacterControllerModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbCharacterControllerModifierVisitor::UserData(data.user_data.into()),
            HkbCharacterControllerModifierVisitor::Name(data.name.clone().into()),
            HkbCharacterControllerModifierVisitor::Id(data.id.into()),
            HkbCharacterControllerModifierVisitor::CloneState(data.clone_state.into()),
            HkbCharacterControllerModifierVisitor::PadNode(data.pad_node.clone()),
            HkbCharacterControllerModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbCharacterControllerModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbCharacterControllerModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbCharacterControllerModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterControllerModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterControllerModifierVisitor::ControlData(data.control_data.clone()),
            HkbCharacterControllerModifierVisitor::InitialVelocity(data.initial_velocity.into()),
            HkbCharacterControllerModifierVisitor::InitialVelocityCoordinates(data.initial_velocity_coordinates.clone().into()),
            HkbCharacterControllerModifierVisitor::MotionMode(data.motion_mode.clone().into()),
            HkbCharacterControllerModifierVisitor::ForceDownwardMomentum(data.force_downward_momentum.into()),
            HkbCharacterControllerModifierVisitor::ApplyGravity(data.apply_gravity.into()),
            HkbCharacterControllerModifierVisitor::SetInitialVelocity(data.set_initial_velocity.into()),
            HkbCharacterControllerModifierVisitor::IsTouchingGround(data.is_touching_ground.into()),
            HkbCharacterControllerModifierVisitor::Gravity(data.gravity.into()),
            HkbCharacterControllerModifierVisitor::Timestep(data.timestep.into()),
            HkbCharacterControllerModifierVisitor::IsInitialVelocityAdded(data.is_initial_velocity_added.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterControllerModifier<'de> {
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
enum HkbCharacterControllerModifierVisitor<'a> {
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
    #[serde(rename = "controlData")]
    ControlData(SingleClass<HkbCharacterControllerControlData>),
    /// Visitor fields
    #[serde(rename = "initialVelocity")]
    InitialVelocity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "initialVelocityCoordinates")]
    InitialVelocityCoordinates(Primitive<InitialVelocityCoordinates>),
    /// Visitor fields
    #[serde(rename = "motionMode")]
    MotionMode(Primitive<MotionMode>),
    /// Visitor fields
    #[serde(rename = "forceDownwardMomentum")]
    ForceDownwardMomentum(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "applyGravity")]
    ApplyGravity(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "setInitialVelocity")]
    SetInitialVelocity(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "gravity", skip_serializing)]
    Gravity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isInitialVelocityAdded", skip_serializing)]
    IsInitialVelocityAdded(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifierVisitor<'de>, "@name",
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
    ("controlData" => ControlData(SingleClass<HkbCharacterControllerControlData>)),
    ("initialVelocity" => InitialVelocity(Primitive<Vector4<f32>>)),
    ("initialVelocityCoordinates" => InitialVelocityCoordinates(Primitive<InitialVelocityCoordinates>)),
    ("motionMode" => MotionMode(Primitive<MotionMode>)),
    ("forceDownwardMomentum" => ForceDownwardMomentum(Primitive<bool>)),
    ("applyGravity" => ApplyGravity(Primitive<bool>)),
    ("setInitialVelocity" => SetInitialVelocity(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
    ("gravity" => Gravity(Primitive<Vector4<f32>>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum InitialVelocityCoordinates {
    #[serde(rename = "INITIAL_VELOCITY_IN_WORLD_COORDINATES")]
    #[default]
    InitialVelocityInWorldCoordinates = 0,
    #[serde(rename = "INITIAL_VELOCITY_IN_MODEL_COORDINATES")]
    InitialVelocityInModelCoordinates = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MotionMode {
    #[serde(rename = "MOTION_MODE_FOLLOW_ANIMATION")]
    #[default]
    MotionModeFollowAnimation = 0,
    #[serde(rename = "MOTION_MODE_DYNAMIC")]
    MotionModeDynamic = 1,
}
