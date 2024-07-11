//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTransformVectorModifier`
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

/// `hkbTransformVectorModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xf93e0e24`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbTransformVectorModifier<'a> {
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
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub translation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vectorIn"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub vector_in: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vectorOut"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub vector_out: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotateOnly"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub rotate_only: bool,
    /// # C++ Class Fields Info
    /// -   name:`"inverse"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE`
    pub inverse: bool,
    /// # C++ Class Fields Info
    /// -   name:`"computeOnActivate"`
    /// -   type: `hkBool`
    /// - offset: 114
    /// -  flags: `FLAGS_NONE`
    pub compute_on_activate: bool,
    /// # C++ Class Fields Info
    /// -   name:`"computeOnModify"`
    /// -   type: `hkBool`
    /// - offset: 115
    /// -  flags: `FLAGS_NONE`
    pub compute_on_modify: bool,
}

impl Serialize for HkbTransformVectorModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbTransformVectorModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbTransformVectorModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbTransformVectorModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbTransformVectorModifierVisitor<'a>>> for HkbTransformVectorModifier<'a> {
    fn from(_values: Vec<HkbTransformVectorModifierVisitor<'a>>) -> Self {
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
            let mut rotation = None;
            let mut translation = None;
            let mut vector_in = None;
            let mut vector_out = None;
            let mut rotate_only = None;
            let mut inverse = None;
            let mut compute_on_activate = None;
            let mut compute_on_modify = None;


        for _value in _values {
            match _value {
                HkbTransformVectorModifierVisitor::Enable(m) => enable = Some(m),
                HkbTransformVectorModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbTransformVectorModifierVisitor::UserData(m) => user_data = Some(m),
                HkbTransformVectorModifierVisitor::Name(m) => name = Some(m),
                HkbTransformVectorModifierVisitor::Id(m) => id = Some(m),
                HkbTransformVectorModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbTransformVectorModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbTransformVectorModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbTransformVectorModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbTransformVectorModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbTransformVectorModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbTransformVectorModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbTransformVectorModifierVisitor::Rotation(m) => rotation = Some(m),
                HkbTransformVectorModifierVisitor::Translation(m) => translation = Some(m),
                HkbTransformVectorModifierVisitor::VectorIn(m) => vector_in = Some(m),
                HkbTransformVectorModifierVisitor::VectorOut(m) => vector_out = Some(m),
                HkbTransformVectorModifierVisitor::RotateOnly(m) => rotate_only = Some(m),
                HkbTransformVectorModifierVisitor::Inverse(m) => inverse = Some(m),
                HkbTransformVectorModifierVisitor::ComputeOnActivate(m) => compute_on_activate = Some(m),
                HkbTransformVectorModifierVisitor::ComputeOnModify(m) => compute_on_modify = Some(m),

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
            rotation: rotation.unwrap_or_default().into_inner(),
            translation: translation.unwrap_or_default().into_inner(),
            vector_in: vector_in.unwrap_or_default().into_inner(),
            vector_out: vector_out.unwrap_or_default().into_inner(),
            rotate_only: rotate_only.unwrap_or_default().into_inner(),
            inverse: inverse.unwrap_or_default().into_inner(),
            compute_on_activate: compute_on_activate.unwrap_or_default().into_inner(),
            compute_on_modify: compute_on_modify.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbTransformVectorModifier<'a>> for Vec<HkbTransformVectorModifierVisitor<'a>> {
    fn from(data: &HkbTransformVectorModifier<'a>) -> Self {
        vec![
            HkbTransformVectorModifierVisitor::Enable(data.enable.into()),
            HkbTransformVectorModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbTransformVectorModifierVisitor::UserData(data.user_data.into()),
            HkbTransformVectorModifierVisitor::Name(data.name.clone().into()),
            HkbTransformVectorModifierVisitor::Id(data.id.into()),
            HkbTransformVectorModifierVisitor::CloneState(data.clone_state.into()),
            HkbTransformVectorModifierVisitor::PadNode(data.pad_node.clone()),
            HkbTransformVectorModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbTransformVectorModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbTransformVectorModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbTransformVectorModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbTransformVectorModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbTransformVectorModifierVisitor::Rotation(data.rotation.clone().into()),
            HkbTransformVectorModifierVisitor::Translation(data.translation.into()),
            HkbTransformVectorModifierVisitor::VectorIn(data.vector_in.into()),
            HkbTransformVectorModifierVisitor::VectorOut(data.vector_out.into()),
            HkbTransformVectorModifierVisitor::RotateOnly(data.rotate_only.into()),
            HkbTransformVectorModifierVisitor::Inverse(data.inverse.into()),
            HkbTransformVectorModifierVisitor::ComputeOnActivate(data.compute_on_activate.into()),
            HkbTransformVectorModifierVisitor::ComputeOnModify(data.compute_on_modify.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbTransformVectorModifier<'de> {
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
enum HkbTransformVectorModifierVisitor<'a> {
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
    #[serde(rename = "rotation")]
    Rotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "translation")]
    Translation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vectorIn")]
    VectorIn(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vectorOut")]
    VectorOut(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rotateOnly")]
    RotateOnly(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "inverse")]
    Inverse(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "computeOnActivate")]
    ComputeOnActivate(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "computeOnModify")]
    ComputeOnModify(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransformVectorModifierVisitor<'de>, "@name",
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
    ("rotation" => Rotation(Primitive<Quaternion<f32>>)),
    ("translation" => Translation(Primitive<Vector4<f32>>)),
    ("vectorIn" => VectorIn(Primitive<Vector4<f32>>)),
    ("vectorOut" => VectorOut(Primitive<Vector4<f32>>)),
    ("rotateOnly" => RotateOnly(Primitive<bool>)),
    ("inverse" => Inverse(Primitive<bool>)),
    ("computeOnActivate" => ComputeOnActivate(Primitive<bool>)),
    ("computeOnModify" => ComputeOnModify(Primitive<bool>)),
}
