//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaWaveletCompressedAnimationCompressionParams`
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

/// `hkaWaveletCompressedAnimationCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0x27c6cafa`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaWaveletCompressedAnimationCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"quantizationBits"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub quantization_bits: u16,
    /// # C++ Class Fields Info
    /// -   name:`"blockSize"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub block_size: u16,
    /// # C++ Class Fields Info
    /// -   name:`"preserve"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub preserve: u16,
    /// # C++ Class Fields Info
    /// -   name:`"truncProp"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub trunc_prop: f32,
    /// # C++ Class Fields Info
    /// -   name:`"useOldStyleTruncation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub use_old_style_truncation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"absolutePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub absolute_position_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"relativePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub relative_position_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub rotation_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub scale_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"absoluteFloatTolerance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub absolute_float_tolerance: f32,
}

impl Serialize for HkaWaveletCompressedAnimationCompressionParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaWaveletCompressedAnimationCompressionParamsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaWaveletCompressedAnimationCompressionParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaWaveletCompressedAnimationCompressionParamsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaWaveletCompressedAnimationCompressionParamsVisitor>> for HkaWaveletCompressedAnimationCompressionParams {
    fn from(_values: Vec<HkaWaveletCompressedAnimationCompressionParamsVisitor>) -> Self {
            let mut quantization_bits = None;
            let mut block_size = None;
            let mut preserve = None;
            let mut trunc_prop = None;
            let mut use_old_style_truncation = None;
            let mut absolute_position_tolerance = None;
            let mut relative_position_tolerance = None;
            let mut rotation_tolerance = None;
            let mut scale_tolerance = None;
            let mut absolute_float_tolerance = None;


        for _value in _values {
            match _value {
                HkaWaveletCompressedAnimationCompressionParamsVisitor::QuantizationBits(m) => quantization_bits = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::BlockSize(m) => block_size = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::Preserve(m) => preserve = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::TruncProp(m) => trunc_prop = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::UseOldStyleTruncation(m) => use_old_style_truncation = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::AbsolutePositionTolerance(m) => absolute_position_tolerance = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::RelativePositionTolerance(m) => relative_position_tolerance = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::RotationTolerance(m) => rotation_tolerance = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::ScaleTolerance(m) => scale_tolerance = Some(m),
                HkaWaveletCompressedAnimationCompressionParamsVisitor::AbsoluteFloatTolerance(m) => absolute_float_tolerance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            quantization_bits: quantization_bits.unwrap_or_default().into_inner(),
            block_size: block_size.unwrap_or_default().into_inner(),
            preserve: preserve.unwrap_or_default().into_inner(),
            trunc_prop: trunc_prop.unwrap_or_default().into_inner(),
            use_old_style_truncation: use_old_style_truncation.unwrap_or_default().into_inner(),
            absolute_position_tolerance: absolute_position_tolerance.unwrap_or_default().into_inner(),
            relative_position_tolerance: relative_position_tolerance.unwrap_or_default().into_inner(),
            rotation_tolerance: rotation_tolerance.unwrap_or_default().into_inner(),
            scale_tolerance: scale_tolerance.unwrap_or_default().into_inner(),
            absolute_float_tolerance: absolute_float_tolerance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaWaveletCompressedAnimationCompressionParams> for Vec<HkaWaveletCompressedAnimationCompressionParamsVisitor> {
    fn from(data: &HkaWaveletCompressedAnimationCompressionParams) -> Self {
        vec![
            HkaWaveletCompressedAnimationCompressionParamsVisitor::QuantizationBits(data.quantization_bits.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::BlockSize(data.block_size.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::Preserve(data.preserve.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::TruncProp(data.trunc_prop.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::UseOldStyleTruncation(data.use_old_style_truncation.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::AbsolutePositionTolerance(data.absolute_position_tolerance.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::RelativePositionTolerance(data.relative_position_tolerance.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::RotationTolerance(data.rotation_tolerance.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::ScaleTolerance(data.scale_tolerance.into()),
            HkaWaveletCompressedAnimationCompressionParamsVisitor::AbsoluteFloatTolerance(data.absolute_float_tolerance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaWaveletCompressedAnimationCompressionParams {
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
enum HkaWaveletCompressedAnimationCompressionParamsVisitor {
    /// Visitor fields
    #[serde(rename = "quantizationBits")]
    QuantizationBits(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "blockSize")]
    BlockSize(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "preserve")]
    Preserve(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "truncProp")]
    TruncProp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "useOldStyleTruncation")]
    UseOldStyleTruncation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "absolutePositionTolerance")]
    AbsolutePositionTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "relativePositionTolerance")]
    RelativePositionTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "absoluteFloatTolerance")]
    AbsoluteFloatTolerance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationCompressionParamsVisitor, "@name",
    ("quantizationBits" => QuantizationBits(Primitive<u16>)),
    ("blockSize" => BlockSize(Primitive<u16>)),
    ("preserve" => Preserve(Primitive<u16>)),
    ("truncProp" => TruncProp(Primitive<f32>)),
    ("useOldStyleTruncation" => UseOldStyleTruncation(Primitive<bool>)),
    ("absolutePositionTolerance" => AbsolutePositionTolerance(Primitive<f32>)),
    ("relativePositionTolerance" => RelativePositionTolerance(Primitive<f32>)),
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("absoluteFloatTolerance" => AbsoluteFloatTolerance(Primitive<f32>)),
}
