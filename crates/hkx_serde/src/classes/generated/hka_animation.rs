//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaAnimation`
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

/// `hkaAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa6fa7e88`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaAnimation<'a> {
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
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: AnimationType,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub number_of_transform_tracks: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub number_of_float_tracks: i32,
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub extracted_motion: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray<struct hkaAnnotationTrack>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub annotation_tracks: HkArrayClass<HkaAnnotationTrack<'a>>,
}

impl Serialize for HkaAnimation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaAnimationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaAnimation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaAnimationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaAnimationVisitor<'a>>> for HkaAnimation<'a> {
    fn from(_values: Vec<HkaAnimationVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _type = None;
            let mut duration = None;
            let mut number_of_transform_tracks = None;
            let mut number_of_float_tracks = None;
            let mut extracted_motion = None;
            let mut annotation_tracks = None;


        for _value in _values {
            match _value {
                HkaAnimationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaAnimationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaAnimationVisitor::Type(m) => _type = Some(m),
                HkaAnimationVisitor::Duration(m) => duration = Some(m),
                HkaAnimationVisitor::NumberOfTransformTracks(m) => number_of_transform_tracks = Some(m),
                HkaAnimationVisitor::NumberOfFloatTracks(m) => number_of_float_tracks = Some(m),
                HkaAnimationVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkaAnimationVisitor::AnnotationTracks(m) => annotation_tracks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            number_of_transform_tracks: number_of_transform_tracks.unwrap_or_default().into_inner(),
            number_of_float_tracks: number_of_float_tracks.unwrap_or_default().into_inner(),
            extracted_motion: extracted_motion.unwrap_or_default().into_inner(),
            annotation_tracks: annotation_tracks.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaAnimation<'a>> for Vec<HkaAnimationVisitor<'a>> {
    fn from(data: &HkaAnimation<'a>) -> Self {
        vec![
            HkaAnimationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaAnimationVisitor::ReferenceCount(data.reference_count.into()),
            HkaAnimationVisitor::Type(data._type.clone().into()),
            HkaAnimationVisitor::Duration(data.duration.into()),
            HkaAnimationVisitor::NumberOfTransformTracks(data.number_of_transform_tracks.into()),
            HkaAnimationVisitor::NumberOfFloatTracks(data.number_of_float_tracks.into()),
            HkaAnimationVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkaAnimationVisitor::AnnotationTracks(data.annotation_tracks.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaAnimation<'de> {
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
enum HkaAnimationVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AnimationType>),
    /// Visitor fields
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numberOfTransformTracks")]
    NumberOfTransformTracks(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numberOfFloatTracks")]
    NumberOfFloatTracks(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "annotationTracks")]
    AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AnimationType {
    #[serde(rename = "HK_UNKNOWN_ANIMATION")]
    #[default]
    HkUnknownAnimation = 0,
    #[serde(rename = "HK_INTERLEAVED_ANIMATION")]
    HkInterleavedAnimation = 1,
    #[serde(rename = "HK_DELTA_COMPRESSED_ANIMATION")]
    HkDeltaCompressedAnimation = 2,
    #[serde(rename = "HK_WAVELET_COMPRESSED_ANIMATION")]
    HkWaveletCompressedAnimation = 3,
    #[serde(rename = "HK_MIRRORED_ANIMATION")]
    HkMirroredAnimation = 4,
    #[serde(rename = "HK_SPLINE_COMPRESSED_ANIMATION")]
    HkSplineCompressedAnimation = 5,
    #[serde(rename = "HK_QUANTIZED_COMPRESSED_ANIMATION")]
    HkQuantizedCompressedAnimation = 6,
}
