//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDampingModifier`
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

/// `hkbDampingModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x9a040f03`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbDampingModifier<'a> {
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
    /// -   name:`"kP"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub k_p: f32,
    /// # C++ Class Fields Info
    /// -   name:`"kI"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub k_i: f32,
    /// # C++ Class Fields Info
    /// -   name:`"kD"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub k_d: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enableScalarDamping"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub enable_scalar_damping: bool,
    /// # C++ Class Fields Info
    /// -   name:`"enableVectorDamping"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    pub enable_vector_damping: bool,
    /// # C++ Class Fields Info
    /// -   name:`"rawValue"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub raw_value: f32,
    /// # C++ Class Fields Info
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub damped_value: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rawVector"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub raw_vector: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub damped_vector: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub vec_error_sum: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub vec_previous_error: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub error_sum: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub previous_error: f32,
}

impl Serialize for HkbDampingModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbDampingModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbDampingModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbDampingModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbDampingModifierVisitor<'a>>> for HkbDampingModifier<'a> {
    fn from(_values: Vec<HkbDampingModifierVisitor<'a>>) -> Self {
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
            let mut k_p = None;
            let mut k_i = None;
            let mut k_d = None;
            let mut enable_scalar_damping = None;
            let mut enable_vector_damping = None;
            let mut raw_value = None;
            let mut damped_value = None;
            let mut raw_vector = None;
            let mut damped_vector = None;
            let mut vec_error_sum = None;
            let mut vec_previous_error = None;
            let mut error_sum = None;
            let mut previous_error = None;


        for _value in _values {
            match _value {
                HkbDampingModifierVisitor::Enable(m) => enable = Some(m),
                HkbDampingModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbDampingModifierVisitor::UserData(m) => user_data = Some(m),
                HkbDampingModifierVisitor::Name(m) => name = Some(m),
                HkbDampingModifierVisitor::Id(m) => id = Some(m),
                HkbDampingModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbDampingModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbDampingModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbDampingModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbDampingModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbDampingModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbDampingModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbDampingModifierVisitor::KP(m) => k_p = Some(m),
                HkbDampingModifierVisitor::KI(m) => k_i = Some(m),
                HkbDampingModifierVisitor::KD(m) => k_d = Some(m),
                HkbDampingModifierVisitor::EnableScalarDamping(m) => enable_scalar_damping = Some(m),
                HkbDampingModifierVisitor::EnableVectorDamping(m) => enable_vector_damping = Some(m),
                HkbDampingModifierVisitor::RawValue(m) => raw_value = Some(m),
                HkbDampingModifierVisitor::DampedValue(m) => damped_value = Some(m),
                HkbDampingModifierVisitor::RawVector(m) => raw_vector = Some(m),
                HkbDampingModifierVisitor::DampedVector(m) => damped_vector = Some(m),
                HkbDampingModifierVisitor::VecErrorSum(m) => vec_error_sum = Some(m),
                HkbDampingModifierVisitor::VecPreviousError(m) => vec_previous_error = Some(m),
                HkbDampingModifierVisitor::ErrorSum(m) => error_sum = Some(m),
                HkbDampingModifierVisitor::PreviousError(m) => previous_error = Some(m),

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
            k_p: k_p.unwrap_or_default().into_inner(),
            k_i: k_i.unwrap_or_default().into_inner(),
            k_d: k_d.unwrap_or_default().into_inner(),
            enable_scalar_damping: enable_scalar_damping.unwrap_or_default().into_inner(),
            enable_vector_damping: enable_vector_damping.unwrap_or_default().into_inner(),
            raw_value: raw_value.unwrap_or_default().into_inner(),
            damped_value: damped_value.unwrap_or_default().into_inner(),
            raw_vector: raw_vector.unwrap_or_default().into_inner(),
            damped_vector: damped_vector.unwrap_or_default().into_inner(),
            vec_error_sum: vec_error_sum.unwrap_or_default().into_inner(),
            vec_previous_error: vec_previous_error.unwrap_or_default().into_inner(),
            error_sum: error_sum.unwrap_or_default().into_inner(),
            previous_error: previous_error.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbDampingModifier<'a>> for Vec<HkbDampingModifierVisitor<'a>> {
    fn from(data: &HkbDampingModifier<'a>) -> Self {
        vec![
            HkbDampingModifierVisitor::Enable(data.enable.into()),
            HkbDampingModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbDampingModifierVisitor::UserData(data.user_data.into()),
            HkbDampingModifierVisitor::Name(data.name.clone().into()),
            HkbDampingModifierVisitor::Id(data.id.into()),
            HkbDampingModifierVisitor::CloneState(data.clone_state.into()),
            HkbDampingModifierVisitor::PadNode(data.pad_node.clone()),
            HkbDampingModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbDampingModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbDampingModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbDampingModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbDampingModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbDampingModifierVisitor::KP(data.k_p.into()),
            HkbDampingModifierVisitor::KI(data.k_i.into()),
            HkbDampingModifierVisitor::KD(data.k_d.into()),
            HkbDampingModifierVisitor::EnableScalarDamping(data.enable_scalar_damping.into()),
            HkbDampingModifierVisitor::EnableVectorDamping(data.enable_vector_damping.into()),
            HkbDampingModifierVisitor::RawValue(data.raw_value.into()),
            HkbDampingModifierVisitor::DampedValue(data.damped_value.into()),
            HkbDampingModifierVisitor::RawVector(data.raw_vector.into()),
            HkbDampingModifierVisitor::DampedVector(data.damped_vector.into()),
            HkbDampingModifierVisitor::VecErrorSum(data.vec_error_sum.into()),
            HkbDampingModifierVisitor::VecPreviousError(data.vec_previous_error.into()),
            HkbDampingModifierVisitor::ErrorSum(data.error_sum.into()),
            HkbDampingModifierVisitor::PreviousError(data.previous_error.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbDampingModifier<'de> {
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
enum HkbDampingModifierVisitor<'a> {
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
    #[serde(rename = "kP")]
    KP(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "kI")]
    KI(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "kD")]
    KD(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "enableScalarDamping")]
    EnableScalarDamping(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "enableVectorDamping")]
    EnableVectorDamping(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "rawValue")]
    RawValue(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rawVector")]
    RawVector(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "dampedVector")]
    DampedVector(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierVisitor<'de>, "@name",
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
    ("kP" => KP(Primitive<f32>)),
    ("kI" => KI(Primitive<f32>)),
    ("kD" => KD(Primitive<f32>)),
    ("enableScalarDamping" => EnableScalarDamping(Primitive<bool>)),
    ("enableVectorDamping" => EnableVectorDamping(Primitive<bool>)),
    ("rawValue" => RawValue(Primitive<f32>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("rawVector" => RawVector(Primitive<Vector4<f32>>)),
    ("dampedVector" => DampedVector(Primitive<Vector4<f32>>)),
    ("vecErrorSum" => VecErrorSum(Primitive<Vector4<f32>>)),
    ("vecPreviousError" => VecPreviousError(Primitive<Vector4<f32>>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
