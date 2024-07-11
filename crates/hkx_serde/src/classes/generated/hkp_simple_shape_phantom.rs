//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleShapePhantom`
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

/// `hkpSimpleShapePhantom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 368
/// -    vtable: true
/// -    parent: `hkpShapePhantom`/`0xcb22fbcd`
/// - signature: `0x32a2a8a8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimpleShapePhantom<'a> {
    /// # C++ Parent class(`hkpShapePhantom` => parent: `hkpPhantom`) field Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub motion_state: SingleClass<HkMotionState>,

    /// # C++ Parent class(`hkpPhantom` => parent: `hkpWorldObject`) field Info
    /// -   name:`"overlapListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub overlap_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpPhantom` => parent: `hkpWorldObject`) field Info
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub phantom_listeners: HkArrayRef<Cow<'a, str>>,

    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub collidable: SingleClass<HkpLinkedCollidable<'a>>,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub multi_thread_check: SingleClass<HkMultiThreadCheck>,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"properties"`
    /// -   type: `hkArray<struct hkpProperty>`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub properties: HkArrayClass<HkpProperty>,
    /// # C++ Parent class(`hkpWorldObject` => parent: `hkReferencedObject`) field Info
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub tree_data: Cow<'a, str>,

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
    /// -   name:`"collisionDetails"`
    /// -   type: `hkArray<struct hkpSimpleShapePhantomCollisionDetail>`
    /// - offset: 352
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub collision_details: HkArrayClass<HkpSimpleShapePhantomCollisionDetail<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 364
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub order_dirty: bool,
}

impl Serialize for HkpSimpleShapePhantom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimpleShapePhantomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimpleShapePhantom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimpleShapePhantomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSimpleShapePhantomVisitor<'a>>> for HkpSimpleShapePhantom<'a> {
    fn from(_values: Vec<HkpSimpleShapePhantomVisitor<'a>>) -> Self {
            let mut motion_state = None;
            let mut overlap_listeners = None;
            let mut phantom_listeners = None;
            let mut world = None;
            let mut user_data = None;
            let mut collidable = None;
            let mut multi_thread_check = None;
            let mut name = None;
            let mut properties = None;
            let mut tree_data = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut collision_details = None;
            let mut order_dirty = None;


        for _value in _values {
            match _value {
                HkpSimpleShapePhantomVisitor::MotionState(m) => motion_state = Some(m),
                HkpSimpleShapePhantomVisitor::OverlapListeners(m) => overlap_listeners = Some(m),
                HkpSimpleShapePhantomVisitor::PhantomListeners(m) => phantom_listeners = Some(m),
                HkpSimpleShapePhantomVisitor::World(m) => world = Some(m),
                HkpSimpleShapePhantomVisitor::UserData(m) => user_data = Some(m),
                HkpSimpleShapePhantomVisitor::Collidable(m) => collidable = Some(m),
                HkpSimpleShapePhantomVisitor::MultiThreadCheck(m) => multi_thread_check = Some(m),
                HkpSimpleShapePhantomVisitor::Name(m) => name = Some(m),
                HkpSimpleShapePhantomVisitor::Properties(m) => properties = Some(m),
                HkpSimpleShapePhantomVisitor::TreeData(m) => tree_data = Some(m),
                HkpSimpleShapePhantomVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpSimpleShapePhantomVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpSimpleShapePhantomVisitor::CollisionDetails(m) => collision_details = Some(m),
                HkpSimpleShapePhantomVisitor::OrderDirty(m) => order_dirty = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            motion_state: motion_state.unwrap_or_default(),
            overlap_listeners: overlap_listeners.unwrap_or_default(),
            phantom_listeners: phantom_listeners.unwrap_or_default(),
            world: world.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            collidable: collidable.unwrap_or_default(),
            multi_thread_check: multi_thread_check.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            properties: properties.unwrap_or_default(),
            tree_data: tree_data.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            collision_details: collision_details.unwrap_or_default(),
            order_dirty: order_dirty.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSimpleShapePhantom<'a>> for Vec<HkpSimpleShapePhantomVisitor<'a>> {
    fn from(data: &HkpSimpleShapePhantom<'a>) -> Self {
        vec![
            HkpSimpleShapePhantomVisitor::MotionState(data.motion_state.clone()),
            HkpSimpleShapePhantomVisitor::OverlapListeners(data.overlap_listeners.clone()),
            HkpSimpleShapePhantomVisitor::PhantomListeners(data.phantom_listeners.clone()),
            HkpSimpleShapePhantomVisitor::World(data.world.clone().into()),
            HkpSimpleShapePhantomVisitor::UserData(data.user_data.into()),
            HkpSimpleShapePhantomVisitor::Collidable(data.collidable.clone()),
            HkpSimpleShapePhantomVisitor::MultiThreadCheck(data.multi_thread_check.clone()),
            HkpSimpleShapePhantomVisitor::Name(data.name.clone().into()),
            HkpSimpleShapePhantomVisitor::Properties(data.properties.clone()),
            HkpSimpleShapePhantomVisitor::TreeData(data.tree_data.clone().into()),
            HkpSimpleShapePhantomVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpSimpleShapePhantomVisitor::ReferenceCount(data.reference_count.into()),
            HkpSimpleShapePhantomVisitor::CollisionDetails(data.collision_details.clone()),
            HkpSimpleShapePhantomVisitor::OrderDirty(data.order_dirty.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimpleShapePhantom<'de> {
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
enum HkpSimpleShapePhantomVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "motionState")]
    MotionState(SingleClass<HkMotionState>),

    /// Visitor fields
    #[serde(rename = "overlapListeners", skip_serializing)]
    OverlapListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantomListeners", skip_serializing)]
    PhantomListeners(HkArrayRef<Cow<'a, str>>),

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

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "collisionDetails", skip_serializing)]
    CollisionDetails(HkArrayClass<HkpSimpleShapePhantomCollisionDetail<'a>>),
    /// Visitor fields
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleShapePhantomVisitor<'de>, "@name",
    ("motionState" => MotionState(SingleClass<HkMotionState>)),
    ("overlapListeners" => OverlapListeners(HkArrayRef<Cow<'de, str>>)),
    ("phantomListeners" => PhantomListeners(HkArrayRef<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(SingleClass<HkpLinkedCollidable<'de>>)),
    ("multiThreadCheck" => MultiThreadCheck(SingleClass<HkMultiThreadCheck>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("collisionDetails" => CollisionDetails(HkArrayClass<HkpSimpleShapePhantomCollisionDetail<'de>>)),
    ("orderDirty" => OrderDirty(Primitive<bool>)),
}
