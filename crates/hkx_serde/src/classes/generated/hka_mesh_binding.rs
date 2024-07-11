//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaMeshBinding`
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

/// `hkaMeshBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x81d9950b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaMeshBinding<'a> {
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
    /// -   name:`"mesh"`
    /// -   type: `struct hkxMesh*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub mesh: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub original_skeleton_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub skeleton: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mappings"`
    /// -   type: `hkArray<struct hkaMeshBindingMapping>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub mappings: HkArrayClass<HkaMeshBindingMapping>,
    /// # C++ Class Fields Info
    /// -   name:`"boneFromSkinMeshTransforms"`
    /// -   type: `hkArray<hkTransform>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bone_from_skin_mesh_transforms: HkArrayMatrix4<Transform<f32>>,
}

impl Serialize for HkaMeshBinding<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaMeshBindingVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaMeshBinding<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaMeshBindingVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaMeshBindingVisitor<'a>>> for HkaMeshBinding<'a> {
    fn from(_values: Vec<HkaMeshBindingVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut mesh = None;
            let mut original_skeleton_name = None;
            let mut skeleton = None;
            let mut mappings = None;
            let mut bone_from_skin_mesh_transforms = None;


        for _value in _values {
            match _value {
                HkaMeshBindingVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaMeshBindingVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaMeshBindingVisitor::Mesh(m) => mesh = Some(m),
                HkaMeshBindingVisitor::OriginalSkeletonName(m) => original_skeleton_name = Some(m),
                HkaMeshBindingVisitor::Skeleton(m) => skeleton = Some(m),
                HkaMeshBindingVisitor::Mappings(m) => mappings = Some(m),
                HkaMeshBindingVisitor::BoneFromSkinMeshTransforms(m) => bone_from_skin_mesh_transforms = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            mesh: mesh.unwrap_or_default().into_inner(),
            original_skeleton_name: original_skeleton_name.unwrap_or_default().into_inner(),
            skeleton: skeleton.unwrap_or_default().into_inner(),
            mappings: mappings.unwrap_or_default(),
            bone_from_skin_mesh_transforms: bone_from_skin_mesh_transforms.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaMeshBinding<'a>> for Vec<HkaMeshBindingVisitor<'a>> {
    fn from(data: &HkaMeshBinding<'a>) -> Self {
        vec![
            HkaMeshBindingVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaMeshBindingVisitor::ReferenceCount(data.reference_count.into()),
            HkaMeshBindingVisitor::Mesh(data.mesh.clone().into()),
            HkaMeshBindingVisitor::OriginalSkeletonName(data.original_skeleton_name.clone().into()),
            HkaMeshBindingVisitor::Skeleton(data.skeleton.clone().into()),
            HkaMeshBindingVisitor::Mappings(data.mappings.clone()),
            HkaMeshBindingVisitor::BoneFromSkinMeshTransforms(data.bone_from_skin_mesh_transforms.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaMeshBinding<'de> {
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
enum HkaMeshBindingVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "mesh")]
    Mesh(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mappings")]
    Mappings(HkArrayClass<HkaMeshBindingMapping>),
    /// Visitor fields
    #[serde(rename = "boneFromSkinMeshTransforms")]
    BoneFromSkinMeshTransforms(HkArrayMatrix4<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaMeshBindingVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("mesh" => Mesh(Primitive<Cow<'de, str>>)),
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'de, str>>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
    ("mappings" => Mappings(HkArrayClass<HkaMeshBindingMapping>)),
    ("boneFromSkinMeshTransforms" => BoneFromSkinMeshTransforms(HkArrayMatrix4<Transform<f32>>)),
}
