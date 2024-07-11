//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorldObject`
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

/// `hkpWorldObject`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 140
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x49fb6f2e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpWorldObject<'a> {
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
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Class Fields Info
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub collidable: SingleClass<HkpLinkedCollidable<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub multi_thread_check: SingleClass<HkMultiThreadCheck>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"properties"`
    /// -   type: `hkArray<struct hkpProperty>`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub properties: HkArrayClass<HkpProperty>,
    /// # C++ Class Fields Info
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub tree_data: Cow<'a, str>,
}

impl Serialize for HkpWorldObject<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpWorldObjectVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpWorldObject<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpWorldObjectVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpWorldObjectVisitor<'a>>> for HkpWorldObject<'a> {
    fn from(_values: Vec<HkpWorldObjectVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut world = None;
            let mut user_data = None;
            let mut collidable = None;
            let mut multi_thread_check = None;
            let mut name = None;
            let mut properties = None;
            let mut tree_data = None;


        for _value in _values {
            match _value {
                HkpWorldObjectVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpWorldObjectVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpWorldObjectVisitor::World(m) => world = Some(m),
                HkpWorldObjectVisitor::UserData(m) => user_data = Some(m),
                HkpWorldObjectVisitor::Collidable(m) => collidable = Some(m),
                HkpWorldObjectVisitor::MultiThreadCheck(m) => multi_thread_check = Some(m),
                HkpWorldObjectVisitor::Name(m) => name = Some(m),
                HkpWorldObjectVisitor::Properties(m) => properties = Some(m),
                HkpWorldObjectVisitor::TreeData(m) => tree_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            collidable: collidable.unwrap_or_default(),
            multi_thread_check: multi_thread_check.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            properties: properties.unwrap_or_default(),
            tree_data: tree_data.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpWorldObject<'a>> for Vec<HkpWorldObjectVisitor<'a>> {
    fn from(data: &HkpWorldObject<'a>) -> Self {
        vec![
            HkpWorldObjectVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpWorldObjectVisitor::ReferenceCount(data.reference_count.into()),
            HkpWorldObjectVisitor::World(data.world.clone().into()),
            HkpWorldObjectVisitor::UserData(data.user_data.into()),
            HkpWorldObjectVisitor::Collidable(data.collidable.clone()),
            HkpWorldObjectVisitor::MultiThreadCheck(data.multi_thread_check.clone()),
            HkpWorldObjectVisitor::Name(data.name.clone().into()),
            HkpWorldObjectVisitor::Properties(data.properties.clone()),
            HkpWorldObjectVisitor::TreeData(data.tree_data.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpWorldObject<'de> {
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
enum HkpWorldObjectVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "collidable")]
    Collidable(SingleClass<HkpLinkedCollidable<'a>>),
    /// Visitor fields
    #[serde(rename = "multiThreadCheck")]
    MultiThreadCheck(SingleClass<HkMultiThreadCheck>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "properties")]
    Properties(HkArrayClass<HkpProperty>),
    /// Visitor fields
    #[serde(rename = "treeData", skip_serializing)]
    TreeData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldObjectVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(SingleClass<HkpLinkedCollidable<'de>>)),
    ("multiThreadCheck" => MultiThreadCheck(SingleClass<HkMultiThreadCheck>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MtChecks {
    #[serde(rename = "MULTI_THREADING_CHECKS_ENABLE")]
    #[default]
    MultiThreadingChecksEnable = 0,
    #[serde(rename = "MULTI_THREADING_CHECKS_IGNORE")]
    MultiThreadingChecksIgnore = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BroadPhaseType {
    #[serde(rename = "BROAD_PHASE_INVALID")]
    #[default]
    BroadPhaseInvalid = 0,
    #[serde(rename = "BROAD_PHASE_ENTITY")]
    BroadPhaseEntity = 1,
    #[serde(rename = "BROAD_PHASE_PHANTOM")]
    BroadPhasePhantom = 2,
    #[serde(rename = "BROAD_PHASE_BORDER")]
    BroadPhaseBorder = 3,
    #[serde(rename = "BROAD_PHASE_MAX_ID")]
    BroadPhaseMaxId = 4,
}
