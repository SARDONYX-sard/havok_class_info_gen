//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpArrayAction`
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

/// `hkpArrayAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpAction`/`0xbdf70a51`
/// - signature: `0x674bcd2d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpArrayAction<'a> {
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,

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
    /// -   name:`"entities"`
    /// -   type: `hkArray<hkpEntity*>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entities: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpArrayAction<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpArrayActionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpArrayAction<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpArrayActionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpArrayActionVisitor<'a>>> for HkpArrayAction<'a> {
    fn from(_values: Vec<HkpArrayActionVisitor<'a>>) -> Self {
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut entities = None;


        for _value in _values {
            match _value {
                HkpArrayActionVisitor::World(m) => world = Some(m),
                HkpArrayActionVisitor::Island(m) => island = Some(m),
                HkpArrayActionVisitor::UserData(m) => user_data = Some(m),
                HkpArrayActionVisitor::Name(m) => name = Some(m),
                HkpArrayActionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpArrayActionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpArrayActionVisitor::Entities(m) => entities = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            entities: entities.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpArrayAction<'a>> for Vec<HkpArrayActionVisitor<'a>> {
    fn from(data: &HkpArrayAction<'a>) -> Self {
        vec![
            HkpArrayActionVisitor::World(data.world.clone().into()),
            HkpArrayActionVisitor::Island(data.island.clone().into()),
            HkpArrayActionVisitor::UserData(data.user_data.into()),
            HkpArrayActionVisitor::Name(data.name.clone().into()),
            HkpArrayActionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpArrayActionVisitor::ReferenceCount(data.reference_count.into()),
            HkpArrayActionVisitor::Entities(data.entities.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpArrayAction<'de> {
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
enum HkpArrayActionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "entities")]
    Entities(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpArrayActionVisitor<'de>, "@name",
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("entities" => Entities(HkArrayRef<Cow<'de, str>>)),
}
