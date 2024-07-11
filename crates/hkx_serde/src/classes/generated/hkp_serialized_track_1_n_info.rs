//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedTrack1nInfo`
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

/// `hkpSerializedTrack1nInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0xf12d48d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSerializedTrack1NInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sectors"`
    /// -   type: `hkArray<hkpAgent1nSector*>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub sectors: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"subTracks"`
    /// -   type: `hkArray<hkpSerializedSubTrack1nInfo*>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub sub_tracks: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpSerializedTrack1NInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSerializedTrack1NInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSerializedTrack1NInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSerializedTrack1NInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSerializedTrack1NInfoVisitor<'a>>> for HkpSerializedTrack1NInfo<'a> {
    fn from(_values: Vec<HkpSerializedTrack1NInfoVisitor<'a>>) -> Self {
            let mut sectors = None;
            let mut sub_tracks = None;


        for _value in _values {
            match _value {
                HkpSerializedTrack1NInfoVisitor::Sectors(m) => sectors = Some(m),
                HkpSerializedTrack1NInfoVisitor::SubTracks(m) => sub_tracks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            sectors: sectors.unwrap_or_default(),
            sub_tracks: sub_tracks.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSerializedTrack1NInfo<'a>> for Vec<HkpSerializedTrack1NInfoVisitor<'a>> {
    fn from(data: &HkpSerializedTrack1NInfo<'a>) -> Self {
        vec![
            HkpSerializedTrack1NInfoVisitor::Sectors(data.sectors.clone()),
            HkpSerializedTrack1NInfoVisitor::SubTracks(data.sub_tracks.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSerializedTrack1NInfo<'de> {
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
enum HkpSerializedTrack1NInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "sectors")]
    Sectors(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "subTracks")]
    SubTracks(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedTrack1NInfoVisitor<'de>, "@name",
    ("sectors" => Sectors(HkArrayRef<Cow<'de, str>>)),
    ("subTracks" => SubTracks(HkArrayRef<Cow<'de, str>>)),
}
