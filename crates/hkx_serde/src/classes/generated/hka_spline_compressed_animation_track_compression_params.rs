//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimationTrackCompressionParams`
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

/// `hkaSplineCompressedAnimationTrackCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: false
/// - signature: `0x42e878d3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSplineCompressedAnimationTrackCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub rotation_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"translationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub translation_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub scale_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"floatingTolerance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub floating_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rotationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub rotation_degree: u16,
    /// # C++ Class Fields Info
    /// -   name:`"translationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    pub translation_degree: u16,
    /// # C++ Class Fields Info
    /// -   name:`"scaleDegree"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub scale_degree: u16,
    /// # C++ Class Fields Info
    /// -   name:`"floatingDegree"`
    /// -   type: `hkUint16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    pub floating_degree: u16,
    /// # C++ Class Fields Info
    /// -   name:`"rotationQuantizationType"`
    /// -   type: `enum RotationQuantization`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub rotation_quantization_type: RotationQuantization,
    /// # C++ Class Fields Info
    /// -   name:`"translationQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    pub translation_quantization_type: ScalarQuantization,
    /// # C++ Class Fields Info
    /// -   name:`"scaleQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    pub scale_quantization_type: ScalarQuantization,
    /// # C++ Class Fields Info
    /// -   name:`"floatQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    pub float_quantization_type: ScalarQuantization,
}

impl Serialize for HkaSplineCompressedAnimationTrackCompressionParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSplineCompressedAnimationTrackCompressionParamsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSplineCompressedAnimationTrackCompressionParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSplineCompressedAnimationTrackCompressionParamsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaSplineCompressedAnimationTrackCompressionParamsVisitor>> for HkaSplineCompressedAnimationTrackCompressionParams {
    fn from(_values: Vec<HkaSplineCompressedAnimationTrackCompressionParamsVisitor>) -> Self {
            let mut rotation_tolerance = None;
            let mut translation_tolerance = None;
            let mut scale_tolerance = None;
            let mut floating_tolerance = None;
            let mut rotation_degree = None;
            let mut translation_degree = None;
            let mut scale_degree = None;
            let mut floating_degree = None;
            let mut rotation_quantization_type = None;
            let mut translation_quantization_type = None;
            let mut scale_quantization_type = None;
            let mut float_quantization_type = None;


        for _value in _values {
            match _value {
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationTolerance(m) => rotation_tolerance = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationTolerance(m) => translation_tolerance = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleTolerance(m) => scale_tolerance = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatingTolerance(m) => floating_tolerance = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationDegree(m) => rotation_degree = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationDegree(m) => translation_degree = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleDegree(m) => scale_degree = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatingDegree(m) => floating_degree = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationQuantizationType(m) => rotation_quantization_type = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationQuantizationType(m) => translation_quantization_type = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleQuantizationType(m) => scale_quantization_type = Some(m),
                HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatQuantizationType(m) => float_quantization_type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rotation_tolerance: rotation_tolerance.unwrap_or_default().into_inner(),
            translation_tolerance: translation_tolerance.unwrap_or_default().into_inner(),
            scale_tolerance: scale_tolerance.unwrap_or_default().into_inner(),
            floating_tolerance: floating_tolerance.unwrap_or_default().into_inner(),
            rotation_degree: rotation_degree.unwrap_or_default().into_inner(),
            translation_degree: translation_degree.unwrap_or_default().into_inner(),
            scale_degree: scale_degree.unwrap_or_default().into_inner(),
            floating_degree: floating_degree.unwrap_or_default().into_inner(),
            rotation_quantization_type: rotation_quantization_type.unwrap_or_default().into_inner(),
            translation_quantization_type: translation_quantization_type.unwrap_or_default().into_inner(),
            scale_quantization_type: scale_quantization_type.unwrap_or_default().into_inner(),
            float_quantization_type: float_quantization_type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaSplineCompressedAnimationTrackCompressionParams> for Vec<HkaSplineCompressedAnimationTrackCompressionParamsVisitor> {
    fn from(data: &HkaSplineCompressedAnimationTrackCompressionParams) -> Self {
        vec![
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationTolerance(data.rotation_tolerance.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationTolerance(data.translation_tolerance.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleTolerance(data.scale_tolerance.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatingTolerance(data.floating_tolerance.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationDegree(data.rotation_degree.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationDegree(data.translation_degree.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleDegree(data.scale_degree.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatingDegree(data.floating_degree.into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::RotationQuantizationType(data.rotation_quantization_type.clone().into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::TranslationQuantizationType(data.translation_quantization_type.clone().into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::ScaleQuantizationType(data.scale_quantization_type.clone().into()),
            HkaSplineCompressedAnimationTrackCompressionParamsVisitor::FloatQuantizationType(data.float_quantization_type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSplineCompressedAnimationTrackCompressionParams {
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
enum HkaSplineCompressedAnimationTrackCompressionParamsVisitor {
    /// Visitor fields
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "translationTolerance")]
    TranslationTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "floatingTolerance")]
    FloatingTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rotationDegree")]
    RotationDegree(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "translationDegree")]
    TranslationDegree(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "scaleDegree")]
    ScaleDegree(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "floatingDegree")]
    FloatingDegree(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "rotationQuantizationType")]
    RotationQuantizationType(Primitive<RotationQuantization>),
    /// Visitor fields
    #[serde(rename = "translationQuantizationType")]
    TranslationQuantizationType(Primitive<ScalarQuantization>),
    /// Visitor fields
    #[serde(rename = "scaleQuantizationType")]
    ScaleQuantizationType(Primitive<ScalarQuantization>),
    /// Visitor fields
    #[serde(rename = "floatQuantizationType")]
    FloatQuantizationType(Primitive<ScalarQuantization>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationTrackCompressionParamsVisitor, "@name",
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("translationTolerance" => TranslationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("floatingTolerance" => FloatingTolerance(Primitive<f32>)),
    ("rotationDegree" => RotationDegree(Primitive<u16>)),
    ("translationDegree" => TranslationDegree(Primitive<u16>)),
    ("scaleDegree" => ScaleDegree(Primitive<u16>)),
    ("floatingDegree" => FloatingDegree(Primitive<u16>)),
    ("rotationQuantizationType" => RotationQuantizationType(Primitive<RotationQuantization>)),
    ("translationQuantizationType" => TranslationQuantizationType(Primitive<ScalarQuantization>)),
    ("scaleQuantizationType" => ScaleQuantizationType(Primitive<ScalarQuantization>)),
    ("floatQuantizationType" => FloatQuantizationType(Primitive<ScalarQuantization>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum RotationQuantization {
    #[serde(rename = "POLAR32")]
    #[default]
    Polar32 = 0,
    #[serde(rename = "THREECOMP40")]
    Threecomp40 = 1,
    #[serde(rename = "THREECOMP48")]
    Threecomp48 = 2,
    #[serde(rename = "THREECOMP24")]
    Threecomp24 = 3,
    #[serde(rename = "STRAIGHT16")]
    Straight16 = 4,
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed = 5,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ScalarQuantization {
    #[serde(rename = "BITS8")]
    #[default]
    Bits8 = 0,
    #[serde(rename = "BITS16")]
    Bits16 = 1,
}
