//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSkinBinding`
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

/// `hkxSkinBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5a93f338`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxSkinBinding<'a> {
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
    /// -   name:`"nodeNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub node_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"bindPose"`
    /// -   type: `hkArray<hkMatrix4>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub bind_pose: HkArrayMatrix4<Matrix4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"initSkinTransform"`
    /// -   type: `hkMatrix4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub init_skin_transform: Matrix4<f32>,
}

impl Serialize for HkxSkinBinding<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxSkinBindingVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxSkinBinding<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxSkinBindingVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxSkinBindingVisitor<'a>>> for HkxSkinBinding<'a> {
    fn from(_values: Vec<HkxSkinBindingVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut mesh = None;
            let mut node_names = None;
            let mut bind_pose = None;
            let mut init_skin_transform = None;


        for _value in _values {
            match _value {
                HkxSkinBindingVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxSkinBindingVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxSkinBindingVisitor::Mesh(m) => mesh = Some(m),
                HkxSkinBindingVisitor::NodeNames(m) => node_names = Some(m),
                HkxSkinBindingVisitor::BindPose(m) => bind_pose = Some(m),
                HkxSkinBindingVisitor::InitSkinTransform(m) => init_skin_transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            mesh: mesh.unwrap_or_default().into_inner(),
            node_names: node_names.unwrap_or_default(),
            bind_pose: bind_pose.unwrap_or_default(),
            init_skin_transform: init_skin_transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxSkinBinding<'a>> for Vec<HkxSkinBindingVisitor<'a>> {
    fn from(data: &HkxSkinBinding<'a>) -> Self {
        vec![
            HkxSkinBindingVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxSkinBindingVisitor::ReferenceCount(data.reference_count.into()),
            HkxSkinBindingVisitor::Mesh(data.mesh.clone().into()),
            HkxSkinBindingVisitor::NodeNames(data.node_names.clone()),
            HkxSkinBindingVisitor::BindPose(data.bind_pose.clone()),
            HkxSkinBindingVisitor::InitSkinTransform(data.init_skin_transform.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxSkinBinding<'de> {
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
enum HkxSkinBindingVisitor<'a> {
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
    #[serde(rename = "nodeNames")]
    NodeNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "bindPose")]
    BindPose(HkArrayMatrix4<Matrix4<f32>>),
    /// Visitor fields
    #[serde(rename = "initSkinTransform")]
    InitSkinTransform(Primitive<Matrix4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSkinBindingVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("mesh" => Mesh(Primitive<Cow<'de, str>>)),
    ("nodeNames" => NodeNames(HkArrayStringPtr<'de>)),
    ("bindPose" => BindPose(HkArrayMatrix4<Matrix4<f32>>)),
    ("initSkinTransform" => InitSkinTransform(Primitive<Matrix4<f32>>)),
}
