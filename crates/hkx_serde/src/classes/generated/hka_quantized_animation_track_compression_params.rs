//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaQuantizedAnimationTrackCompressionParams`
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

/// `hkaQuantizedAnimationTrackCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xf7d64649`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaQuantizedAnimationTrackCompressionParams {
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
}

impl Serialize for HkaQuantizedAnimationTrackCompressionParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaQuantizedAnimationTrackCompressionParamsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaQuantizedAnimationTrackCompressionParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaQuantizedAnimationTrackCompressionParamsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaQuantizedAnimationTrackCompressionParamsVisitor>> for HkaQuantizedAnimationTrackCompressionParams {
    fn from(_values: Vec<HkaQuantizedAnimationTrackCompressionParamsVisitor>) -> Self {
            let mut rotation_tolerance = None;
            let mut translation_tolerance = None;
            let mut scale_tolerance = None;
            let mut floating_tolerance = None;


        for _value in _values {
            match _value {
                HkaQuantizedAnimationTrackCompressionParamsVisitor::RotationTolerance(m) => rotation_tolerance = Some(m),
                HkaQuantizedAnimationTrackCompressionParamsVisitor::TranslationTolerance(m) => translation_tolerance = Some(m),
                HkaQuantizedAnimationTrackCompressionParamsVisitor::ScaleTolerance(m) => scale_tolerance = Some(m),
                HkaQuantizedAnimationTrackCompressionParamsVisitor::FloatingTolerance(m) => floating_tolerance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rotation_tolerance: rotation_tolerance.unwrap_or_default().into_inner(),
            translation_tolerance: translation_tolerance.unwrap_or_default().into_inner(),
            scale_tolerance: scale_tolerance.unwrap_or_default().into_inner(),
            floating_tolerance: floating_tolerance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaQuantizedAnimationTrackCompressionParams> for Vec<HkaQuantizedAnimationTrackCompressionParamsVisitor> {
    fn from(data: &HkaQuantizedAnimationTrackCompressionParams) -> Self {
        vec![
            HkaQuantizedAnimationTrackCompressionParamsVisitor::RotationTolerance(data.rotation_tolerance.into()),
            HkaQuantizedAnimationTrackCompressionParamsVisitor::TranslationTolerance(data.translation_tolerance.into()),
            HkaQuantizedAnimationTrackCompressionParamsVisitor::ScaleTolerance(data.scale_tolerance.into()),
            HkaQuantizedAnimationTrackCompressionParamsVisitor::FloatingTolerance(data.floating_tolerance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaQuantizedAnimationTrackCompressionParams {
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
enum HkaQuantizedAnimationTrackCompressionParamsVisitor {
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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaQuantizedAnimationTrackCompressionParamsVisitor, "@name",
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("translationTolerance" => TranslationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("floatingTolerance" => FloatingTolerance(Primitive<f32>)),
}
