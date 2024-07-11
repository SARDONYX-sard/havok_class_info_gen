//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTwistModifier`
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

/// `hkbTwistModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xb6b76b32`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbTwistModifier<'a> {
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
    /// -   name:`"axisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub axis_of_rotation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"twistAngle"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub twist_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub start_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    pub end_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"setAngleMethod"`
    /// -   type: `enum SetAngleMethod`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub set_angle_method: SetAngleMethod,
    /// # C++ Class Fields Info
    /// -   name:`"rotationAxisCoordinates"`
    /// -   type: `enum RotationAxisCoordinates`
    /// - offset: 73
    /// -  flags: `FLAGS_NONE`
    pub rotation_axis_coordinates: RotationAxisCoordinates,
    /// # C++ Class Fields Info
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    pub is_additive: bool,
    /// # C++ Class Fields Info
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub bone_chain_indices: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"parentBoneIndices"`
    /// -   type: `hkArray<void>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub parent_bone_indices: HkArrayRef<()>,
}

impl Serialize for HkbTwistModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbTwistModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbTwistModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbTwistModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbTwistModifierVisitor<'a>>> for HkbTwistModifier<'a> {
    fn from(_values: Vec<HkbTwistModifierVisitor<'a>>) -> Self {
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
            let mut axis_of_rotation = None;
            let mut twist_angle = None;
            let mut start_bone_index = None;
            let mut end_bone_index = None;
            let mut set_angle_method = None;
            let mut rotation_axis_coordinates = None;
            let mut is_additive = None;
            let mut bone_chain_indices = None;
            let mut parent_bone_indices = None;


        for _value in _values {
            match _value {
                HkbTwistModifierVisitor::Enable(m) => enable = Some(m),
                HkbTwistModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbTwistModifierVisitor::UserData(m) => user_data = Some(m),
                HkbTwistModifierVisitor::Name(m) => name = Some(m),
                HkbTwistModifierVisitor::Id(m) => id = Some(m),
                HkbTwistModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbTwistModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbTwistModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbTwistModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbTwistModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbTwistModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbTwistModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbTwistModifierVisitor::AxisOfRotation(m) => axis_of_rotation = Some(m),
                HkbTwistModifierVisitor::TwistAngle(m) => twist_angle = Some(m),
                HkbTwistModifierVisitor::StartBoneIndex(m) => start_bone_index = Some(m),
                HkbTwistModifierVisitor::EndBoneIndex(m) => end_bone_index = Some(m),
                HkbTwistModifierVisitor::SetAngleMethod(m) => set_angle_method = Some(m),
                HkbTwistModifierVisitor::RotationAxisCoordinates(m) => rotation_axis_coordinates = Some(m),
                HkbTwistModifierVisitor::IsAdditive(m) => is_additive = Some(m),
                HkbTwistModifierVisitor::BoneChainIndices(m) => bone_chain_indices = Some(m),
                HkbTwistModifierVisitor::ParentBoneIndices(m) => parent_bone_indices = Some(m),

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
            axis_of_rotation: axis_of_rotation.unwrap_or_default().into_inner(),
            twist_angle: twist_angle.unwrap_or_default().into_inner(),
            start_bone_index: start_bone_index.unwrap_or_default().into_inner(),
            end_bone_index: end_bone_index.unwrap_or_default().into_inner(),
            set_angle_method: set_angle_method.unwrap_or_default().into_inner(),
            rotation_axis_coordinates: rotation_axis_coordinates.unwrap_or_default().into_inner(),
            is_additive: is_additive.unwrap_or_default().into_inner(),
            bone_chain_indices: bone_chain_indices.unwrap_or_default(),
            parent_bone_indices: parent_bone_indices.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbTwistModifier<'a>> for Vec<HkbTwistModifierVisitor<'a>> {
    fn from(data: &HkbTwistModifier<'a>) -> Self {
        vec![
            HkbTwistModifierVisitor::Enable(data.enable.into()),
            HkbTwistModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbTwistModifierVisitor::UserData(data.user_data.into()),
            HkbTwistModifierVisitor::Name(data.name.clone().into()),
            HkbTwistModifierVisitor::Id(data.id.into()),
            HkbTwistModifierVisitor::CloneState(data.clone_state.into()),
            HkbTwistModifierVisitor::PadNode(data.pad_node.clone()),
            HkbTwistModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbTwistModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbTwistModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbTwistModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbTwistModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbTwistModifierVisitor::AxisOfRotation(data.axis_of_rotation.into()),
            HkbTwistModifierVisitor::TwistAngle(data.twist_angle.into()),
            HkbTwistModifierVisitor::StartBoneIndex(data.start_bone_index.into()),
            HkbTwistModifierVisitor::EndBoneIndex(data.end_bone_index.into()),
            HkbTwistModifierVisitor::SetAngleMethod(data.set_angle_method.clone().into()),
            HkbTwistModifierVisitor::RotationAxisCoordinates(data.rotation_axis_coordinates.clone().into()),
            HkbTwistModifierVisitor::IsAdditive(data.is_additive.into()),
            HkbTwistModifierVisitor::BoneChainIndices(data.bone_chain_indices.clone()),
            HkbTwistModifierVisitor::ParentBoneIndices(data.parent_bone_indices.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbTwistModifier<'de> {
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
enum HkbTwistModifierVisitor<'a> {
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
    #[serde(rename = "axisOfRotation")]
    AxisOfRotation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "twistAngle")]
    TwistAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "setAngleMethod")]
    SetAngleMethod(Primitive<SetAngleMethod>),
    /// Visitor fields
    #[serde(rename = "rotationAxisCoordinates")]
    RotationAxisCoordinates(Primitive<RotationAxisCoordinates>),
    /// Visitor fields
    #[serde(rename = "isAdditive")]
    IsAdditive(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "boneChainIndices", skip_serializing)]
    BoneChainIndices(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "parentBoneIndices", skip_serializing)]
    ParentBoneIndices(HkArrayRef<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTwistModifierVisitor<'de>, "@name",
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
    ("axisOfRotation" => AxisOfRotation(Primitive<Vector4<f32>>)),
    ("twistAngle" => TwistAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("setAngleMethod" => SetAngleMethod(Primitive<SetAngleMethod>)),
    ("rotationAxisCoordinates" => RotationAxisCoordinates(Primitive<RotationAxisCoordinates>)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
    ("boneChainIndices" => BoneChainIndices(HkArrayRef<()>)),
    ("parentBoneIndices" => ParentBoneIndices(HkArrayRef<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SetAngleMethod {
    #[serde(rename = "LINEAR")]
    #[default]
    Linear = 0,
    #[serde(rename = "RAMPED")]
    Ramped = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum RotationAxisCoordinates {
    #[serde(rename = "ROTATION_AXIS_IN_MODEL_COORDINATES")]
    #[default]
    RotationAxisInModelCoordinates = 0,
    #[serde(rename = "ROTATION_AXIS_IN_LOCAL_COORDINATES")]
    RotationAxisInLocalCoordinates = 1,
}
