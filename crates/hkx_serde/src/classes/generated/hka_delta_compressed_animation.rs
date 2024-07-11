//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaDeltaCompressedAnimation`
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

/// `hkaDeltaCompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 120
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x90a68d40`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaDeltaCompressedAnimation<'a> {
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
    /// -   name:`"numberOfPoses"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub number_of_poses: i32,
    /// # C++ Class Fields Info
    /// -   name:`"blockSize"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub block_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"qFormat"`
    /// -   type: `struct hkaDeltaCompressedAnimationQuantizationFormat`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub q_format: SingleClass<HkaDeltaCompressedAnimationQuantizationFormat>,
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataIdx"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub quantized_data_idx: u32,
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub quantized_data_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"staticMaskIdx"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub static_mask_idx: u32,
    /// # C++ Class Fields Info
    /// -   name:`"staticMaskSize"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub static_mask_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"staticDOFsIdx"`
    /// -   type: `hkUint32`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub static_do_fs_idx: u32,
    /// # C++ Class Fields Info
    /// -   name:`"staticDOFsSize"`
    /// -   type: `hkUint32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub static_do_fs_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"numStaticTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub num_static_transform_do_fs: u32,
    /// # C++ Class Fields Info
    /// -   name:`"numDynamicTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub num_dynamic_transform_do_fs: u32,
    /// # C++ Class Fields Info
    /// -   name:`"totalBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub total_block_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"lastBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub last_block_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"dataBuffer"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub data_buffer: HkArrayNum<u8>,
}

impl Serialize for HkaDeltaCompressedAnimation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaDeltaCompressedAnimationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaDeltaCompressedAnimation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaDeltaCompressedAnimationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaDeltaCompressedAnimationVisitor<'a>>> for HkaDeltaCompressedAnimation<'a> {
    fn from(_values: Vec<HkaDeltaCompressedAnimationVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut duration = None;
            let mut number_of_transform_tracks = None;
            let mut number_of_float_tracks = None;
            let mut extracted_motion = None;
            let mut annotation_tracks = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut number_of_poses = None;
            let mut block_size = None;
            let mut q_format = None;
            let mut quantized_data_idx = None;
            let mut quantized_data_size = None;
            let mut static_mask_idx = None;
            let mut static_mask_size = None;
            let mut static_do_fs_idx = None;
            let mut static_do_fs_size = None;
            let mut num_static_transform_do_fs = None;
            let mut num_dynamic_transform_do_fs = None;
            let mut total_block_size = None;
            let mut last_block_size = None;
            let mut data_buffer = None;


        for _value in _values {
            match _value {
                HkaDeltaCompressedAnimationVisitor::Type(m) => _type = Some(m),
                HkaDeltaCompressedAnimationVisitor::Duration(m) => duration = Some(m),
                HkaDeltaCompressedAnimationVisitor::NumberOfTransformTracks(m) => number_of_transform_tracks = Some(m),
                HkaDeltaCompressedAnimationVisitor::NumberOfFloatTracks(m) => number_of_float_tracks = Some(m),
                HkaDeltaCompressedAnimationVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkaDeltaCompressedAnimationVisitor::AnnotationTracks(m) => annotation_tracks = Some(m),
                HkaDeltaCompressedAnimationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaDeltaCompressedAnimationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaDeltaCompressedAnimationVisitor::NumberOfPoses(m) => number_of_poses = Some(m),
                HkaDeltaCompressedAnimationVisitor::BlockSize(m) => block_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::QFormat(m) => q_format = Some(m),
                HkaDeltaCompressedAnimationVisitor::QuantizedDataIdx(m) => quantized_data_idx = Some(m),
                HkaDeltaCompressedAnimationVisitor::QuantizedDataSize(m) => quantized_data_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::StaticMaskIdx(m) => static_mask_idx = Some(m),
                HkaDeltaCompressedAnimationVisitor::StaticMaskSize(m) => static_mask_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::StaticDoFsIdx(m) => static_do_fs_idx = Some(m),
                HkaDeltaCompressedAnimationVisitor::StaticDoFsSize(m) => static_do_fs_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::NumStaticTransformDoFs(m) => num_static_transform_do_fs = Some(m),
                HkaDeltaCompressedAnimationVisitor::NumDynamicTransformDoFs(m) => num_dynamic_transform_do_fs = Some(m),
                HkaDeltaCompressedAnimationVisitor::TotalBlockSize(m) => total_block_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::LastBlockSize(m) => last_block_size = Some(m),
                HkaDeltaCompressedAnimationVisitor::DataBuffer(m) => data_buffer = Some(m),

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
            number_of_poses: number_of_poses.unwrap_or_default().into_inner(),
            block_size: block_size.unwrap_or_default().into_inner(),
            q_format: q_format.unwrap_or_default(),
            quantized_data_idx: quantized_data_idx.unwrap_or_default().into_inner(),
            quantized_data_size: quantized_data_size.unwrap_or_default().into_inner(),
            static_mask_idx: static_mask_idx.unwrap_or_default().into_inner(),
            static_mask_size: static_mask_size.unwrap_or_default().into_inner(),
            static_do_fs_idx: static_do_fs_idx.unwrap_or_default().into_inner(),
            static_do_fs_size: static_do_fs_size.unwrap_or_default().into_inner(),
            num_static_transform_do_fs: num_static_transform_do_fs.unwrap_or_default().into_inner(),
            num_dynamic_transform_do_fs: num_dynamic_transform_do_fs.unwrap_or_default().into_inner(),
            total_block_size: total_block_size.unwrap_or_default().into_inner(),
            last_block_size: last_block_size.unwrap_or_default().into_inner(),
            data_buffer: data_buffer.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaDeltaCompressedAnimation<'a>> for Vec<HkaDeltaCompressedAnimationVisitor<'a>> {
    fn from(data: &HkaDeltaCompressedAnimation<'a>) -> Self {
        vec![
            HkaDeltaCompressedAnimationVisitor::Type(data._type.clone().into()),
            HkaDeltaCompressedAnimationVisitor::Duration(data.duration.into()),
            HkaDeltaCompressedAnimationVisitor::NumberOfTransformTracks(data.number_of_transform_tracks.into()),
            HkaDeltaCompressedAnimationVisitor::NumberOfFloatTracks(data.number_of_float_tracks.into()),
            HkaDeltaCompressedAnimationVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkaDeltaCompressedAnimationVisitor::AnnotationTracks(data.annotation_tracks.clone()),
            HkaDeltaCompressedAnimationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaDeltaCompressedAnimationVisitor::ReferenceCount(data.reference_count.into()),
            HkaDeltaCompressedAnimationVisitor::NumberOfPoses(data.number_of_poses.into()),
            HkaDeltaCompressedAnimationVisitor::BlockSize(data.block_size.into()),
            HkaDeltaCompressedAnimationVisitor::QFormat(data.q_format.clone()),
            HkaDeltaCompressedAnimationVisitor::QuantizedDataIdx(data.quantized_data_idx.into()),
            HkaDeltaCompressedAnimationVisitor::QuantizedDataSize(data.quantized_data_size.into()),
            HkaDeltaCompressedAnimationVisitor::StaticMaskIdx(data.static_mask_idx.into()),
            HkaDeltaCompressedAnimationVisitor::StaticMaskSize(data.static_mask_size.into()),
            HkaDeltaCompressedAnimationVisitor::StaticDoFsIdx(data.static_do_fs_idx.into()),
            HkaDeltaCompressedAnimationVisitor::StaticDoFsSize(data.static_do_fs_size.into()),
            HkaDeltaCompressedAnimationVisitor::NumStaticTransformDoFs(data.num_static_transform_do_fs.into()),
            HkaDeltaCompressedAnimationVisitor::NumDynamicTransformDoFs(data.num_dynamic_transform_do_fs.into()),
            HkaDeltaCompressedAnimationVisitor::TotalBlockSize(data.total_block_size.into()),
            HkaDeltaCompressedAnimationVisitor::LastBlockSize(data.last_block_size.into()),
            HkaDeltaCompressedAnimationVisitor::DataBuffer(data.data_buffer.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaDeltaCompressedAnimation<'de> {
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
enum HkaDeltaCompressedAnimationVisitor<'a> {
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
    #[serde(rename = "numberOfPoses")]
    NumberOfPoses(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "blockSize")]
    BlockSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "qFormat")]
    QFormat(SingleClass<HkaDeltaCompressedAnimationQuantizationFormat>),
    /// Visitor fields
    #[serde(rename = "quantizedDataIdx")]
    QuantizedDataIdx(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "quantizedDataSize")]
    QuantizedDataSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "staticMaskIdx")]
    StaticMaskIdx(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "staticMaskSize")]
    StaticMaskSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "staticDOFsIdx")]
    StaticDoFsIdx(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "staticDOFsSize")]
    StaticDoFsSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "numStaticTransformDOFs")]
    NumStaticTransformDoFs(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "numDynamicTransformDOFs")]
    NumDynamicTransformDoFs(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "totalBlockSize")]
    TotalBlockSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "lastBlockSize")]
    LastBlockSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "dataBuffer")]
    DataBuffer(HkArrayNum<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaDeltaCompressedAnimationVisitor<'de>, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("numberOfPoses" => NumberOfPoses(Primitive<i32>)),
    ("blockSize" => BlockSize(Primitive<i32>)),
    ("qFormat" => QFormat(SingleClass<HkaDeltaCompressedAnimationQuantizationFormat>)),
    ("quantizedDataIdx" => QuantizedDataIdx(Primitive<u32>)),
    ("quantizedDataSize" => QuantizedDataSize(Primitive<u32>)),
    ("staticMaskIdx" => StaticMaskIdx(Primitive<u32>)),
    ("staticMaskSize" => StaticMaskSize(Primitive<u32>)),
    ("staticDOFsIdx" => StaticDoFsIdx(Primitive<u32>)),
    ("staticDOFsSize" => StaticDoFsSize(Primitive<u32>)),
    ("numStaticTransformDOFs" => NumStaticTransformDoFs(Primitive<u32>)),
    ("numDynamicTransformDOFs" => NumDynamicTransformDoFs(Primitive<u32>)),
    ("totalBlockSize" => TotalBlockSize(Primitive<u32>)),
    ("lastBlockSize" => LastBlockSize(Primitive<u32>)),
    ("dataBuffer" => DataBuffer(HkArrayNum<u8>)),
}
