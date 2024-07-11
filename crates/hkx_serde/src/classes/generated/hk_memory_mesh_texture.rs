//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshTexture`
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

/// `hkMemoryMeshTexture`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkMeshTexture`/`0xc9887918`
/// - signature: `0x2db6577c`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryMeshTexture<'a> {
    // C++ Parent class(`hkMeshTexture` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"filename"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub filename: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"format"`
    /// -   type: `enum Format`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub format: Format,
    /// # C++ Class Fields Info
    /// -   name:`"hasMipMaps"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    pub has_mip_maps: bool,
    /// # C++ Class Fields Info
    /// -   name:`"filterMode"`
    /// -   type: `enum FilterMode`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    pub filter_mode: FilterMode,
    /// # C++ Class Fields Info
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureUsageType`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    pub usage_hint: TextureUsageType,
    /// # C++ Class Fields Info
    /// -   name:`"textureCoordChannel"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub texture_coord_channel: i32,
}

impl Serialize for HkMemoryMeshTexture<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryMeshTextureVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryMeshTexture<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryMeshTextureVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMemoryMeshTextureVisitor<'a>>> for HkMemoryMeshTexture<'a> {
    fn from(_values: Vec<HkMemoryMeshTextureVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut filename = None;
            let mut data = None;
            let mut format = None;
            let mut has_mip_maps = None;
            let mut filter_mode = None;
            let mut usage_hint = None;
            let mut texture_coord_channel = None;


        for _value in _values {
            match _value {
                HkMemoryMeshTextureVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMemoryMeshTextureVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMemoryMeshTextureVisitor::Filename(m) => filename = Some(m),
                HkMemoryMeshTextureVisitor::Data(m) => data = Some(m),
                HkMemoryMeshTextureVisitor::Format(m) => format = Some(m),
                HkMemoryMeshTextureVisitor::HasMipMaps(m) => has_mip_maps = Some(m),
                HkMemoryMeshTextureVisitor::FilterMode(m) => filter_mode = Some(m),
                HkMemoryMeshTextureVisitor::UsageHint(m) => usage_hint = Some(m),
                HkMemoryMeshTextureVisitor::TextureCoordChannel(m) => texture_coord_channel = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            filename: filename.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default(),
            format: format.unwrap_or_default().into_inner(),
            has_mip_maps: has_mip_maps.unwrap_or_default().into_inner(),
            filter_mode: filter_mode.unwrap_or_default().into_inner(),
            usage_hint: usage_hint.unwrap_or_default().into_inner(),
            texture_coord_channel: texture_coord_channel.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMemoryMeshTexture<'a>> for Vec<HkMemoryMeshTextureVisitor<'a>> {
    fn from(data: &HkMemoryMeshTexture<'a>) -> Self {
        vec![
            HkMemoryMeshTextureVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMemoryMeshTextureVisitor::ReferenceCount(data.reference_count.into()),
            HkMemoryMeshTextureVisitor::Filename(data.filename.clone().into()),
            HkMemoryMeshTextureVisitor::Data(data.data.clone()),
            HkMemoryMeshTextureVisitor::Format(data.format.clone().into()),
            HkMemoryMeshTextureVisitor::HasMipMaps(data.has_mip_maps.into()),
            HkMemoryMeshTextureVisitor::FilterMode(data.filter_mode.clone().into()),
            HkMemoryMeshTextureVisitor::UsageHint(data.usage_hint.clone().into()),
            HkMemoryMeshTextureVisitor::TextureCoordChannel(data.texture_coord_channel.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryMeshTexture<'de> {
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
enum HkMemoryMeshTextureVisitor<'a> {
    // C++ Parent class(`hkMeshTexture` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "filename")]
    Filename(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "format")]
    Format(Primitive<Format>),
    /// Visitor fields
    #[serde(rename = "hasMipMaps")]
    HasMipMaps(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "filterMode")]
    FilterMode(Primitive<FilterMode>),
    /// Visitor fields
    #[serde(rename = "usageHint")]
    UsageHint(Primitive<TextureUsageType>),
    /// Visitor fields
    #[serde(rename = "textureCoordChannel")]
    TextureCoordChannel(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshTextureVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("filename" => Filename(Primitive<Cow<'de, str>>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("format" => Format(Primitive<Format>)),
    ("hasMipMaps" => HasMipMaps(Primitive<bool>)),
    ("filterMode" => FilterMode(Primitive<FilterMode>)),
    ("usageHint" => UsageHint(Primitive<TextureUsageType>)),
    ("textureCoordChannel" => TextureCoordChannel(Primitive<i32>)),
}
