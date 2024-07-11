//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbMirroredSkeletonInfo`
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

/// `hkbMirroredSkeletonInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc6c2da4f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbMirroredSkeletonInfo {
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
    /// -   name:`"mirrorAxis"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub mirror_axis: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"bonePairMap"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bone_pair_map: HkArrayNum<i16>,
}

impl Serialize for HkbMirroredSkeletonInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbMirroredSkeletonInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbMirroredSkeletonInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbMirroredSkeletonInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbMirroredSkeletonInfoVisitor>> for HkbMirroredSkeletonInfo {
    fn from(_values: Vec<HkbMirroredSkeletonInfoVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut mirror_axis = None;
            let mut bone_pair_map = None;


        for _value in _values {
            match _value {
                HkbMirroredSkeletonInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbMirroredSkeletonInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbMirroredSkeletonInfoVisitor::MirrorAxis(m) => mirror_axis = Some(m),
                HkbMirroredSkeletonInfoVisitor::BonePairMap(m) => bone_pair_map = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            mirror_axis: mirror_axis.unwrap_or_default().into_inner(),
            bone_pair_map: bone_pair_map.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbMirroredSkeletonInfo> for Vec<HkbMirroredSkeletonInfoVisitor> {
    fn from(data: &HkbMirroredSkeletonInfo) -> Self {
        vec![
            HkbMirroredSkeletonInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbMirroredSkeletonInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbMirroredSkeletonInfoVisitor::MirrorAxis(data.mirror_axis.into()),
            HkbMirroredSkeletonInfoVisitor::BonePairMap(data.bone_pair_map.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbMirroredSkeletonInfo {
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
enum HkbMirroredSkeletonInfoVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "mirrorAxis")]
    MirrorAxis(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "bonePairMap")]
    BonePairMap(HkArrayNum<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMirroredSkeletonInfoVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("mirrorAxis" => MirrorAxis(Primitive<Vector4<f32>>)),
    ("bonePairMap" => BonePairMap(HkArrayNum<i16>)),
}
