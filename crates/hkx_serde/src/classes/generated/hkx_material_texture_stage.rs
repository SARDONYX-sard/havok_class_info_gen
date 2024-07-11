//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialTextureStage`
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

/// `hkxMaterialTextureStage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xfa6facb2`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxMaterialTextureStage<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"texture"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub texture: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub usage_hint: TextureType,
    /// # C++ Class Fields Info
    /// -   name:`"tcoordChannel"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub tcoord_channel: i32,
}

impl Serialize for HkxMaterialTextureStage<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxMaterialTextureStageVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxMaterialTextureStage<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxMaterialTextureStageVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxMaterialTextureStageVisitor<'a>>> for HkxMaterialTextureStage<'a> {
    fn from(_values: Vec<HkxMaterialTextureStageVisitor<'a>>) -> Self {
            let mut texture = None;
            let mut usage_hint = None;
            let mut tcoord_channel = None;


        for _value in _values {
            match _value {
                HkxMaterialTextureStageVisitor::Texture(m) => texture = Some(m),
                HkxMaterialTextureStageVisitor::UsageHint(m) => usage_hint = Some(m),
                HkxMaterialTextureStageVisitor::TcoordChannel(m) => tcoord_channel = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            texture: texture.unwrap_or_default().into_inner(),
            usage_hint: usage_hint.unwrap_or_default().into_inner(),
            tcoord_channel: tcoord_channel.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxMaterialTextureStage<'a>> for Vec<HkxMaterialTextureStageVisitor<'a>> {
    fn from(data: &HkxMaterialTextureStage<'a>) -> Self {
        vec![
            HkxMaterialTextureStageVisitor::Texture(data.texture.clone().into()),
            HkxMaterialTextureStageVisitor::UsageHint(data.usage_hint.clone().into()),
            HkxMaterialTextureStageVisitor::TcoordChannel(data.tcoord_channel.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxMaterialTextureStage<'de> {
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
enum HkxMaterialTextureStageVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "texture")]
    Texture(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "usageHint")]
    UsageHint(Primitive<TextureType>),
    /// Visitor fields
    #[serde(rename = "tcoordChannel")]
    TcoordChannel(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialTextureStageVisitor<'de>, "@name",
    ("texture" => Texture(Primitive<Cow<'de, str>>)),
    ("usageHint" => UsageHint(Primitive<TextureType>)),
    ("tcoordChannel" => TcoordChannel(Primitive<i32>)),
}
