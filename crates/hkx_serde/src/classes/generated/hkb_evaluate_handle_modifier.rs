//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvaluateHandleModifier`
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

/// `hkbEvaluateHandleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x79757102`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEvaluateHandleModifier<'a> {
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
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub handle: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"handlePositionOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub handle_position_out: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"handleRotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub handle_rotation_out: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"isValidOut"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub is_valid_out: bool,
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub extrapolation_time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub handle_change_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub handle_change_mode: HandleChangeMode,
    /// # C++ Class Fields Info
    /// -   name:`"oldHandle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub old_handle: SingleClass<HkbHandle<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"oldHandlePosition"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub old_handle_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"oldHandleRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub old_handle_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_since_last_modify: f32,
    /// # C++ Class Fields Info
    /// -   name:`"smoothlyChangingHandles"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub smoothly_changing_handles: bool,
}

impl Serialize for HkbEvaluateHandleModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEvaluateHandleModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEvaluateHandleModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEvaluateHandleModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbEvaluateHandleModifierVisitor<'a>>> for HkbEvaluateHandleModifier<'a> {
    fn from(_values: Vec<HkbEvaluateHandleModifierVisitor<'a>>) -> Self {
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
            let mut handle = None;
            let mut handle_position_out = None;
            let mut handle_rotation_out = None;
            let mut is_valid_out = None;
            let mut extrapolation_time_step = None;
            let mut handle_change_speed = None;
            let mut handle_change_mode = None;
            let mut old_handle = None;
            let mut old_handle_position = None;
            let mut old_handle_rotation = None;
            let mut time_since_last_modify = None;
            let mut smoothly_changing_handles = None;


        for _value in _values {
            match _value {
                HkbEvaluateHandleModifierVisitor::Enable(m) => enable = Some(m),
                HkbEvaluateHandleModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbEvaluateHandleModifierVisitor::UserData(m) => user_data = Some(m),
                HkbEvaluateHandleModifierVisitor::Name(m) => name = Some(m),
                HkbEvaluateHandleModifierVisitor::Id(m) => id = Some(m),
                HkbEvaluateHandleModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbEvaluateHandleModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbEvaluateHandleModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbEvaluateHandleModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbEvaluateHandleModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbEvaluateHandleModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbEvaluateHandleModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbEvaluateHandleModifierVisitor::Handle(m) => handle = Some(m),
                HkbEvaluateHandleModifierVisitor::HandlePositionOut(m) => handle_position_out = Some(m),
                HkbEvaluateHandleModifierVisitor::HandleRotationOut(m) => handle_rotation_out = Some(m),
                HkbEvaluateHandleModifierVisitor::IsValidOut(m) => is_valid_out = Some(m),
                HkbEvaluateHandleModifierVisitor::ExtrapolationTimeStep(m) => extrapolation_time_step = Some(m),
                HkbEvaluateHandleModifierVisitor::HandleChangeSpeed(m) => handle_change_speed = Some(m),
                HkbEvaluateHandleModifierVisitor::HandleChangeMode(m) => handle_change_mode = Some(m),
                HkbEvaluateHandleModifierVisitor::OldHandle(m) => old_handle = Some(m),
                HkbEvaluateHandleModifierVisitor::OldHandlePosition(m) => old_handle_position = Some(m),
                HkbEvaluateHandleModifierVisitor::OldHandleRotation(m) => old_handle_rotation = Some(m),
                HkbEvaluateHandleModifierVisitor::TimeSinceLastModify(m) => time_since_last_modify = Some(m),
                HkbEvaluateHandleModifierVisitor::SmoothlyChangingHandles(m) => smoothly_changing_handles = Some(m),

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
            handle: handle.unwrap_or_default().into_inner(),
            handle_position_out: handle_position_out.unwrap_or_default().into_inner(),
            handle_rotation_out: handle_rotation_out.unwrap_or_default().into_inner(),
            is_valid_out: is_valid_out.unwrap_or_default().into_inner(),
            extrapolation_time_step: extrapolation_time_step.unwrap_or_default().into_inner(),
            handle_change_speed: handle_change_speed.unwrap_or_default().into_inner(),
            handle_change_mode: handle_change_mode.unwrap_or_default().into_inner(),
            old_handle: old_handle.unwrap_or_default(),
            old_handle_position: old_handle_position.unwrap_or_default().into_inner(),
            old_handle_rotation: old_handle_rotation.unwrap_or_default().into_inner(),
            time_since_last_modify: time_since_last_modify.unwrap_or_default().into_inner(),
            smoothly_changing_handles: smoothly_changing_handles.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbEvaluateHandleModifier<'a>> for Vec<HkbEvaluateHandleModifierVisitor<'a>> {
    fn from(data: &HkbEvaluateHandleModifier<'a>) -> Self {
        vec![
            HkbEvaluateHandleModifierVisitor::Enable(data.enable.into()),
            HkbEvaluateHandleModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbEvaluateHandleModifierVisitor::UserData(data.user_data.into()),
            HkbEvaluateHandleModifierVisitor::Name(data.name.clone().into()),
            HkbEvaluateHandleModifierVisitor::Id(data.id.into()),
            HkbEvaluateHandleModifierVisitor::CloneState(data.clone_state.into()),
            HkbEvaluateHandleModifierVisitor::PadNode(data.pad_node.clone()),
            HkbEvaluateHandleModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbEvaluateHandleModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbEvaluateHandleModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbEvaluateHandleModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbEvaluateHandleModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbEvaluateHandleModifierVisitor::Handle(data.handle.clone().into()),
            HkbEvaluateHandleModifierVisitor::HandlePositionOut(data.handle_position_out.into()),
            HkbEvaluateHandleModifierVisitor::HandleRotationOut(data.handle_rotation_out.clone().into()),
            HkbEvaluateHandleModifierVisitor::IsValidOut(data.is_valid_out.into()),
            HkbEvaluateHandleModifierVisitor::ExtrapolationTimeStep(data.extrapolation_time_step.into()),
            HkbEvaluateHandleModifierVisitor::HandleChangeSpeed(data.handle_change_speed.into()),
            HkbEvaluateHandleModifierVisitor::HandleChangeMode(data.handle_change_mode.clone().into()),
            HkbEvaluateHandleModifierVisitor::OldHandle(data.old_handle.clone()),
            HkbEvaluateHandleModifierVisitor::OldHandlePosition(data.old_handle_position.into()),
            HkbEvaluateHandleModifierVisitor::OldHandleRotation(data.old_handle_rotation.clone().into()),
            HkbEvaluateHandleModifierVisitor::TimeSinceLastModify(data.time_since_last_modify.into()),
            HkbEvaluateHandleModifierVisitor::SmoothlyChangingHandles(data.smoothly_changing_handles.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEvaluateHandleModifier<'de> {
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
enum HkbEvaluateHandleModifierVisitor<'a> {
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
    #[serde(rename = "handle")]
    Handle(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "handlePositionOut")]
    HandlePositionOut(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "handleRotationOut")]
    HandleRotationOut(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "isValidOut")]
    IsValidOut(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(Primitive<HandleChangeMode>),
    /// Visitor fields
    #[serde(rename = "oldHandle", skip_serializing)]
    OldHandle(SingleClass<HkbHandle<'a>>),
    /// Visitor fields
    #[serde(rename = "oldHandlePosition", skip_serializing)]
    OldHandlePosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "oldHandleRotation", skip_serializing)]
    OldHandleRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "smoothlyChangingHandles", skip_serializing)]
    SmoothlyChangingHandles(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateHandleModifierVisitor<'de>, "@name",
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
    ("handle" => Handle(Primitive<Cow<'de, str>>)),
    ("handlePositionOut" => HandlePositionOut(Primitive<Vector4<f32>>)),
    ("handleRotationOut" => HandleRotationOut(Primitive<Quaternion<f32>>)),
    ("isValidOut" => IsValidOut(Primitive<bool>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("handleChangeSpeed" => HandleChangeSpeed(Primitive<f32>)),
    ("handleChangeMode" => HandleChangeMode(Primitive<HandleChangeMode>)),
    ("oldHandle" => OldHandle(SingleClass<HkbHandle<'de>>)),
    ("oldHandlePosition" => OldHandlePosition(Primitive<Vector4<f32>>)),
    ("oldHandleRotation" => OldHandleRotation(Primitive<Quaternion<f32>>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("smoothlyChangingHandles" => SmoothlyChangingHandles(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    #[default]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
