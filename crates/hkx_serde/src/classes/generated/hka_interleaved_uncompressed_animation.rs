//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaInterleavedUncompressedAnimation`
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

/// `hkaInterleavedUncompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x930af031`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaInterleavedUncompressedAnimation<'a> {
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
    /// -   name:`"transforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub transforms: HkArrayMatrix3<QsTransform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"floats"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub floats: HkArrayNum<f32>,
}

impl Serialize for HkaInterleavedUncompressedAnimation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaInterleavedUncompressedAnimationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaInterleavedUncompressedAnimation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaInterleavedUncompressedAnimationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaInterleavedUncompressedAnimationVisitor<'a>>> for HkaInterleavedUncompressedAnimation<'a> {
    fn from(_values: Vec<HkaInterleavedUncompressedAnimationVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut duration = None;
            let mut number_of_transform_tracks = None;
            let mut number_of_float_tracks = None;
            let mut extracted_motion = None;
            let mut annotation_tracks = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut transforms = None;
            let mut floats = None;


        for _value in _values {
            match _value {
                HkaInterleavedUncompressedAnimationVisitor::Type(m) => _type = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::Duration(m) => duration = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::NumberOfTransformTracks(m) => number_of_transform_tracks = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::NumberOfFloatTracks(m) => number_of_float_tracks = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::AnnotationTracks(m) => annotation_tracks = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::Transforms(m) => transforms = Some(m),
                HkaInterleavedUncompressedAnimationVisitor::Floats(m) => floats = Some(m),

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
            transforms: transforms.unwrap_or_default(),
            floats: floats.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaInterleavedUncompressedAnimation<'a>> for Vec<HkaInterleavedUncompressedAnimationVisitor<'a>> {
    fn from(data: &HkaInterleavedUncompressedAnimation<'a>) -> Self {
        vec![
            HkaInterleavedUncompressedAnimationVisitor::Type(data._type.clone().into()),
            HkaInterleavedUncompressedAnimationVisitor::Duration(data.duration.into()),
            HkaInterleavedUncompressedAnimationVisitor::NumberOfTransformTracks(data.number_of_transform_tracks.into()),
            HkaInterleavedUncompressedAnimationVisitor::NumberOfFloatTracks(data.number_of_float_tracks.into()),
            HkaInterleavedUncompressedAnimationVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkaInterleavedUncompressedAnimationVisitor::AnnotationTracks(data.annotation_tracks.clone()),
            HkaInterleavedUncompressedAnimationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaInterleavedUncompressedAnimationVisitor::ReferenceCount(data.reference_count.into()),
            HkaInterleavedUncompressedAnimationVisitor::Transforms(data.transforms.clone()),
            HkaInterleavedUncompressedAnimationVisitor::Floats(data.floats.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaInterleavedUncompressedAnimation<'de> {
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
enum HkaInterleavedUncompressedAnimationVisitor<'a> {
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
    #[serde(rename = "transforms")]
    Transforms(HkArrayMatrix3<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "floats")]
    Floats(HkArrayNum<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaInterleavedUncompressedAnimationVisitor<'de>, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transforms" => Transforms(HkArrayMatrix3<QsTransform<f32>>)),
    ("floats" => Floats(HkArrayNum<f32>)),
}
