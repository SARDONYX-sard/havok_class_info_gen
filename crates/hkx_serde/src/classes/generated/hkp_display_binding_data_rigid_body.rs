//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpDisplayBindingDataRigidBody`
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

/// `hkpDisplayBindingDataRigidBody`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xfe16e2a3`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpDisplayBindingDataRigidBody<'a> {
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
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rigid_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"displayObjectPtr"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub display_object_ptr: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rigidBodyFromDisplayObjectTransform"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub rigid_body_from_display_object_transform: Matrix4<f32>,
}

impl Serialize for HkpDisplayBindingDataRigidBody<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpDisplayBindingDataRigidBodyVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpDisplayBindingDataRigidBody<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpDisplayBindingDataRigidBodyVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpDisplayBindingDataRigidBodyVisitor<'a>>> for HkpDisplayBindingDataRigidBody<'a> {
    fn from(_values: Vec<HkpDisplayBindingDataRigidBodyVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rigid_body = None;
            let mut display_object_ptr = None;
            let mut rigid_body_from_display_object_transform = None;


        for _value in _values {
            match _value {
                HkpDisplayBindingDataRigidBodyVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpDisplayBindingDataRigidBodyVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpDisplayBindingDataRigidBodyVisitor::RigidBody(m) => rigid_body = Some(m),
                HkpDisplayBindingDataRigidBodyVisitor::DisplayObjectPtr(m) => display_object_ptr = Some(m),
                HkpDisplayBindingDataRigidBodyVisitor::RigidBodyFromDisplayObjectTransform(m) => rigid_body_from_display_object_transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rigid_body: rigid_body.unwrap_or_default().into_inner(),
            display_object_ptr: display_object_ptr.unwrap_or_default().into_inner(),
            rigid_body_from_display_object_transform: rigid_body_from_display_object_transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpDisplayBindingDataRigidBody<'a>> for Vec<HkpDisplayBindingDataRigidBodyVisitor<'a>> {
    fn from(data: &HkpDisplayBindingDataRigidBody<'a>) -> Self {
        vec![
            HkpDisplayBindingDataRigidBodyVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpDisplayBindingDataRigidBodyVisitor::ReferenceCount(data.reference_count.into()),
            HkpDisplayBindingDataRigidBodyVisitor::RigidBody(data.rigid_body.clone().into()),
            HkpDisplayBindingDataRigidBodyVisitor::DisplayObjectPtr(data.display_object_ptr.clone().into()),
            HkpDisplayBindingDataRigidBodyVisitor::RigidBodyFromDisplayObjectTransform(data.rigid_body_from_display_object_transform.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpDisplayBindingDataRigidBody<'de> {
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
enum HkpDisplayBindingDataRigidBodyVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "rigidBody")]
    RigidBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "displayObjectPtr")]
    DisplayObjectPtr(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rigidBodyFromDisplayObjectTransform")]
    RigidBodyFromDisplayObjectTransform(Primitive<Matrix4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataRigidBodyVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rigidBody" => RigidBody(Primitive<Cow<'de, str>>)),
    ("displayObjectPtr" => DisplayObjectPtr(Primitive<Cow<'de, str>>)),
    ("rigidBodyFromDisplayObjectTransform" => RigidBodyFromDisplayObjectTransform(Primitive<Matrix4<f32>>)),
}
