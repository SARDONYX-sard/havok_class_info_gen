//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipTrigger`
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

/// `hkbClipTrigger`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x7eb45cea`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbClipTrigger<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"relativeToEndOfClip"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub relative_to_end_of_clip: bool,
    /// # C++ Class Fields Info
    /// -   name:`"acyclic"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    pub acyclic: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isAnnotation"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    pub is_annotation: bool,
}

impl Serialize for HkbClipTrigger<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbClipTriggerVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbClipTrigger<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbClipTriggerVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbClipTriggerVisitor<'a>>> for HkbClipTrigger<'a> {
    fn from(_values: Vec<HkbClipTriggerVisitor<'a>>) -> Self {
            let mut local_time = None;
            let mut event = None;
            let mut relative_to_end_of_clip = None;
            let mut acyclic = None;
            let mut is_annotation = None;


        for _value in _values {
            match _value {
                HkbClipTriggerVisitor::LocalTime(m) => local_time = Some(m),
                HkbClipTriggerVisitor::Event(m) => event = Some(m),
                HkbClipTriggerVisitor::RelativeToEndOfClip(m) => relative_to_end_of_clip = Some(m),
                HkbClipTriggerVisitor::Acyclic(m) => acyclic = Some(m),
                HkbClipTriggerVisitor::IsAnnotation(m) => is_annotation = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            local_time: local_time.unwrap_or_default().into_inner(),
            event: event.unwrap_or_default(),
            relative_to_end_of_clip: relative_to_end_of_clip.unwrap_or_default().into_inner(),
            acyclic: acyclic.unwrap_or_default().into_inner(),
            is_annotation: is_annotation.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbClipTrigger<'a>> for Vec<HkbClipTriggerVisitor<'a>> {
    fn from(data: &HkbClipTrigger<'a>) -> Self {
        vec![
            HkbClipTriggerVisitor::LocalTime(data.local_time.into()),
            HkbClipTriggerVisitor::Event(data.event.clone()),
            HkbClipTriggerVisitor::RelativeToEndOfClip(data.relative_to_end_of_clip.into()),
            HkbClipTriggerVisitor::Acyclic(data.acyclic.into()),
            HkbClipTriggerVisitor::IsAnnotation(data.is_annotation.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbClipTrigger<'de> {
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
enum HkbClipTriggerVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "event")]
    Event(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "relativeToEndOfClip")]
    RelativeToEndOfClip(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "acyclic")]
    Acyclic(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isAnnotation")]
    IsAnnotation(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipTriggerVisitor<'de>, "@name",
    ("localTime" => LocalTime(Primitive<f32>)),
    ("event" => Event(SingleClass<HkbEventProperty<'de>>)),
    ("relativeToEndOfClip" => RelativeToEndOfClip(Primitive<bool>)),
    ("acyclic" => Acyclic(Primitive<bool>)),
    ("isAnnotation" => IsAnnotation(Primitive<bool>)),
}
