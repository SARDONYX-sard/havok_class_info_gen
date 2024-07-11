//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSBoneSwitchGeneratorBoneData`
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

/// `BSBoneSwitchGeneratorBoneData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xc1215be6`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsBoneSwitchGeneratorBoneData<'a> {
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
    /// -   name:`"pGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"spBoneWeight"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub sp_bone_weight: Cow<'a, str>,
}

impl Serialize for BsBoneSwitchGeneratorBoneData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsBoneSwitchGeneratorBoneDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsBoneSwitchGeneratorBoneData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsBoneSwitchGeneratorBoneDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsBoneSwitchGeneratorBoneDataVisitor<'a>>> for BsBoneSwitchGeneratorBoneData<'a> {
    fn from(_values: Vec<BsBoneSwitchGeneratorBoneDataVisitor<'a>>) -> Self {
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut p_generator = None;
            let mut sp_bone_weight = None;


        for _value in _values {
            match _value {
                BsBoneSwitchGeneratorBoneDataVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::PGenerator(m) => p_generator = Some(m),
                BsBoneSwitchGeneratorBoneDataVisitor::SpBoneWeight(m) => sp_bone_weight = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            p_generator: p_generator.unwrap_or_default().into_inner(),
            sp_bone_weight: sp_bone_weight.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsBoneSwitchGeneratorBoneData<'a>> for Vec<BsBoneSwitchGeneratorBoneDataVisitor<'a>> {
    fn from(data: &BsBoneSwitchGeneratorBoneData<'a>) -> Self {
        vec![
            BsBoneSwitchGeneratorBoneDataVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsBoneSwitchGeneratorBoneDataVisitor::CachedBindables(data.cached_bindables.clone()),
            BsBoneSwitchGeneratorBoneDataVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsBoneSwitchGeneratorBoneDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsBoneSwitchGeneratorBoneDataVisitor::ReferenceCount(data.reference_count.into()),
            BsBoneSwitchGeneratorBoneDataVisitor::PGenerator(data.p_generator.clone().into()),
            BsBoneSwitchGeneratorBoneDataVisitor::SpBoneWeight(data.sp_bone_weight.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsBoneSwitchGeneratorBoneData<'de> {
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
enum BsBoneSwitchGeneratorBoneDataVisitor<'a> {
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
    #[serde(rename = "pGenerator")]
    PGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "spBoneWeight")]
    SpBoneWeight(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsBoneSwitchGeneratorBoneDataVisitor<'de>, "@name",
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("pGenerator" => PGenerator(Primitive<Cow<'de, str>>)),
    ("spBoneWeight" => SpBoneWeight(Primitive<Cow<'de, str>>)),
}
