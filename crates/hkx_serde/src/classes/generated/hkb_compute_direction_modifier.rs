//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbComputeDirectionModifier`
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

/// `hkbComputeDirectionModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xdf358bd3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbComputeDirectionModifier<'a> {
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
    /// -   name:`"pointIn"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub point_in: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"pointOut"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub point_out: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"groundAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub ground_angle_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"upAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub up_angle_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub vertical_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"reverseGroundAngle"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub reverse_ground_angle: bool,
    /// # C++ Class Fields Info
    /// -   name:`"reverseUpAngle"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    pub reverse_up_angle: bool,
    /// # C++ Class Fields Info
    /// -   name:`"projectPoint"`
    /// -   type: `hkBool`
    /// - offset: 94
    /// -  flags: `FLAGS_NONE`
    pub project_point: bool,
    /// # C++ Class Fields Info
    /// -   name:`"normalizePoint"`
    /// -   type: `hkBool`
    /// - offset: 95
    /// -  flags: `FLAGS_NONE`
    pub normalize_point: bool,
    /// # C++ Class Fields Info
    /// -   name:`"computeOnlyOnce"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub compute_only_once: bool,
    /// # C++ Class Fields Info
    /// -   name:`"computedOutput"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    pub computed_output: bool,
}

impl Serialize for HkbComputeDirectionModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbComputeDirectionModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbComputeDirectionModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbComputeDirectionModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbComputeDirectionModifierVisitor<'a>>> for HkbComputeDirectionModifier<'a> {
    fn from(_values: Vec<HkbComputeDirectionModifierVisitor<'a>>) -> Self {
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
            let mut point_in = None;
            let mut point_out = None;
            let mut ground_angle_out = None;
            let mut up_angle_out = None;
            let mut vertical_offset = None;
            let mut reverse_ground_angle = None;
            let mut reverse_up_angle = None;
            let mut project_point = None;
            let mut normalize_point = None;
            let mut compute_only_once = None;
            let mut computed_output = None;


        for _value in _values {
            match _value {
                HkbComputeDirectionModifierVisitor::Enable(m) => enable = Some(m),
                HkbComputeDirectionModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbComputeDirectionModifierVisitor::UserData(m) => user_data = Some(m),
                HkbComputeDirectionModifierVisitor::Name(m) => name = Some(m),
                HkbComputeDirectionModifierVisitor::Id(m) => id = Some(m),
                HkbComputeDirectionModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbComputeDirectionModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbComputeDirectionModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbComputeDirectionModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbComputeDirectionModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbComputeDirectionModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbComputeDirectionModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbComputeDirectionModifierVisitor::PointIn(m) => point_in = Some(m),
                HkbComputeDirectionModifierVisitor::PointOut(m) => point_out = Some(m),
                HkbComputeDirectionModifierVisitor::GroundAngleOut(m) => ground_angle_out = Some(m),
                HkbComputeDirectionModifierVisitor::UpAngleOut(m) => up_angle_out = Some(m),
                HkbComputeDirectionModifierVisitor::VerticalOffset(m) => vertical_offset = Some(m),
                HkbComputeDirectionModifierVisitor::ReverseGroundAngle(m) => reverse_ground_angle = Some(m),
                HkbComputeDirectionModifierVisitor::ReverseUpAngle(m) => reverse_up_angle = Some(m),
                HkbComputeDirectionModifierVisitor::ProjectPoint(m) => project_point = Some(m),
                HkbComputeDirectionModifierVisitor::NormalizePoint(m) => normalize_point = Some(m),
                HkbComputeDirectionModifierVisitor::ComputeOnlyOnce(m) => compute_only_once = Some(m),
                HkbComputeDirectionModifierVisitor::ComputedOutput(m) => computed_output = Some(m),

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
            point_in: point_in.unwrap_or_default().into_inner(),
            point_out: point_out.unwrap_or_default().into_inner(),
            ground_angle_out: ground_angle_out.unwrap_or_default().into_inner(),
            up_angle_out: up_angle_out.unwrap_or_default().into_inner(),
            vertical_offset: vertical_offset.unwrap_or_default().into_inner(),
            reverse_ground_angle: reverse_ground_angle.unwrap_or_default().into_inner(),
            reverse_up_angle: reverse_up_angle.unwrap_or_default().into_inner(),
            project_point: project_point.unwrap_or_default().into_inner(),
            normalize_point: normalize_point.unwrap_or_default().into_inner(),
            compute_only_once: compute_only_once.unwrap_or_default().into_inner(),
            computed_output: computed_output.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbComputeDirectionModifier<'a>> for Vec<HkbComputeDirectionModifierVisitor<'a>> {
    fn from(data: &HkbComputeDirectionModifier<'a>) -> Self {
        vec![
            HkbComputeDirectionModifierVisitor::Enable(data.enable.into()),
            HkbComputeDirectionModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbComputeDirectionModifierVisitor::UserData(data.user_data.into()),
            HkbComputeDirectionModifierVisitor::Name(data.name.clone().into()),
            HkbComputeDirectionModifierVisitor::Id(data.id.into()),
            HkbComputeDirectionModifierVisitor::CloneState(data.clone_state.into()),
            HkbComputeDirectionModifierVisitor::PadNode(data.pad_node.clone()),
            HkbComputeDirectionModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbComputeDirectionModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbComputeDirectionModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbComputeDirectionModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbComputeDirectionModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbComputeDirectionModifierVisitor::PointIn(data.point_in.into()),
            HkbComputeDirectionModifierVisitor::PointOut(data.point_out.into()),
            HkbComputeDirectionModifierVisitor::GroundAngleOut(data.ground_angle_out.into()),
            HkbComputeDirectionModifierVisitor::UpAngleOut(data.up_angle_out.into()),
            HkbComputeDirectionModifierVisitor::VerticalOffset(data.vertical_offset.into()),
            HkbComputeDirectionModifierVisitor::ReverseGroundAngle(data.reverse_ground_angle.into()),
            HkbComputeDirectionModifierVisitor::ReverseUpAngle(data.reverse_up_angle.into()),
            HkbComputeDirectionModifierVisitor::ProjectPoint(data.project_point.into()),
            HkbComputeDirectionModifierVisitor::NormalizePoint(data.normalize_point.into()),
            HkbComputeDirectionModifierVisitor::ComputeOnlyOnce(data.compute_only_once.into()),
            HkbComputeDirectionModifierVisitor::ComputedOutput(data.computed_output.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbComputeDirectionModifier<'de> {
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
enum HkbComputeDirectionModifierVisitor<'a> {
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
    #[serde(rename = "pointIn")]
    PointIn(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "pointOut")]
    PointOut(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "groundAngleOut")]
    GroundAngleOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "upAngleOut")]
    UpAngleOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "reverseGroundAngle")]
    ReverseGroundAngle(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "reverseUpAngle")]
    ReverseUpAngle(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "projectPoint")]
    ProjectPoint(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "normalizePoint")]
    NormalizePoint(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "computeOnlyOnce")]
    ComputeOnlyOnce(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "computedOutput")]
    ComputedOutput(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeDirectionModifierVisitor<'de>, "@name",
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
    ("pointIn" => PointIn(Primitive<Vector4<f32>>)),
    ("pointOut" => PointOut(Primitive<Vector4<f32>>)),
    ("groundAngleOut" => GroundAngleOut(Primitive<f32>)),
    ("upAngleOut" => UpAngleOut(Primitive<f32>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("reverseGroundAngle" => ReverseGroundAngle(Primitive<bool>)),
    ("reverseUpAngle" => ReverseUpAngle(Primitive<bool>)),
    ("projectPoint" => ProjectPoint(Primitive<bool>)),
    ("normalizePoint" => NormalizePoint(Primitive<bool>)),
    ("computeOnlyOnce" => ComputeOnlyOnce(Primitive<bool>)),
    ("computedOutput" => ComputedOutput(Primitive<bool>)),
}
