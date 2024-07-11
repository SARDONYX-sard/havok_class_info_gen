//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbPoseMatchingGeneratorInternalState`
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

/// `hkbPoseMatchingGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x552d9dd4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbPoseMatchingGeneratorInternalState {
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
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub current_match: i32,
    /// # C++ Class Fields Info
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub best_match: i32,
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub time_since_better_match: f32,
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub reset_current_match_local_time: bool,
}

impl Serialize for HkbPoseMatchingGeneratorInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbPoseMatchingGeneratorInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbPoseMatchingGeneratorInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbPoseMatchingGeneratorInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbPoseMatchingGeneratorInternalStateVisitor>> for HkbPoseMatchingGeneratorInternalState {
    fn from(_values: Vec<HkbPoseMatchingGeneratorInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut current_match = None;
            let mut best_match = None;
            let mut time_since_better_match = None;
            let mut error = None;
            let mut reset_current_match_local_time = None;


        for _value in _values {
            match _value {
                HkbPoseMatchingGeneratorInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::CurrentMatch(m) => current_match = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::BestMatch(m) => best_match = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::TimeSinceBetterMatch(m) => time_since_better_match = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::Error(m) => error = Some(m),
                HkbPoseMatchingGeneratorInternalStateVisitor::ResetCurrentMatchLocalTime(m) => reset_current_match_local_time = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            current_match: current_match.unwrap_or_default().into_inner(),
            best_match: best_match.unwrap_or_default().into_inner(),
            time_since_better_match: time_since_better_match.unwrap_or_default().into_inner(),
            error: error.unwrap_or_default().into_inner(),
            reset_current_match_local_time: reset_current_match_local_time.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbPoseMatchingGeneratorInternalState> for Vec<HkbPoseMatchingGeneratorInternalStateVisitor> {
    fn from(data: &HkbPoseMatchingGeneratorInternalState) -> Self {
        vec![
            HkbPoseMatchingGeneratorInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::CurrentMatch(data.current_match.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::BestMatch(data.best_match.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::TimeSinceBetterMatch(data.time_since_better_match.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::Error(data.error.into()),
            HkbPoseMatchingGeneratorInternalStateVisitor::ResetCurrentMatchLocalTime(data.reset_current_match_local_time.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbPoseMatchingGeneratorInternalState {
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
enum HkbPoseMatchingGeneratorInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "currentMatch")]
    CurrentMatch(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "bestMatch")]
    BestMatch(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "timeSinceBetterMatch")]
    TimeSinceBetterMatch(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "error")]
    Error(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "resetCurrentMatchLocalTime")]
    ResetCurrentMatchLocalTime(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGeneratorInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
}
