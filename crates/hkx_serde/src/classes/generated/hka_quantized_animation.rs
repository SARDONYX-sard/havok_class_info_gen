//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaQuantizedAnimation`
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

/// `hkaQuantizedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x3920f053`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaQuantizedAnimation<'a> {
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
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"endian"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub endian: u32,
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub skeleton: Cow<'a, str>,
}

impl Serialize for HkaQuantizedAnimation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaQuantizedAnimationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaQuantizedAnimation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaQuantizedAnimationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaQuantizedAnimationVisitor<'a>>> for HkaQuantizedAnimation<'a> {
    fn from(_values: Vec<HkaQuantizedAnimationVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut duration = None;
            let mut number_of_transform_tracks = None;
            let mut number_of_float_tracks = None;
            let mut extracted_motion = None;
            let mut annotation_tracks = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut data = None;
            let mut endian = None;
            let mut skeleton = None;


        for _value in _values {
            match _value {
                HkaQuantizedAnimationVisitor::Type(m) => _type = Some(m),
                HkaQuantizedAnimationVisitor::Duration(m) => duration = Some(m),
                HkaQuantizedAnimationVisitor::NumberOfTransformTracks(m) => number_of_transform_tracks = Some(m),
                HkaQuantizedAnimationVisitor::NumberOfFloatTracks(m) => number_of_float_tracks = Some(m),
                HkaQuantizedAnimationVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkaQuantizedAnimationVisitor::AnnotationTracks(m) => annotation_tracks = Some(m),
                HkaQuantizedAnimationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaQuantizedAnimationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaQuantizedAnimationVisitor::Data(m) => data = Some(m),
                HkaQuantizedAnimationVisitor::Endian(m) => endian = Some(m),
                HkaQuantizedAnimationVisitor::Skeleton(m) => skeleton = Some(m),

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
            data: data.unwrap_or_default(),
            endian: endian.unwrap_or_default().into_inner(),
            skeleton: skeleton.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaQuantizedAnimation<'a>> for Vec<HkaQuantizedAnimationVisitor<'a>> {
    fn from(data: &HkaQuantizedAnimation<'a>) -> Self {
        vec![
            HkaQuantizedAnimationVisitor::Type(data._type.clone().into()),
            HkaQuantizedAnimationVisitor::Duration(data.duration.into()),
            HkaQuantizedAnimationVisitor::NumberOfTransformTracks(data.number_of_transform_tracks.into()),
            HkaQuantizedAnimationVisitor::NumberOfFloatTracks(data.number_of_float_tracks.into()),
            HkaQuantizedAnimationVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkaQuantizedAnimationVisitor::AnnotationTracks(data.annotation_tracks.clone()),
            HkaQuantizedAnimationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaQuantizedAnimationVisitor::ReferenceCount(data.reference_count.into()),
            HkaQuantizedAnimationVisitor::Data(data.data.clone()),
            HkaQuantizedAnimationVisitor::Endian(data.endian.into()),
            HkaQuantizedAnimationVisitor::Skeleton(data.skeleton.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaQuantizedAnimation<'de> {
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
enum HkaQuantizedAnimationVisitor<'a> {
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
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "endian")]
    Endian(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "skeleton", skip_serializing)]
    Skeleton(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaQuantizedAnimationVisitor<'de>, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("endian" => Endian(Primitive<u32>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
}
