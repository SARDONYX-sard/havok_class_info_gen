//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialEffect`
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

/// `hkxMaterialEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1d39f925`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxMaterialEffect<'a> {
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
    /// -   type: `enum EffectType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub _type: EffectType,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
}

impl Serialize for HkxMaterialEffect<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxMaterialEffectVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxMaterialEffect<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxMaterialEffectVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxMaterialEffectVisitor<'a>>> for HkxMaterialEffect<'a> {
    fn from(_values: Vec<HkxMaterialEffectVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut _type = None;
            let mut data = None;


        for _value in _values {
            match _value {
                HkxMaterialEffectVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxMaterialEffectVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxMaterialEffectVisitor::Name(m) => name = Some(m),
                HkxMaterialEffectVisitor::Type(m) => _type = Some(m),
                HkxMaterialEffectVisitor::Data(m) => data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxMaterialEffect<'a>> for Vec<HkxMaterialEffectVisitor<'a>> {
    fn from(data: &HkxMaterialEffect<'a>) -> Self {
        vec![
            HkxMaterialEffectVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxMaterialEffectVisitor::ReferenceCount(data.reference_count.into()),
            HkxMaterialEffectVisitor::Name(data.name.clone().into()),
            HkxMaterialEffectVisitor::Type(data._type.clone().into()),
            HkxMaterialEffectVisitor::Data(data.data.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxMaterialEffect<'de> {
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
enum HkxMaterialEffectVisitor<'a> {
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
    Type(Primitive<EffectType>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialEffectVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<EffectType>)),
    ("data" => Data(HkArrayNum<u8>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EffectType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    #[default]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_INLINE")]
    EffectTypeHlslFxInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_FX_INLINE")]
    EffectTypeCgFxInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_FILENAME")]
    EffectTypeHlslFxFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FX_FILENAME")]
    EffectTypeCgFxFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
