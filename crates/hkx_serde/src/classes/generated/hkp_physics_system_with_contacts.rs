//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPhysicsSystemWithContacts`
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

/// `hkpPhysicsSystemWithContacts`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpPhysicsSystem`/`0xff724c17`
/// - signature: `0xd0fd4bbe`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPhysicsSystemWithContacts<'a> {
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"rigidBodies"`
    /// -   type: `hkArray<hkpRigidBody*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rigid_bodies: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"constraints"`
    /// -   type: `hkArray<hkpConstraintInstance*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub constraints: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"actions"`
    /// -   type: `hkArray<hkpAction*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub actions: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"phantoms"`
    /// -   type: `hkArray<hkpPhantom*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub phantoms: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub active: bool,

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
    /// -   name:`"contacts"`
    /// -   type: `hkArray<hkpSerializedAgentNnEntry*>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub contacts: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpPhysicsSystemWithContacts<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPhysicsSystemWithContactsVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPhysicsSystemWithContacts<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPhysicsSystemWithContactsVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPhysicsSystemWithContactsVisitor<'a>>> for HkpPhysicsSystemWithContacts<'a> {
    fn from(_values: Vec<HkpPhysicsSystemWithContactsVisitor<'a>>) -> Self {
            let mut rigid_bodies = None;
            let mut constraints = None;
            let mut actions = None;
            let mut phantoms = None;
            let mut name = None;
            let mut user_data = None;
            let mut active = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut contacts = None;


        for _value in _values {
            match _value {
                HkpPhysicsSystemWithContactsVisitor::RigidBodies(m) => rigid_bodies = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Constraints(m) => constraints = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Actions(m) => actions = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Phantoms(m) => phantoms = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Name(m) => name = Some(m),
                HkpPhysicsSystemWithContactsVisitor::UserData(m) => user_data = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Active(m) => active = Some(m),
                HkpPhysicsSystemWithContactsVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpPhysicsSystemWithContactsVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpPhysicsSystemWithContactsVisitor::Contacts(m) => contacts = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rigid_bodies: rigid_bodies.unwrap_or_default(),
            constraints: constraints.unwrap_or_default(),
            actions: actions.unwrap_or_default(),
            phantoms: phantoms.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            active: active.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            contacts: contacts.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPhysicsSystemWithContacts<'a>> for Vec<HkpPhysicsSystemWithContactsVisitor<'a>> {
    fn from(data: &HkpPhysicsSystemWithContacts<'a>) -> Self {
        vec![
            HkpPhysicsSystemWithContactsVisitor::RigidBodies(data.rigid_bodies.clone()),
            HkpPhysicsSystemWithContactsVisitor::Constraints(data.constraints.clone()),
            HkpPhysicsSystemWithContactsVisitor::Actions(data.actions.clone()),
            HkpPhysicsSystemWithContactsVisitor::Phantoms(data.phantoms.clone()),
            HkpPhysicsSystemWithContactsVisitor::Name(data.name.clone().into()),
            HkpPhysicsSystemWithContactsVisitor::UserData(data.user_data.into()),
            HkpPhysicsSystemWithContactsVisitor::Active(data.active.into()),
            HkpPhysicsSystemWithContactsVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpPhysicsSystemWithContactsVisitor::ReferenceCount(data.reference_count.into()),
            HkpPhysicsSystemWithContactsVisitor::Contacts(data.contacts.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPhysicsSystemWithContacts<'de> {
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
enum HkpPhysicsSystemWithContactsVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "rigidBodies")]
    RigidBodies(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraints")]
    Constraints(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "actions")]
    Actions(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantoms")]
    Phantoms(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "active")]
    Active(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "contacts")]
    Contacts(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsSystemWithContactsVisitor<'de>, "@name",
    ("rigidBodies" => RigidBodies(HkArrayRef<Cow<'de, str>>)),
    ("constraints" => Constraints(HkArrayRef<Cow<'de, str>>)),
    ("actions" => Actions(HkArrayRef<Cow<'de, str>>)),
    ("phantoms" => Phantoms(HkArrayRef<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("active" => Active(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("contacts" => Contacts(HkArrayRef<Cow<'de, str>>)),
}
