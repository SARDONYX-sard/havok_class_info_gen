//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBoneWeightArray`
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

/// `hkbBoneWeightArray`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xcd902b77`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBoneWeightArray<'a> {
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
    /// -   name:`"boneWeights"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub bone_weights: HkArrayNum<f32>,
}

impl Serialize for HkbBoneWeightArray<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBoneWeightArrayVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBoneWeightArray<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBoneWeightArrayVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBoneWeightArrayVisitor<'a>>> for HkbBoneWeightArray<'a> {
    fn from(_values: Vec<HkbBoneWeightArrayVisitor<'a>>) -> Self {
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut bone_weights = None;


        for _value in _values {
            match _value {
                HkbBoneWeightArrayVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbBoneWeightArrayVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbBoneWeightArrayVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbBoneWeightArrayVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBoneWeightArrayVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBoneWeightArrayVisitor::BoneWeights(m) => bone_weights = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            bone_weights: bone_weights.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBoneWeightArray<'a>> for Vec<HkbBoneWeightArrayVisitor<'a>> {
    fn from(data: &HkbBoneWeightArray<'a>) -> Self {
        vec![
            HkbBoneWeightArrayVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbBoneWeightArrayVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbBoneWeightArrayVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbBoneWeightArrayVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBoneWeightArrayVisitor::ReferenceCount(data.reference_count.into()),
            HkbBoneWeightArrayVisitor::BoneWeights(data.bone_weights.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBoneWeightArray<'de> {
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
enum HkbBoneWeightArrayVisitor<'a> {
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
    #[serde(rename = "boneWeights")]
    BoneWeights(HkArrayNum<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBoneWeightArrayVisitor<'de>, "@name",
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("boneWeights" => BoneWeights(HkArrayNum<f32>)),
}
