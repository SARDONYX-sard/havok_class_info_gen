//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexDescription`
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

/// `hkxVertexDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2df6313d`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxVertexDescription {
    /// # C++ Class Fields Info
    /// -   name:`"decls"`
    /// -   type: `hkArray<struct hkxVertexDescriptionElementDecl>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub decls: HkArrayClass<HkxVertexDescriptionElementDecl>,
}

impl Serialize for HkxVertexDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxVertexDescriptionVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxVertexDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxVertexDescriptionVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxVertexDescriptionVisitor>> for HkxVertexDescription {
    fn from(_values: Vec<HkxVertexDescriptionVisitor>) -> Self {
            let mut decls = None;


        for _value in _values {
            match _value {
                HkxVertexDescriptionVisitor::Decls(m) => decls = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            decls: decls.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxVertexDescription> for Vec<HkxVertexDescriptionVisitor> {
    fn from(data: &HkxVertexDescription) -> Self {
        vec![
            HkxVertexDescriptionVisitor::Decls(data.decls.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxVertexDescription {
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
enum HkxVertexDescriptionVisitor {
    /// Visitor fields
    #[serde(rename = "decls")]
    Decls(HkArrayClass<HkxVertexDescriptionElementDecl>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionVisitor, "@name",
    ("decls" => Decls(HkArrayClass<HkxVertexDescriptionElementDecl>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum DataType {
    #[serde(rename = "HKX_DT_NONE")]
    #[default]
    HkxDtNone = 0,
    #[serde(rename = "HKX_DT_UINT8")]
    HkxDtUint8 = 1,
    #[serde(rename = "HKX_DT_INT16")]
    HkxDtInt16 = 2,
    #[serde(rename = "HKX_DT_UINT32")]
    HkxDtUint32 = 3,
    #[serde(rename = "HKX_DT_FLOAT")]
    HkxDtFloat = 4,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum DataUsage {
    #[serde(rename = "HKX_DU_NONE")]
    #[default]
    HkxDuNone = 0,
    #[serde(rename = "HKX_DU_POSITION")]
    HkxDuPosition = 1,
    #[serde(rename = "HKX_DU_COLOR")]
    HkxDuColor = 2,
    #[serde(rename = "HKX_DU_NORMAL")]
    HkxDuNormal = 4,
    #[serde(rename = "HKX_DU_TANGENT")]
    HkxDuTangent = 8,
    #[serde(rename = "HKX_DU_BINORMAL")]
    HkxDuBinormal = 16,
    #[serde(rename = "HKX_DU_TEXCOORD")]
    HkxDuTexcoord = 32,
    #[serde(rename = "HKX_DU_BLENDWEIGHTS")]
    HkxDuBlendweights = 64,
    #[serde(rename = "HKX_DU_BLENDINDICES")]
    HkxDuBlendindices = 128,
    #[serde(rename = "HKX_DU_USERDATA")]
    HkxDuUserdata = 256,
}
