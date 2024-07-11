//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaRagdollInstance`
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

/// `hkaRagdollInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x154948e8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaRagdollInstance<'a> {
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
    /// -   name:`"rigidBodies"`
    /// -   type: `hkArray<hkpRigidBody*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rigid_bodies: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"constraints"`
    /// -   type: `hkArray<hkpConstraintInstance*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub constraints: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"boneToRigidBodyMap"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bone_to_rigid_body_map: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub skeleton: Cow<'a, str>,
}

impl Serialize for HkaRagdollInstance<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaRagdollInstanceVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaRagdollInstance<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaRagdollInstanceVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaRagdollInstanceVisitor<'a>>> for HkaRagdollInstance<'a> {
    fn from(_values: Vec<HkaRagdollInstanceVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rigid_bodies = None;
            let mut constraints = None;
            let mut bone_to_rigid_body_map = None;
            let mut skeleton = None;


        for _value in _values {
            match _value {
                HkaRagdollInstanceVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaRagdollInstanceVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaRagdollInstanceVisitor::RigidBodies(m) => rigid_bodies = Some(m),
                HkaRagdollInstanceVisitor::Constraints(m) => constraints = Some(m),
                HkaRagdollInstanceVisitor::BoneToRigidBodyMap(m) => bone_to_rigid_body_map = Some(m),
                HkaRagdollInstanceVisitor::Skeleton(m) => skeleton = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rigid_bodies: rigid_bodies.unwrap_or_default(),
            constraints: constraints.unwrap_or_default(),
            bone_to_rigid_body_map: bone_to_rigid_body_map.unwrap_or_default(),
            skeleton: skeleton.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaRagdollInstance<'a>> for Vec<HkaRagdollInstanceVisitor<'a>> {
    fn from(data: &HkaRagdollInstance<'a>) -> Self {
        vec![
            HkaRagdollInstanceVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaRagdollInstanceVisitor::ReferenceCount(data.reference_count.into()),
            HkaRagdollInstanceVisitor::RigidBodies(data.rigid_bodies.clone()),
            HkaRagdollInstanceVisitor::Constraints(data.constraints.clone()),
            HkaRagdollInstanceVisitor::BoneToRigidBodyMap(data.bone_to_rigid_body_map.clone()),
            HkaRagdollInstanceVisitor::Skeleton(data.skeleton.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaRagdollInstance<'de> {
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
enum HkaRagdollInstanceVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "rigidBodies")]
    RigidBodies(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraints")]
    Constraints(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "boneToRigidBodyMap")]
    BoneToRigidBodyMap(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaRagdollInstanceVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rigidBodies" => RigidBodies(HkArrayRef<Cow<'de, str>>)),
    ("constraints" => Constraints(HkArrayRef<Cow<'de, str>>)),
    ("boneToRigidBodyMap" => BoneToRigidBodyMap(HkArrayNum<i32>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
}
