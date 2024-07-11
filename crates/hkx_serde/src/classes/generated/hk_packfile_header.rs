//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackfileHeader`
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

/// `hkPackfileHeader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x79f9ffda`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkPackfileHeader {
    /// # C++ Class Fields Info
    /// -   name:`"magic"`
    /// -   type: `hkInt32[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub magic: CStyleArray<[i32; 2]>,
    /// # C++ Class Fields Info
    /// -   name:`"userTag"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub user_tag: i32,
    /// # C++ Class Fields Info
    /// -   name:`"fileVersion"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub file_version: i32,
    /// # C++ Class Fields Info
    /// -   name:`"layoutRules"`
    /// -   type: `hkUint8[4]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub layout_rules: CStyleArray<[u8; 4]>,
    /// # C++ Class Fields Info
    /// -   name:`"numSections"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub num_sections: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contentsSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub contents_section_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contentsSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub contents_section_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contentsClassNameSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub contents_class_name_section_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contentsClassNameSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub contents_class_name_section_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contentsVersion"`
    /// -   type: `hkChar[16]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub contents_version: CStyleArray<[char; 16]>,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub flags: i32,
    /// # C++ Class Fields Info
    /// -   name:`"pad"`
    /// -   type: `hkInt32[1]`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub pad: CStyleArray<[i32; 1]>,
}

impl Serialize for HkPackfileHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkPackfileHeaderVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkPackfileHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkPackfileHeaderVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkPackfileHeaderVisitor>> for HkPackfileHeader {
    fn from(_values: Vec<HkPackfileHeaderVisitor>) -> Self {
            let mut magic = None;
            let mut user_tag = None;
            let mut file_version = None;
            let mut layout_rules = None;
            let mut num_sections = None;
            let mut contents_section_index = None;
            let mut contents_section_offset = None;
            let mut contents_class_name_section_index = None;
            let mut contents_class_name_section_offset = None;
            let mut contents_version = None;
            let mut flags = None;
            let mut pad = None;


        for _value in _values {
            match _value {
                HkPackfileHeaderVisitor::Magic(m) => magic = Some(m),
                HkPackfileHeaderVisitor::UserTag(m) => user_tag = Some(m),
                HkPackfileHeaderVisitor::FileVersion(m) => file_version = Some(m),
                HkPackfileHeaderVisitor::LayoutRules(m) => layout_rules = Some(m),
                HkPackfileHeaderVisitor::NumSections(m) => num_sections = Some(m),
                HkPackfileHeaderVisitor::ContentsSectionIndex(m) => contents_section_index = Some(m),
                HkPackfileHeaderVisitor::ContentsSectionOffset(m) => contents_section_offset = Some(m),
                HkPackfileHeaderVisitor::ContentsClassNameSectionIndex(m) => contents_class_name_section_index = Some(m),
                HkPackfileHeaderVisitor::ContentsClassNameSectionOffset(m) => contents_class_name_section_offset = Some(m),
                HkPackfileHeaderVisitor::ContentsVersion(m) => contents_version = Some(m),
                HkPackfileHeaderVisitor::Flags(m) => flags = Some(m),
                HkPackfileHeaderVisitor::Pad(m) => pad = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            magic: magic.unwrap_or_default(),
            user_tag: user_tag.unwrap_or_default().into_inner(),
            file_version: file_version.unwrap_or_default().into_inner(),
            layout_rules: layout_rules.unwrap_or_default(),
            num_sections: num_sections.unwrap_or_default().into_inner(),
            contents_section_index: contents_section_index.unwrap_or_default().into_inner(),
            contents_section_offset: contents_section_offset.unwrap_or_default().into_inner(),
            contents_class_name_section_index: contents_class_name_section_index.unwrap_or_default().into_inner(),
            contents_class_name_section_offset: contents_class_name_section_offset.unwrap_or_default().into_inner(),
            contents_version: contents_version.unwrap_or_default(),
            flags: flags.unwrap_or_default().into_inner(),
            pad: pad.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkPackfileHeader> for Vec<HkPackfileHeaderVisitor> {
    fn from(data: &HkPackfileHeader) -> Self {
        vec![
            HkPackfileHeaderVisitor::Magic(data.magic.clone()),
            HkPackfileHeaderVisitor::UserTag(data.user_tag.into()),
            HkPackfileHeaderVisitor::FileVersion(data.file_version.into()),
            HkPackfileHeaderVisitor::LayoutRules(data.layout_rules.clone()),
            HkPackfileHeaderVisitor::NumSections(data.num_sections.into()),
            HkPackfileHeaderVisitor::ContentsSectionIndex(data.contents_section_index.into()),
            HkPackfileHeaderVisitor::ContentsSectionOffset(data.contents_section_offset.into()),
            HkPackfileHeaderVisitor::ContentsClassNameSectionIndex(data.contents_class_name_section_index.into()),
            HkPackfileHeaderVisitor::ContentsClassNameSectionOffset(data.contents_class_name_section_offset.into()),
            HkPackfileHeaderVisitor::ContentsVersion(data.contents_version.clone()),
            HkPackfileHeaderVisitor::Flags(data.flags.into()),
            HkPackfileHeaderVisitor::Pad(data.pad.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkPackfileHeader {
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
enum HkPackfileHeaderVisitor {
    /// Visitor fields
    #[serde(rename = "magic")]
    Magic(CStyleArray<[i32; 2]>),
    /// Visitor fields
    #[serde(rename = "userTag")]
    UserTag(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "fileVersion")]
    FileVersion(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "layoutRules")]
    LayoutRules(CStyleArray<[u8; 4]>),
    /// Visitor fields
    #[serde(rename = "numSections")]
    NumSections(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contentsSectionIndex")]
    ContentsSectionIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contentsSectionOffset")]
    ContentsSectionOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contentsClassNameSectionIndex")]
    ContentsClassNameSectionIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contentsClassNameSectionOffset")]
    ContentsClassNameSectionOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contentsVersion")]
    ContentsVersion(CStyleArray<[char; 16]>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "pad")]
    Pad(CStyleArray<[i32; 1]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileHeaderVisitor, "@name",
    ("magic" => Magic(CStyleArray<[i32; 2]>)),
    ("userTag" => UserTag(Primitive<i32>)),
    ("fileVersion" => FileVersion(Primitive<i32>)),
    ("layoutRules" => LayoutRules(CStyleArray<[u8; 4]>)),
    ("numSections" => NumSections(Primitive<i32>)),
    ("contentsSectionIndex" => ContentsSectionIndex(Primitive<i32>)),
    ("contentsSectionOffset" => ContentsSectionOffset(Primitive<i32>)),
    ("contentsClassNameSectionIndex" => ContentsClassNameSectionIndex(Primitive<i32>)),
    ("contentsClassNameSectionOffset" => ContentsClassNameSectionOffset(Primitive<i32>)),
    ("contentsVersion" => ContentsVersion(CStyleArray<[char; 16]>)),
    ("flags" => Flags(Primitive<i32>)),
    ("pad" => Pad(CStyleArray<[i32; 1]>)),
}
