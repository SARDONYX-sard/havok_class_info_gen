//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialShader`
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

/// `hkxMaterialShader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x28515eff`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxMaterialShader<'a> {
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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum ShaderType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub _type: ShaderType,
    /// # C++ Class Fields Info
    /// -   name:`"vertexEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub vertex_entry_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"geomEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub geom_entry_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pixelEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub pixel_entry_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
}

impl Serialize for HkxMaterialShader<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxMaterialShaderVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxMaterialShader<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxMaterialShaderVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxMaterialShaderVisitor<'a>>> for HkxMaterialShader<'a> {
    fn from(_values: Vec<HkxMaterialShaderVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut _type = None;
            let mut vertex_entry_name = None;
            let mut geom_entry_name = None;
            let mut pixel_entry_name = None;
            let mut data = None;


        for _value in _values {
            match _value {
                HkxMaterialShaderVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxMaterialShaderVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxMaterialShaderVisitor::Name(m) => name = Some(m),
                HkxMaterialShaderVisitor::Type(m) => _type = Some(m),
                HkxMaterialShaderVisitor::VertexEntryName(m) => vertex_entry_name = Some(m),
                HkxMaterialShaderVisitor::GeomEntryName(m) => geom_entry_name = Some(m),
                HkxMaterialShaderVisitor::PixelEntryName(m) => pixel_entry_name = Some(m),
                HkxMaterialShaderVisitor::Data(m) => data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            vertex_entry_name: vertex_entry_name.unwrap_or_default().into_inner(),
            geom_entry_name: geom_entry_name.unwrap_or_default().into_inner(),
            pixel_entry_name: pixel_entry_name.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxMaterialShader<'a>> for Vec<HkxMaterialShaderVisitor<'a>> {
    fn from(data: &HkxMaterialShader<'a>) -> Self {
        vec![
            HkxMaterialShaderVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxMaterialShaderVisitor::ReferenceCount(data.reference_count.into()),
            HkxMaterialShaderVisitor::Name(data.name.clone().into()),
            HkxMaterialShaderVisitor::Type(data._type.clone().into()),
            HkxMaterialShaderVisitor::VertexEntryName(data.vertex_entry_name.clone().into()),
            HkxMaterialShaderVisitor::GeomEntryName(data.geom_entry_name.clone().into()),
            HkxMaterialShaderVisitor::PixelEntryName(data.pixel_entry_name.clone().into()),
            HkxMaterialShaderVisitor::Data(data.data.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxMaterialShader<'de> {
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
enum HkxMaterialShaderVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<ShaderType>),
    /// Visitor fields
    #[serde(rename = "vertexEntryName")]
    VertexEntryName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "geomEntryName")]
    GeomEntryName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pixelEntryName")]
    PixelEntryName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialShaderVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<ShaderType>)),
    ("vertexEntryName" => VertexEntryName(Primitive<Cow<'de, str>>)),
    ("geomEntryName" => GeomEntryName(Primitive<Cow<'de, str>>)),
    ("pixelEntryName" => PixelEntryName(Primitive<Cow<'de, str>>)),
    ("data" => Data(HkArrayNum<u8>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ShaderType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    #[default]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_INLINE")]
    EffectTypeHlslInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_INLINE")]
    EffectTypeCgInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FILENAME")]
    EffectTypeHlslFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FILENAME")]
    EffectTypeCgFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
