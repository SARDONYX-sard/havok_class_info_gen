//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackfileSectionHeader`
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

/// `hkPackfileSectionHeader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xf2a92154`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkPackfileSectionHeader {
    /// # C++ Class Fields Info
    /// -   name:`"sectionTag"`
    /// -   type: `hkChar[19]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub section_tag: CStyleArray<[char; 19]>,
    /// # C++ Class Fields Info
    /// -   name:`"nullByte"`
    /// -   type: `hkChar`
    /// - offset: 19
    /// -  flags: `FLAGS_NONE`
    pub null_byte: char,
    /// # C++ Class Fields Info
    /// -   name:`"absoluteDataStart"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub absolute_data_start: i32,
    /// # C++ Class Fields Info
    /// -   name:`"localFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub local_fixups_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"globalFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub global_fixups_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"virtualFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub virtual_fixups_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"exportsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub exports_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"importsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub imports_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"endOffset"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub end_offset: i32,
}

impl Serialize for HkPackfileSectionHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkPackfileSectionHeaderVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkPackfileSectionHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkPackfileSectionHeaderVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkPackfileSectionHeaderVisitor>> for HkPackfileSectionHeader {
    fn from(_values: Vec<HkPackfileSectionHeaderVisitor>) -> Self {
            let mut section_tag = None;
            let mut null_byte = None;
            let mut absolute_data_start = None;
            let mut local_fixups_offset = None;
            let mut global_fixups_offset = None;
            let mut virtual_fixups_offset = None;
            let mut exports_offset = None;
            let mut imports_offset = None;
            let mut end_offset = None;


        for _value in _values {
            match _value {
                HkPackfileSectionHeaderVisitor::SectionTag(m) => section_tag = Some(m),
                HkPackfileSectionHeaderVisitor::NullByte(m) => null_byte = Some(m),
                HkPackfileSectionHeaderVisitor::AbsoluteDataStart(m) => absolute_data_start = Some(m),
                HkPackfileSectionHeaderVisitor::LocalFixupsOffset(m) => local_fixups_offset = Some(m),
                HkPackfileSectionHeaderVisitor::GlobalFixupsOffset(m) => global_fixups_offset = Some(m),
                HkPackfileSectionHeaderVisitor::VirtualFixupsOffset(m) => virtual_fixups_offset = Some(m),
                HkPackfileSectionHeaderVisitor::ExportsOffset(m) => exports_offset = Some(m),
                HkPackfileSectionHeaderVisitor::ImportsOffset(m) => imports_offset = Some(m),
                HkPackfileSectionHeaderVisitor::EndOffset(m) => end_offset = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            section_tag: section_tag.unwrap_or_default(),
            null_byte: null_byte.unwrap_or_default().into_inner(),
            absolute_data_start: absolute_data_start.unwrap_or_default().into_inner(),
            local_fixups_offset: local_fixups_offset.unwrap_or_default().into_inner(),
            global_fixups_offset: global_fixups_offset.unwrap_or_default().into_inner(),
            virtual_fixups_offset: virtual_fixups_offset.unwrap_or_default().into_inner(),
            exports_offset: exports_offset.unwrap_or_default().into_inner(),
            imports_offset: imports_offset.unwrap_or_default().into_inner(),
            end_offset: end_offset.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkPackfileSectionHeader> for Vec<HkPackfileSectionHeaderVisitor> {
    fn from(data: &HkPackfileSectionHeader) -> Self {
        vec![
            HkPackfileSectionHeaderVisitor::SectionTag(data.section_tag.clone()),
            HkPackfileSectionHeaderVisitor::NullByte(data.null_byte.into()),
            HkPackfileSectionHeaderVisitor::AbsoluteDataStart(data.absolute_data_start.into()),
            HkPackfileSectionHeaderVisitor::LocalFixupsOffset(data.local_fixups_offset.into()),
            HkPackfileSectionHeaderVisitor::GlobalFixupsOffset(data.global_fixups_offset.into()),
            HkPackfileSectionHeaderVisitor::VirtualFixupsOffset(data.virtual_fixups_offset.into()),
            HkPackfileSectionHeaderVisitor::ExportsOffset(data.exports_offset.into()),
            HkPackfileSectionHeaderVisitor::ImportsOffset(data.imports_offset.into()),
            HkPackfileSectionHeaderVisitor::EndOffset(data.end_offset.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkPackfileSectionHeader {
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
enum HkPackfileSectionHeaderVisitor {
    /// Visitor fields
    #[serde(rename = "sectionTag")]
    SectionTag(CStyleArray<[char; 19]>),
    /// Visitor fields
    #[serde(rename = "nullByte")]
    NullByte(Primitive<char>),
    /// Visitor fields
    #[serde(rename = "absoluteDataStart")]
    AbsoluteDataStart(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "localFixupsOffset")]
    LocalFixupsOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "globalFixupsOffset")]
    GlobalFixupsOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "virtualFixupsOffset")]
    VirtualFixupsOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "exportsOffset")]
    ExportsOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "importsOffset")]
    ImportsOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "endOffset")]
    EndOffset(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileSectionHeaderVisitor, "@name",
    ("sectionTag" => SectionTag(CStyleArray<[char; 19]>)),
    ("nullByte" => NullByte(Primitive<char>)),
    ("absoluteDataStart" => AbsoluteDataStart(Primitive<i32>)),
    ("localFixupsOffset" => LocalFixupsOffset(Primitive<i32>)),
    ("globalFixupsOffset" => GlobalFixupsOffset(Primitive<i32>)),
    ("virtualFixupsOffset" => VirtualFixupsOffset(Primitive<i32>)),
    ("exportsOffset" => ExportsOffset(Primitive<i32>)),
    ("importsOffset" => ImportsOffset(Primitive<i32>)),
    ("endOffset" => EndOffset(Primitive<i32>)),
}
