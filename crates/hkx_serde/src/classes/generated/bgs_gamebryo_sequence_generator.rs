//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BGSGamebryoSequenceGenerator`
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

/// `BGSGamebryoSequenceGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xc8df2d77`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BgsGamebryoSequenceGenerator<'a> {
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
    /// -   name:`"pSequence"`
    /// -   type: `char*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub p_sequence: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eBlendModeFunction"`
    /// -   type: `enum BlendModeFunction`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub e_blend_mode_function: BlendModeFunction,
    /// # C++ Class Fields Info
    /// -   name:`"fPercent"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub f_percent: f32,
    /// # C++ Class Fields Info
    /// -   name:`"events"`
    /// -   type: `hkArray<void>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub events: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"fTime"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub f_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bDelayedActivate"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_delayed_activate: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bLooping"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_looping: bool,
}

impl Serialize for BgsGamebryoSequenceGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BgsGamebryoSequenceGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BgsGamebryoSequenceGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BgsGamebryoSequenceGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BgsGamebryoSequenceGeneratorVisitor<'a>>> for BgsGamebryoSequenceGenerator<'a> {
    fn from(_values: Vec<BgsGamebryoSequenceGeneratorVisitor<'a>>) -> Self {
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
            let mut p_sequence = None;
            let mut e_blend_mode_function = None;
            let mut f_percent = None;
            let mut events = None;
            let mut f_time = None;
            let mut b_delayed_activate = None;
            let mut b_looping = None;


        for _value in _values {
            match _value {
                BgsGamebryoSequenceGeneratorVisitor::UserData(m) => user_data = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::Name(m) => name = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::Id(m) => id = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::PSequence(m) => p_sequence = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::EBlendModeFunction(m) => e_blend_mode_function = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::FPercent(m) => f_percent = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::Events(m) => events = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::FTime(m) => f_time = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::BDelayedActivate(m) => b_delayed_activate = Some(m),
                BgsGamebryoSequenceGeneratorVisitor::BLooping(m) => b_looping = Some(m),

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
            p_sequence: p_sequence.unwrap_or_default().into_inner(),
            e_blend_mode_function: e_blend_mode_function.unwrap_or_default().into_inner(),
            f_percent: f_percent.unwrap_or_default().into_inner(),
            events: events.unwrap_or_default(),
            f_time: f_time.unwrap_or_default().into_inner(),
            b_delayed_activate: b_delayed_activate.unwrap_or_default().into_inner(),
            b_looping: b_looping.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BgsGamebryoSequenceGenerator<'a>> for Vec<BgsGamebryoSequenceGeneratorVisitor<'a>> {
    fn from(data: &BgsGamebryoSequenceGenerator<'a>) -> Self {
        vec![
            BgsGamebryoSequenceGeneratorVisitor::UserData(data.user_data.into()),
            BgsGamebryoSequenceGeneratorVisitor::Name(data.name.clone().into()),
            BgsGamebryoSequenceGeneratorVisitor::Id(data.id.into()),
            BgsGamebryoSequenceGeneratorVisitor::CloneState(data.clone_state.into()),
            BgsGamebryoSequenceGeneratorVisitor::PadNode(data.pad_node.clone()),
            BgsGamebryoSequenceGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BgsGamebryoSequenceGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            BgsGamebryoSequenceGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BgsGamebryoSequenceGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BgsGamebryoSequenceGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            BgsGamebryoSequenceGeneratorVisitor::PSequence(data.p_sequence.clone().into()),
            BgsGamebryoSequenceGeneratorVisitor::EBlendModeFunction(data.e_blend_mode_function.clone().into()),
            BgsGamebryoSequenceGeneratorVisitor::FPercent(data.f_percent.into()),
            BgsGamebryoSequenceGeneratorVisitor::Events(data.events.clone()),
            BgsGamebryoSequenceGeneratorVisitor::FTime(data.f_time.into()),
            BgsGamebryoSequenceGeneratorVisitor::BDelayedActivate(data.b_delayed_activate.into()),
            BgsGamebryoSequenceGeneratorVisitor::BLooping(data.b_looping.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BgsGamebryoSequenceGenerator<'de> {
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
enum BgsGamebryoSequenceGeneratorVisitor<'a> {
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
    #[serde(rename = "pSequence")]
    PSequence(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eBlendModeFunction")]
    EBlendModeFunction(Primitive<BlendModeFunction>),
    /// Visitor fields
    #[serde(rename = "fPercent")]
    FPercent(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "events", skip_serializing)]
    Events(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "fTime", skip_serializing)]
    FTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bDelayedActivate", skip_serializing)]
    BDelayedActivate(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bLooping", skip_serializing)]
    BLooping(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BgsGamebryoSequenceGeneratorVisitor<'de>, "@name",
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
    ("pSequence" => PSequence(Primitive<Cow<'de, str>>)),
    ("eBlendModeFunction" => EBlendModeFunction(Primitive<BlendModeFunction>)),
    ("fPercent" => FPercent(Primitive<f32>)),
    ("events" => Events(HkArrayRef<()>)),
    ("fTime" => FTime(Primitive<f32>)),
    ("bDelayedActivate" => BDelayedActivate(Primitive<bool>)),
    ("bLooping" => BLooping(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BlendModeFunction {
    #[serde(rename = "BMF_NONE")]
    #[default]
    BmfNone = 0,
    #[serde(rename = "BMF_PERCENT")]
    BmfPercent = 1,
    #[serde(rename = "BMF_ONE_MINUS_PERCENT")]
    BmfOneMinusPercent = 2,
}
