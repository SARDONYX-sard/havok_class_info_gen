//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedSubTrack1nInfo`
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

/// `hkpSerializedSubTrack1nInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `hkpSerializedTrack1nInfo`/`0xf12d48d9`
/// - signature: `0x10155a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSerializedSubTrack1NInfo<'a> {
    /// # C++ Parent class(`hkpSerializedTrack1nInfo` => parent: `None`) field Info
    /// -   name:`"sectors"`
    /// -   type: `hkArray<hkpAgent1nSector*>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub sectors: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpSerializedTrack1nInfo` => parent: `None`) field Info
    /// -   name:`"subTracks"`
    /// -   type: `hkArray<hkpSerializedSubTrack1nInfo*>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub sub_tracks: HkArrayRef<Cow<'a, str>>,

    /// # C++ Class Fields Info
    /// -   name:`"sectorIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub sector_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"offsetInSector"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub offset_in_sector: i32,
}

impl Serialize for HkpSerializedSubTrack1NInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSerializedSubTrack1NInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSerializedSubTrack1NInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSerializedSubTrack1NInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSerializedSubTrack1NInfoVisitor<'a>>> for HkpSerializedSubTrack1NInfo<'a> {
    fn from(_values: Vec<HkpSerializedSubTrack1NInfoVisitor<'a>>) -> Self {
            let mut sectors = None;
            let mut sub_tracks = None;
            let mut sector_index = None;
            let mut offset_in_sector = None;


        for _value in _values {
            match _value {
                HkpSerializedSubTrack1NInfoVisitor::Sectors(m) => sectors = Some(m),
                HkpSerializedSubTrack1NInfoVisitor::SubTracks(m) => sub_tracks = Some(m),
                HkpSerializedSubTrack1NInfoVisitor::SectorIndex(m) => sector_index = Some(m),
                HkpSerializedSubTrack1NInfoVisitor::OffsetInSector(m) => offset_in_sector = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            sectors: sectors.unwrap_or_default(),
            sub_tracks: sub_tracks.unwrap_or_default(),
            sector_index: sector_index.unwrap_or_default().into_inner(),
            offset_in_sector: offset_in_sector.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSerializedSubTrack1NInfo<'a>> for Vec<HkpSerializedSubTrack1NInfoVisitor<'a>> {
    fn from(data: &HkpSerializedSubTrack1NInfo<'a>) -> Self {
        vec![
            HkpSerializedSubTrack1NInfoVisitor::Sectors(data.sectors.clone()),
            HkpSerializedSubTrack1NInfoVisitor::SubTracks(data.sub_tracks.clone()),
            HkpSerializedSubTrack1NInfoVisitor::SectorIndex(data.sector_index.into()),
            HkpSerializedSubTrack1NInfoVisitor::OffsetInSector(data.offset_in_sector.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSerializedSubTrack1NInfo<'de> {
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
enum HkpSerializedSubTrack1NInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "sectors")]
    Sectors(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "subTracks")]
    SubTracks(HkArrayRef<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "sectorIndex")]
    SectorIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "offsetInSector")]
    OffsetInSector(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedSubTrack1NInfoVisitor<'de>, "@name",
    ("sectors" => Sectors(HkArrayRef<Cow<'de, str>>)),
    ("subTracks" => SubTracks(HkArrayRef<Cow<'de, str>>)),
    ("sectorIndex" => SectorIndex(Primitive<i32>)),
    ("offsetInSector" => OffsetInSector(Primitive<i32>)),
}
