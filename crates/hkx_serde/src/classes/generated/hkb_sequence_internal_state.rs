//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSequenceInternalState`
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

/// `hkbSequenceInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x419b9a05`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSequenceInternalState {
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
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub next_sample_events: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub next_sample_reals: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub next_sample_bools: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub next_sample_ints: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: bool,
}

impl Serialize for HkbSequenceInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSequenceInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSequenceInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSequenceInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbSequenceInternalStateVisitor>> for HkbSequenceInternalState {
    fn from(_values: Vec<HkbSequenceInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut next_sample_events = None;
            let mut next_sample_reals = None;
            let mut next_sample_bools = None;
            let mut next_sample_ints = None;
            let mut time = None;
            let mut is_enabled = None;


        for _value in _values {
            match _value {
                HkbSequenceInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSequenceInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSequenceInternalStateVisitor::NextSampleEvents(m) => next_sample_events = Some(m),
                HkbSequenceInternalStateVisitor::NextSampleReals(m) => next_sample_reals = Some(m),
                HkbSequenceInternalStateVisitor::NextSampleBools(m) => next_sample_bools = Some(m),
                HkbSequenceInternalStateVisitor::NextSampleInts(m) => next_sample_ints = Some(m),
                HkbSequenceInternalStateVisitor::Time(m) => time = Some(m),
                HkbSequenceInternalStateVisitor::IsEnabled(m) => is_enabled = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            next_sample_events: next_sample_events.unwrap_or_default(),
            next_sample_reals: next_sample_reals.unwrap_or_default(),
            next_sample_bools: next_sample_bools.unwrap_or_default(),
            next_sample_ints: next_sample_ints.unwrap_or_default(),
            time: time.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbSequenceInternalState> for Vec<HkbSequenceInternalStateVisitor> {
    fn from(data: &HkbSequenceInternalState) -> Self {
        vec![
            HkbSequenceInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSequenceInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbSequenceInternalStateVisitor::NextSampleEvents(data.next_sample_events.clone()),
            HkbSequenceInternalStateVisitor::NextSampleReals(data.next_sample_reals.clone()),
            HkbSequenceInternalStateVisitor::NextSampleBools(data.next_sample_bools.clone()),
            HkbSequenceInternalStateVisitor::NextSampleInts(data.next_sample_ints.clone()),
            HkbSequenceInternalStateVisitor::Time(data.time.into()),
            HkbSequenceInternalStateVisitor::IsEnabled(data.is_enabled.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSequenceInternalState {
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
enum HkbSequenceInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "nextSampleEvents")]
    NextSampleEvents(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "nextSampleReals")]
    NextSampleReals(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "nextSampleBools")]
    NextSampleBools(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "nextSampleInts")]
    NextSampleInts(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequenceInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nextSampleEvents" => NextSampleEvents(HkArrayNum<i32>)),
    ("nextSampleReals" => NextSampleReals(HkArrayNum<i32>)),
    ("nextSampleBools" => NextSampleBools(HkArrayNum<i32>)),
    ("nextSampleInts" => NextSampleInts(HkArrayNum<i32>)),
    ("time" => Time(Primitive<f32>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
}
