//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaWaveletCompressedAnimationQuantizationFormat`
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

/// `hkaWaveletCompressedAnimationQuantizationFormat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0x724a7561`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaWaveletCompressedAnimationQuantizationFormat {
    /// # C++ Class Fields Info
    /// -   name:`"maxBitWidth"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub max_bit_width: u8,
    /// # C++ Class Fields Info
    /// -   name:`"preserved"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub preserved: u8,
    /// # C++ Class Fields Info
    /// -   name:`"numD"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub num_d: u32,
    /// # C++ Class Fields Info
    /// -   name:`"offsetIdx"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub offset_idx: u32,
    /// # C++ Class Fields Info
    /// -   name:`"scaleIdx"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub scale_idx: u32,
    /// # C++ Class Fields Info
    /// -   name:`"bitWidthIdx"`
    /// -   type: `hkUint32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub bit_width_idx: u32,
}

impl Serialize for HkaWaveletCompressedAnimationQuantizationFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaWaveletCompressedAnimationQuantizationFormatVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaWaveletCompressedAnimationQuantizationFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaWaveletCompressedAnimationQuantizationFormatVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaWaveletCompressedAnimationQuantizationFormatVisitor>> for HkaWaveletCompressedAnimationQuantizationFormat {
    fn from(_values: Vec<HkaWaveletCompressedAnimationQuantizationFormatVisitor>) -> Self {
            let mut max_bit_width = None;
            let mut preserved = None;
            let mut num_d = None;
            let mut offset_idx = None;
            let mut scale_idx = None;
            let mut bit_width_idx = None;


        for _value in _values {
            match _value {
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::MaxBitWidth(m) => max_bit_width = Some(m),
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::Preserved(m) => preserved = Some(m),
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::NumD(m) => num_d = Some(m),
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::OffsetIdx(m) => offset_idx = Some(m),
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::ScaleIdx(m) => scale_idx = Some(m),
                HkaWaveletCompressedAnimationQuantizationFormatVisitor::BitWidthIdx(m) => bit_width_idx = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            max_bit_width: max_bit_width.unwrap_or_default().into_inner(),
            preserved: preserved.unwrap_or_default().into_inner(),
            num_d: num_d.unwrap_or_default().into_inner(),
            offset_idx: offset_idx.unwrap_or_default().into_inner(),
            scale_idx: scale_idx.unwrap_or_default().into_inner(),
            bit_width_idx: bit_width_idx.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaWaveletCompressedAnimationQuantizationFormat> for Vec<HkaWaveletCompressedAnimationQuantizationFormatVisitor> {
    fn from(data: &HkaWaveletCompressedAnimationQuantizationFormat) -> Self {
        vec![
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::MaxBitWidth(data.max_bit_width.into()),
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::Preserved(data.preserved.into()),
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::NumD(data.num_d.into()),
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::OffsetIdx(data.offset_idx.into()),
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::ScaleIdx(data.scale_idx.into()),
            HkaWaveletCompressedAnimationQuantizationFormatVisitor::BitWidthIdx(data.bit_width_idx.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaWaveletCompressedAnimationQuantizationFormat {
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
enum HkaWaveletCompressedAnimationQuantizationFormatVisitor {
    /// Visitor fields
    #[serde(rename = "maxBitWidth")]
    MaxBitWidth(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "preserved")]
    Preserved(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "numD")]
    NumD(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "offsetIdx")]
    OffsetIdx(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "scaleIdx")]
    ScaleIdx(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "bitWidthIdx")]
    BitWidthIdx(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationQuantizationFormatVisitor, "@name",
    ("maxBitWidth" => MaxBitWidth(Primitive<u8>)),
    ("preserved" => Preserved(Primitive<u8>)),
    ("numD" => NumD(Primitive<u32>)),
    ("offsetIdx" => OffsetIdx(Primitive<u32>)),
    ("scaleIdx" => ScaleIdx(Primitive<u32>)),
    ("bitWidthIdx" => BitWidthIdx(Primitive<u32>)),
}
