//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeleton`
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

/// `hkaSkeleton`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x366e8220`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSkeleton<'a> {
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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"parentIndices"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub parent_indices: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `hkArray<struct hkaBone>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub bones: HkArrayClass<HkaBone<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"referencePose"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub reference_pose: HkArrayMatrix3<QsTransform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"referenceFloats"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub reference_floats: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"floatSlots"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub float_slots: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"localFrames"`
    /// -   type: `hkArray<struct hkaSkeletonLocalFrameOnBone>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub local_frames: HkArrayClass<HkaSkeletonLocalFrameOnBone<'a>>,
}

impl Serialize for HkaSkeleton<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSkeletonVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSkeleton<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSkeletonVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaSkeletonVisitor<'a>>> for HkaSkeleton<'a> {
    fn from(_values: Vec<HkaSkeletonVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut parent_indices = None;
            let mut bones = None;
            let mut reference_pose = None;
            let mut reference_floats = None;
            let mut float_slots = None;
            let mut local_frames = None;


        for _value in _values {
            match _value {
                HkaSkeletonVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaSkeletonVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaSkeletonVisitor::Name(m) => name = Some(m),
                HkaSkeletonVisitor::ParentIndices(m) => parent_indices = Some(m),
                HkaSkeletonVisitor::Bones(m) => bones = Some(m),
                HkaSkeletonVisitor::ReferencePose(m) => reference_pose = Some(m),
                HkaSkeletonVisitor::ReferenceFloats(m) => reference_floats = Some(m),
                HkaSkeletonVisitor::FloatSlots(m) => float_slots = Some(m),
                HkaSkeletonVisitor::LocalFrames(m) => local_frames = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            parent_indices: parent_indices.unwrap_or_default(),
            bones: bones.unwrap_or_default(),
            reference_pose: reference_pose.unwrap_or_default(),
            reference_floats: reference_floats.unwrap_or_default(),
            float_slots: float_slots.unwrap_or_default(),
            local_frames: local_frames.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaSkeleton<'a>> for Vec<HkaSkeletonVisitor<'a>> {
    fn from(data: &HkaSkeleton<'a>) -> Self {
        vec![
            HkaSkeletonVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaSkeletonVisitor::ReferenceCount(data.reference_count.into()),
            HkaSkeletonVisitor::Name(data.name.clone().into()),
            HkaSkeletonVisitor::ParentIndices(data.parent_indices.clone()),
            HkaSkeletonVisitor::Bones(data.bones.clone()),
            HkaSkeletonVisitor::ReferencePose(data.reference_pose.clone()),
            HkaSkeletonVisitor::ReferenceFloats(data.reference_floats.clone()),
            HkaSkeletonVisitor::FloatSlots(data.float_slots.clone()),
            HkaSkeletonVisitor::LocalFrames(data.local_frames.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSkeleton<'de> {
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
enum HkaSkeletonVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "parentIndices")]
    ParentIndices(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "bones")]
    Bones(HkArrayClass<HkaBone<'a>>),
    /// Visitor fields
    #[serde(rename = "referencePose")]
    ReferencePose(HkArrayMatrix3<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "referenceFloats")]
    ReferenceFloats(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "floatSlots")]
    FloatSlots(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "localFrames")]
    LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parentIndices" => ParentIndices(HkArrayNum<i16>)),
    ("bones" => Bones(HkArrayClass<HkaBone<'de>>)),
    ("referencePose" => ReferencePose(HkArrayMatrix3<QsTransform<f32>>)),
    ("referenceFloats" => ReferenceFloats(HkArrayNum<f32>)),
    ("floatSlots" => FloatSlots(HkArrayStringPtr<'de>)),
    ("localFrames" => LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone<'de>>)),
}
