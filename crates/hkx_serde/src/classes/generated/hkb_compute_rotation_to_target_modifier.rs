//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbComputeRotationToTargetModifier`
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

/// `hkbComputeRotationToTargetModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x47665f1c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbComputeRotationToTargetModifier<'a> {
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
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub rotation_out: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub target_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"currentPosition"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub current_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"currentRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub current_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"localAxisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub local_axis_of_rotation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"localFacingDirection"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub local_facing_direction: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"resultIsDelta"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub result_is_delta: bool,
}

impl Serialize for HkbComputeRotationToTargetModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbComputeRotationToTargetModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbComputeRotationToTargetModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbComputeRotationToTargetModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbComputeRotationToTargetModifierVisitor<'a>>> for HkbComputeRotationToTargetModifier<'a> {
    fn from(_values: Vec<HkbComputeRotationToTargetModifierVisitor<'a>>) -> Self {
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
            let mut rotation_out = None;
            let mut target_position = None;
            let mut current_position = None;
            let mut current_rotation = None;
            let mut local_axis_of_rotation = None;
            let mut local_facing_direction = None;
            let mut result_is_delta = None;


        for _value in _values {
            match _value {
                HkbComputeRotationToTargetModifierVisitor::Enable(m) => enable = Some(m),
                HkbComputeRotationToTargetModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbComputeRotationToTargetModifierVisitor::UserData(m) => user_data = Some(m),
                HkbComputeRotationToTargetModifierVisitor::Name(m) => name = Some(m),
                HkbComputeRotationToTargetModifierVisitor::Id(m) => id = Some(m),
                HkbComputeRotationToTargetModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbComputeRotationToTargetModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbComputeRotationToTargetModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbComputeRotationToTargetModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbComputeRotationToTargetModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbComputeRotationToTargetModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbComputeRotationToTargetModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbComputeRotationToTargetModifierVisitor::RotationOut(m) => rotation_out = Some(m),
                HkbComputeRotationToTargetModifierVisitor::TargetPosition(m) => target_position = Some(m),
                HkbComputeRotationToTargetModifierVisitor::CurrentPosition(m) => current_position = Some(m),
                HkbComputeRotationToTargetModifierVisitor::CurrentRotation(m) => current_rotation = Some(m),
                HkbComputeRotationToTargetModifierVisitor::LocalAxisOfRotation(m) => local_axis_of_rotation = Some(m),
                HkbComputeRotationToTargetModifierVisitor::LocalFacingDirection(m) => local_facing_direction = Some(m),
                HkbComputeRotationToTargetModifierVisitor::ResultIsDelta(m) => result_is_delta = Some(m),

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
            rotation_out: rotation_out.unwrap_or_default().into_inner(),
            target_position: target_position.unwrap_or_default().into_inner(),
            current_position: current_position.unwrap_or_default().into_inner(),
            current_rotation: current_rotation.unwrap_or_default().into_inner(),
            local_axis_of_rotation: local_axis_of_rotation.unwrap_or_default().into_inner(),
            local_facing_direction: local_facing_direction.unwrap_or_default().into_inner(),
            result_is_delta: result_is_delta.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbComputeRotationToTargetModifier<'a>> for Vec<HkbComputeRotationToTargetModifierVisitor<'a>> {
    fn from(data: &HkbComputeRotationToTargetModifier<'a>) -> Self {
        vec![
            HkbComputeRotationToTargetModifierVisitor::Enable(data.enable.into()),
            HkbComputeRotationToTargetModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbComputeRotationToTargetModifierVisitor::UserData(data.user_data.into()),
            HkbComputeRotationToTargetModifierVisitor::Name(data.name.clone().into()),
            HkbComputeRotationToTargetModifierVisitor::Id(data.id.into()),
            HkbComputeRotationToTargetModifierVisitor::CloneState(data.clone_state.into()),
            HkbComputeRotationToTargetModifierVisitor::PadNode(data.pad_node.clone()),
            HkbComputeRotationToTargetModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbComputeRotationToTargetModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbComputeRotationToTargetModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbComputeRotationToTargetModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbComputeRotationToTargetModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbComputeRotationToTargetModifierVisitor::RotationOut(data.rotation_out.clone().into()),
            HkbComputeRotationToTargetModifierVisitor::TargetPosition(data.target_position.into()),
            HkbComputeRotationToTargetModifierVisitor::CurrentPosition(data.current_position.into()),
            HkbComputeRotationToTargetModifierVisitor::CurrentRotation(data.current_rotation.clone().into()),
            HkbComputeRotationToTargetModifierVisitor::LocalAxisOfRotation(data.local_axis_of_rotation.into()),
            HkbComputeRotationToTargetModifierVisitor::LocalFacingDirection(data.local_facing_direction.into()),
            HkbComputeRotationToTargetModifierVisitor::ResultIsDelta(data.result_is_delta.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbComputeRotationToTargetModifier<'de> {
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
enum HkbComputeRotationToTargetModifierVisitor<'a> {
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
    #[serde(rename = "rotationOut")]
    RotationOut(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "targetPosition")]
    TargetPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "currentPosition")]
    CurrentPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "currentRotation")]
    CurrentRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "localAxisOfRotation")]
    LocalAxisOfRotation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "localFacingDirection")]
    LocalFacingDirection(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "resultIsDelta")]
    ResultIsDelta(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeRotationToTargetModifierVisitor<'de>, "@name",
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
    ("rotationOut" => RotationOut(Primitive<Quaternion<f32>>)),
    ("targetPosition" => TargetPosition(Primitive<Vector4<f32>>)),
    ("currentPosition" => CurrentPosition(Primitive<Vector4<f32>>)),
    ("currentRotation" => CurrentRotation(Primitive<Quaternion<f32>>)),
    ("localAxisOfRotation" => LocalAxisOfRotation(Primitive<Vector4<f32>>)),
    ("localFacingDirection" => LocalFacingDirection(Primitive<Vector4<f32>>)),
    ("resultIsDelta" => ResultIsDelta(Primitive<bool>)),
}
