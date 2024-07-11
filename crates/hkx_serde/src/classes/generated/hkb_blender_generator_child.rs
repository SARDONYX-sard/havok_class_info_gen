//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlenderGeneratorChild`
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

/// `hkbBlenderGeneratorChild`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xe2b384b0`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBlenderGeneratorChild<'a> {
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
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"boneWeights"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub bone_weights: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"weight"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub weight: f32,
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelWeight"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub world_from_model_weight: f32,
}

impl Serialize for HkbBlenderGeneratorChild<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBlenderGeneratorChildVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBlenderGeneratorChild<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBlenderGeneratorChildVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBlenderGeneratorChildVisitor<'a>>> for HkbBlenderGeneratorChild<'a> {
    fn from(_values: Vec<HkbBlenderGeneratorChildVisitor<'a>>) -> Self {
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut generator = None;
            let mut bone_weights = None;
            let mut weight = None;
            let mut world_from_model_weight = None;


        for _value in _values {
            match _value {
                HkbBlenderGeneratorChildVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbBlenderGeneratorChildVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbBlenderGeneratorChildVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbBlenderGeneratorChildVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBlenderGeneratorChildVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBlenderGeneratorChildVisitor::Generator(m) => generator = Some(m),
                HkbBlenderGeneratorChildVisitor::BoneWeights(m) => bone_weights = Some(m),
                HkbBlenderGeneratorChildVisitor::Weight(m) => weight = Some(m),
                HkbBlenderGeneratorChildVisitor::WorldFromModelWeight(m) => world_from_model_weight = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            generator: generator.unwrap_or_default().into_inner(),
            bone_weights: bone_weights.unwrap_or_default().into_inner(),
            weight: weight.unwrap_or_default().into_inner(),
            world_from_model_weight: world_from_model_weight.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBlenderGeneratorChild<'a>> for Vec<HkbBlenderGeneratorChildVisitor<'a>> {
    fn from(data: &HkbBlenderGeneratorChild<'a>) -> Self {
        vec![
            HkbBlenderGeneratorChildVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbBlenderGeneratorChildVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbBlenderGeneratorChildVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbBlenderGeneratorChildVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBlenderGeneratorChildVisitor::ReferenceCount(data.reference_count.into()),
            HkbBlenderGeneratorChildVisitor::Generator(data.generator.clone().into()),
            HkbBlenderGeneratorChildVisitor::BoneWeights(data.bone_weights.clone().into()),
            HkbBlenderGeneratorChildVisitor::Weight(data.weight.into()),
            HkbBlenderGeneratorChildVisitor::WorldFromModelWeight(data.world_from_model_weight.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBlenderGeneratorChild<'de> {
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
enum HkbBlenderGeneratorChildVisitor<'a> {
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
    #[serde(rename = "generator")]
    Generator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "boneWeights")]
    BoneWeights(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "weight")]
    Weight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "worldFromModelWeight")]
    WorldFromModelWeight(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorChildVisitor<'de>, "@name",
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("generator" => Generator(Primitive<Cow<'de, str>>)),
    ("boneWeights" => BoneWeights(Primitive<Cow<'de, str>>)),
    ("weight" => Weight(Primitive<f32>)),
    ("worldFromModelWeight" => WorldFromModelWeight(Primitive<f32>)),
}
