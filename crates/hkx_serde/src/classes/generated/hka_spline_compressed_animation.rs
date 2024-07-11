//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimation`
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

/// `hkaSplineCompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 132
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x792ee0bb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSplineCompressedAnimation<'a> {
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: AnimationType,
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub number_of_transform_tracks: i32,
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub number_of_float_tracks: i32,
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub extracted_motion: Cow<'a, str>,
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray<struct hkaAnnotationTrack>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub annotation_tracks: HkArrayClass<HkaAnnotationTrack<'a>>,

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
    /// -   name:`"numFrames"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub num_frames: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numBlocks"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub num_blocks: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxFramesPerBlock"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub max_frames_per_block: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maskAndQuantizationSize"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub mask_and_quantization_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"blockDuration"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub block_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"blockInverseDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub block_inverse_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"frameDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub frame_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"blockOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub block_offsets: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"floatBlockOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub float_block_offsets: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"transformOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub transform_offsets: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"floatOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub float_offsets: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"endian"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub endian: i32,
}

impl Serialize for HkaSplineCompressedAnimation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSplineCompressedAnimationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSplineCompressedAnimation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSplineCompressedAnimationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaSplineCompressedAnimationVisitor<'a>>> for HkaSplineCompressedAnimation<'a> {
    fn from(_values: Vec<HkaSplineCompressedAnimationVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut duration = None;
            let mut number_of_transform_tracks = None;
            let mut number_of_float_tracks = None;
            let mut extracted_motion = None;
            let mut annotation_tracks = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut num_frames = None;
            let mut num_blocks = None;
            let mut max_frames_per_block = None;
            let mut mask_and_quantization_size = None;
            let mut block_duration = None;
            let mut block_inverse_duration = None;
            let mut frame_duration = None;
            let mut block_offsets = None;
            let mut float_block_offsets = None;
            let mut transform_offsets = None;
            let mut float_offsets = None;
            let mut data = None;
            let mut endian = None;


        for _value in _values {
            match _value {
                HkaSplineCompressedAnimationVisitor::Type(m) => _type = Some(m),
                HkaSplineCompressedAnimationVisitor::Duration(m) => duration = Some(m),
                HkaSplineCompressedAnimationVisitor::NumberOfTransformTracks(m) => number_of_transform_tracks = Some(m),
                HkaSplineCompressedAnimationVisitor::NumberOfFloatTracks(m) => number_of_float_tracks = Some(m),
                HkaSplineCompressedAnimationVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkaSplineCompressedAnimationVisitor::AnnotationTracks(m) => annotation_tracks = Some(m),
                HkaSplineCompressedAnimationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaSplineCompressedAnimationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaSplineCompressedAnimationVisitor::NumFrames(m) => num_frames = Some(m),
                HkaSplineCompressedAnimationVisitor::NumBlocks(m) => num_blocks = Some(m),
                HkaSplineCompressedAnimationVisitor::MaxFramesPerBlock(m) => max_frames_per_block = Some(m),
                HkaSplineCompressedAnimationVisitor::MaskAndQuantizationSize(m) => mask_and_quantization_size = Some(m),
                HkaSplineCompressedAnimationVisitor::BlockDuration(m) => block_duration = Some(m),
                HkaSplineCompressedAnimationVisitor::BlockInverseDuration(m) => block_inverse_duration = Some(m),
                HkaSplineCompressedAnimationVisitor::FrameDuration(m) => frame_duration = Some(m),
                HkaSplineCompressedAnimationVisitor::BlockOffsets(m) => block_offsets = Some(m),
                HkaSplineCompressedAnimationVisitor::FloatBlockOffsets(m) => float_block_offsets = Some(m),
                HkaSplineCompressedAnimationVisitor::TransformOffsets(m) => transform_offsets = Some(m),
                HkaSplineCompressedAnimationVisitor::FloatOffsets(m) => float_offsets = Some(m),
                HkaSplineCompressedAnimationVisitor::Data(m) => data = Some(m),
                HkaSplineCompressedAnimationVisitor::Endian(m) => endian = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            number_of_transform_tracks: number_of_transform_tracks.unwrap_or_default().into_inner(),
            number_of_float_tracks: number_of_float_tracks.unwrap_or_default().into_inner(),
            extracted_motion: extracted_motion.unwrap_or_default().into_inner(),
            annotation_tracks: annotation_tracks.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            num_frames: num_frames.unwrap_or_default().into_inner(),
            num_blocks: num_blocks.unwrap_or_default().into_inner(),
            max_frames_per_block: max_frames_per_block.unwrap_or_default().into_inner(),
            mask_and_quantization_size: mask_and_quantization_size.unwrap_or_default().into_inner(),
            block_duration: block_duration.unwrap_or_default().into_inner(),
            block_inverse_duration: block_inverse_duration.unwrap_or_default().into_inner(),
            frame_duration: frame_duration.unwrap_or_default().into_inner(),
            block_offsets: block_offsets.unwrap_or_default(),
            float_block_offsets: float_block_offsets.unwrap_or_default(),
            transform_offsets: transform_offsets.unwrap_or_default(),
            float_offsets: float_offsets.unwrap_or_default(),
            data: data.unwrap_or_default(),
            endian: endian.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaSplineCompressedAnimation<'a>> for Vec<HkaSplineCompressedAnimationVisitor<'a>> {
    fn from(data: &HkaSplineCompressedAnimation<'a>) -> Self {
        vec![
            HkaSplineCompressedAnimationVisitor::Type(data._type.clone().into()),
            HkaSplineCompressedAnimationVisitor::Duration(data.duration.into()),
            HkaSplineCompressedAnimationVisitor::NumberOfTransformTracks(data.number_of_transform_tracks.into()),
            HkaSplineCompressedAnimationVisitor::NumberOfFloatTracks(data.number_of_float_tracks.into()),
            HkaSplineCompressedAnimationVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkaSplineCompressedAnimationVisitor::AnnotationTracks(data.annotation_tracks.clone()),
            HkaSplineCompressedAnimationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaSplineCompressedAnimationVisitor::ReferenceCount(data.reference_count.into()),
            HkaSplineCompressedAnimationVisitor::NumFrames(data.num_frames.into()),
            HkaSplineCompressedAnimationVisitor::NumBlocks(data.num_blocks.into()),
            HkaSplineCompressedAnimationVisitor::MaxFramesPerBlock(data.max_frames_per_block.into()),
            HkaSplineCompressedAnimationVisitor::MaskAndQuantizationSize(data.mask_and_quantization_size.into()),
            HkaSplineCompressedAnimationVisitor::BlockDuration(data.block_duration.into()),
            HkaSplineCompressedAnimationVisitor::BlockInverseDuration(data.block_inverse_duration.into()),
            HkaSplineCompressedAnimationVisitor::FrameDuration(data.frame_duration.into()),
            HkaSplineCompressedAnimationVisitor::BlockOffsets(data.block_offsets.clone()),
            HkaSplineCompressedAnimationVisitor::FloatBlockOffsets(data.float_block_offsets.clone()),
            HkaSplineCompressedAnimationVisitor::TransformOffsets(data.transform_offsets.clone()),
            HkaSplineCompressedAnimationVisitor::FloatOffsets(data.float_offsets.clone()),
            HkaSplineCompressedAnimationVisitor::Data(data.data.clone()),
            HkaSplineCompressedAnimationVisitor::Endian(data.endian.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSplineCompressedAnimation<'de> {
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
enum HkaSplineCompressedAnimationVisitor<'a> {
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

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "numFrames")]
    NumFrames(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numBlocks")]
    NumBlocks(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxFramesPerBlock")]
    MaxFramesPerBlock(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maskAndQuantizationSize")]
    MaskAndQuantizationSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "blockDuration")]
    BlockDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "blockInverseDuration")]
    BlockInverseDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "frameDuration")]
    FrameDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "blockOffsets")]
    BlockOffsets(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "floatBlockOffsets")]
    FloatBlockOffsets(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "transformOffsets")]
    TransformOffsets(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "floatOffsets")]
    FloatOffsets(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "endian")]
    Endian(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationVisitor<'de>, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("numFrames" => NumFrames(Primitive<i32>)),
    ("numBlocks" => NumBlocks(Primitive<i32>)),
    ("maxFramesPerBlock" => MaxFramesPerBlock(Primitive<i32>)),
    ("maskAndQuantizationSize" => MaskAndQuantizationSize(Primitive<i32>)),
    ("blockDuration" => BlockDuration(Primitive<f32>)),
    ("blockInverseDuration" => BlockInverseDuration(Primitive<f32>)),
    ("frameDuration" => FrameDuration(Primitive<f32>)),
    ("blockOffsets" => BlockOffsets(HkArrayNum<u32>)),
    ("floatBlockOffsets" => FloatBlockOffsets(HkArrayNum<u32>)),
    ("transformOffsets" => TransformOffsets(HkArrayNum<u32>)),
    ("floatOffsets" => FloatOffsets(HkArrayNum<u32>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("endian" => Endian(Primitive<i32>)),
}
