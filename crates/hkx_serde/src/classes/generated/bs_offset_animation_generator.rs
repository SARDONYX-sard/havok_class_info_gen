//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSOffsetAnimationGenerator`
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

/// `BSOffsetAnimationGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xb8571122`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsOffsetAnimationGenerator<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
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
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_default_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pOffsetClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_offset_clip_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetVariable"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub f_offset_variable: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeStart"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub f_offset_range_start: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeEnd"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub f_offset_range_end: f32,
    /// # C++ Class Fields Info
    /// -   name:`"BoneOffsetA"`
    /// -   type: `hkArray<void>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub bone_offset_a: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"BoneIndexA"`
    /// -   type: `hkArray<void>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub bone_index_a: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"fCurrentPercentage"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub f_current_percentage: f32,
    /// # C++ Class Fields Info
    /// -   name:`"iCurrentFrame"`
    /// -   type: `hkUint32`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub i_current_frame: u32,
    /// # C++ Class Fields Info
    /// -   name:`"bZeroOffset"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_zero_offset: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bOffsetValid"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_offset_valid: bool,
}

impl Serialize for BsOffsetAnimationGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsOffsetAnimationGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsOffsetAnimationGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsOffsetAnimationGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsOffsetAnimationGeneratorVisitor<'a>>> for BsOffsetAnimationGenerator<'a> {
    fn from(_values: Vec<BsOffsetAnimationGeneratorVisitor<'a>>) -> Self {
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
            let mut p_default_generator = None;
            let mut p_offset_clip_generator = None;
            let mut f_offset_variable = None;
            let mut f_offset_range_start = None;
            let mut f_offset_range_end = None;
            let mut bone_offset_a = None;
            let mut bone_index_a = None;
            let mut f_current_percentage = None;
            let mut i_current_frame = None;
            let mut b_zero_offset = None;
            let mut b_offset_valid = None;


        for _value in _values {
            match _value {
                BsOffsetAnimationGeneratorVisitor::UserData(m) => user_data = Some(m),
                BsOffsetAnimationGeneratorVisitor::Name(m) => name = Some(m),
                BsOffsetAnimationGeneratorVisitor::Id(m) => id = Some(m),
                BsOffsetAnimationGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                BsOffsetAnimationGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                BsOffsetAnimationGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsOffsetAnimationGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsOffsetAnimationGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsOffsetAnimationGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsOffsetAnimationGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsOffsetAnimationGeneratorVisitor::PDefaultGenerator(m) => p_default_generator = Some(m),
                BsOffsetAnimationGeneratorVisitor::POffsetClipGenerator(m) => p_offset_clip_generator = Some(m),
                BsOffsetAnimationGeneratorVisitor::FOffsetVariable(m) => f_offset_variable = Some(m),
                BsOffsetAnimationGeneratorVisitor::FOffsetRangeStart(m) => f_offset_range_start = Some(m),
                BsOffsetAnimationGeneratorVisitor::FOffsetRangeEnd(m) => f_offset_range_end = Some(m),
                BsOffsetAnimationGeneratorVisitor::BoneOffsetA(m) => bone_offset_a = Some(m),
                BsOffsetAnimationGeneratorVisitor::BoneIndexA(m) => bone_index_a = Some(m),
                BsOffsetAnimationGeneratorVisitor::FCurrentPercentage(m) => f_current_percentage = Some(m),
                BsOffsetAnimationGeneratorVisitor::ICurrentFrame(m) => i_current_frame = Some(m),
                BsOffsetAnimationGeneratorVisitor::BZeroOffset(m) => b_zero_offset = Some(m),
                BsOffsetAnimationGeneratorVisitor::BOffsetValid(m) => b_offset_valid = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
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
            p_default_generator: p_default_generator.unwrap_or_default().into_inner(),
            p_offset_clip_generator: p_offset_clip_generator.unwrap_or_default().into_inner(),
            f_offset_variable: f_offset_variable.unwrap_or_default().into_inner(),
            f_offset_range_start: f_offset_range_start.unwrap_or_default().into_inner(),
            f_offset_range_end: f_offset_range_end.unwrap_or_default().into_inner(),
            bone_offset_a: bone_offset_a.unwrap_or_default(),
            bone_index_a: bone_index_a.unwrap_or_default(),
            f_current_percentage: f_current_percentage.unwrap_or_default().into_inner(),
            i_current_frame: i_current_frame.unwrap_or_default().into_inner(),
            b_zero_offset: b_zero_offset.unwrap_or_default().into_inner(),
            b_offset_valid: b_offset_valid.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsOffsetAnimationGenerator<'a>> for Vec<BsOffsetAnimationGeneratorVisitor<'a>> {
    fn from(data: &BsOffsetAnimationGenerator<'a>) -> Self {
        vec![
            BsOffsetAnimationGeneratorVisitor::UserData(data.user_data.into()),
            BsOffsetAnimationGeneratorVisitor::Name(data.name.clone().into()),
            BsOffsetAnimationGeneratorVisitor::Id(data.id.into()),
            BsOffsetAnimationGeneratorVisitor::CloneState(data.clone_state.into()),
            BsOffsetAnimationGeneratorVisitor::PadNode(data.pad_node.clone()),
            BsOffsetAnimationGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsOffsetAnimationGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            BsOffsetAnimationGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsOffsetAnimationGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsOffsetAnimationGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            BsOffsetAnimationGeneratorVisitor::PDefaultGenerator(data.p_default_generator.clone().into()),
            BsOffsetAnimationGeneratorVisitor::POffsetClipGenerator(data.p_offset_clip_generator.clone().into()),
            BsOffsetAnimationGeneratorVisitor::FOffsetVariable(data.f_offset_variable.into()),
            BsOffsetAnimationGeneratorVisitor::FOffsetRangeStart(data.f_offset_range_start.into()),
            BsOffsetAnimationGeneratorVisitor::FOffsetRangeEnd(data.f_offset_range_end.into()),
            BsOffsetAnimationGeneratorVisitor::BoneOffsetA(data.bone_offset_a.clone()),
            BsOffsetAnimationGeneratorVisitor::BoneIndexA(data.bone_index_a.clone()),
            BsOffsetAnimationGeneratorVisitor::FCurrentPercentage(data.f_current_percentage.into()),
            BsOffsetAnimationGeneratorVisitor::ICurrentFrame(data.i_current_frame.into()),
            BsOffsetAnimationGeneratorVisitor::BZeroOffset(data.b_zero_offset.into()),
            BsOffsetAnimationGeneratorVisitor::BOffsetValid(data.b_offset_valid.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsOffsetAnimationGenerator<'de> {
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
enum BsOffsetAnimationGeneratorVisitor<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
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
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pOffsetClipGenerator")]
    POffsetClipGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "fOffsetVariable")]
    FOffsetVariable(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fOffsetRangeStart")]
    FOffsetRangeStart(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fOffsetRangeEnd")]
    FOffsetRangeEnd(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "BoneOffsetA", skip_serializing)]
    BoneOffsetA(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "BoneIndexA", skip_serializing)]
    BoneIndexA(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "fCurrentPercentage", skip_serializing)]
    FCurrentPercentage(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "iCurrentFrame", skip_serializing)]
    ICurrentFrame(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "bZeroOffset", skip_serializing)]
    BZeroOffset(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bOffsetValid", skip_serializing)]
    BOffsetValid(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsOffsetAnimationGeneratorVisitor<'de>, "@name",
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
    ("pDefaultGenerator" => PDefaultGenerator(Primitive<Cow<'de, str>>)),
    ("pOffsetClipGenerator" => POffsetClipGenerator(Primitive<Cow<'de, str>>)),
    ("fOffsetVariable" => FOffsetVariable(Primitive<f32>)),
    ("fOffsetRangeStart" => FOffsetRangeStart(Primitive<f32>)),
    ("fOffsetRangeEnd" => FOffsetRangeEnd(Primitive<f32>)),
    ("BoneOffsetA" => BoneOffsetA(HkArrayRef<()>)),
    ("BoneIndexA" => BoneIndexA(HkArrayRef<()>)),
    ("fCurrentPercentage" => FCurrentPercentage(Primitive<f32>)),
    ("iCurrentFrame" => ICurrentFrame(Primitive<u32>)),
    ("bZeroOffset" => BZeroOffset(Primitive<bool>)),
    ("bOffsetValid" => BOffsetValid(Primitive<bool>)),
}
