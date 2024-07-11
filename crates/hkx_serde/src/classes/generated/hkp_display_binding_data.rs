//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpDisplayBindingData`
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

/// `hkpDisplayBindingData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xdc46c906`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpDisplayBindingData<'a> {
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
    /// -   name:`"rigidBodyBindings"`
    /// -   type: `hkArray<hkpDisplayBindingDataRigidBody*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rigid_body_bindings: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"physicsSystemBindings"`
    /// -   type: `hkArray<hkpDisplayBindingDataPhysicsSystem*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub physics_system_bindings: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpDisplayBindingData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpDisplayBindingDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpDisplayBindingData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpDisplayBindingDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpDisplayBindingDataVisitor<'a>>> for HkpDisplayBindingData<'a> {
    fn from(_values: Vec<HkpDisplayBindingDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rigid_body_bindings = None;
            let mut physics_system_bindings = None;


        for _value in _values {
            match _value {
                HkpDisplayBindingDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpDisplayBindingDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpDisplayBindingDataVisitor::RigidBodyBindings(m) => rigid_body_bindings = Some(m),
                HkpDisplayBindingDataVisitor::PhysicsSystemBindings(m) => physics_system_bindings = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rigid_body_bindings: rigid_body_bindings.unwrap_or_default(),
            physics_system_bindings: physics_system_bindings.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpDisplayBindingData<'a>> for Vec<HkpDisplayBindingDataVisitor<'a>> {
    fn from(data: &HkpDisplayBindingData<'a>) -> Self {
        vec![
            HkpDisplayBindingDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpDisplayBindingDataVisitor::ReferenceCount(data.reference_count.into()),
            HkpDisplayBindingDataVisitor::RigidBodyBindings(data.rigid_body_bindings.clone()),
            HkpDisplayBindingDataVisitor::PhysicsSystemBindings(data.physics_system_bindings.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpDisplayBindingData<'de> {
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
enum HkpDisplayBindingDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "rigidBodyBindings")]
    RigidBodyBindings(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "physicsSystemBindings")]
    PhysicsSystemBindings(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rigidBodyBindings" => RigidBodyBindings(HkArrayRef<Cow<'de, str>>)),
    ("physicsSystemBindings" => PhysicsSystemBindings(HkArrayRef<Cow<'de, str>>)),
}
