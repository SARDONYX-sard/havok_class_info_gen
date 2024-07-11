//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRigidBody`
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

/// `hkpRigidBody`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 544
/// -    vtable: true
/// -    parent: `hkpEntity`/`0xa03c774b`
/// - signature: `0x75f8d805`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRigidBody<'a> {
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"material"`
    /// -   type: `struct hkpMaterial`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub material: SingleClass<HkpMaterial>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"limitContactImpulseUtilAndFlag"`
    /// -   type: `void*`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub limit_contact_impulse_util_and_flag: Cow<'a, str>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub damage_multiplier: f32,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"breakableBody"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub breakable_body: Cow<'a, str>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"solverData"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub solver_data: u32,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"storageIndex"`
    /// -   type: `hkUint16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub storage_index: u16,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"contactPointCallbackDelay"`
    /// -   type: `hkUint16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    pub contact_point_callback_delay: u16,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"constraintsMaster"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub constraints_master: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"constraintsSlave"`
    /// -   type: `hkArray<hkpConstraintInstance*>`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE|NOT_OWNED|SERIALIZE_IGNORED`
    pub constraints_slave: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"constraintRuntime"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub constraint_runtime: HkArrayNum<u8>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"simulationIsland"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub simulation_island: Cow<'a, str>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"autoRemoveLevel"`
    /// -   type: `hkInt8`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub auto_remove_level: i8,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"numShapeKeysInContactPointProperties"`
    /// -   type: `hkUint8`
    /// - offset: 209
    /// -  flags: `FLAGS_NONE`
    pub num_shape_keys_in_contact_point_properties: u8,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"responseModifierFlags"`
    /// -   type: `hkUint8`
    /// - offset: 210
    /// -  flags: `FLAGS_NONE`
    pub response_modifier_flags: u8,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub uid: u32,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"spuCollisionCallback"`
    /// -   type: `struct hkpEntitySpuCollisionCallback`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    pub spu_collision_callback: SingleClass<HkpEntitySpuCollisionCallback<'a>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"motion"`
    /// -   type: `struct hkpMaxSizeMotion`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub motion: SingleClass<HkpMaxSizeMotion<'a>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"contactListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 512
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_listeners: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"actions"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 520
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub actions: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 528
    /// -  flags: `FLAGS_NONE`
    pub local_frame: Cow<'a, str>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"extendedListeners"`
    /// -   type: `struct hkpEntityExtendedListeners*`
    /// - offset: 532
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub extended_listeners: Cow<'a, str>,
    /// # C++ Parent class(`hkpEntity` => parent: `hkpWorldObject`) field Info
    /// -   name:`"npData"`
    /// -   type: `hkUint32`
    /// - offset: 536
    /// -  flags: `FLAGS_NONE`
    pub np_data: u32,

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
}

impl Serialize for HkpRigidBody<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRigidBodyVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRigidBody<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRigidBodyVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpRigidBodyVisitor<'a>>> for HkpRigidBody<'a> {
    fn from(_values: Vec<HkpRigidBodyVisitor<'a>>) -> Self {
            let mut material = None;
            let mut limit_contact_impulse_util_and_flag = None;
            let mut damage_multiplier = None;
            let mut breakable_body = None;
            let mut solver_data = None;
            let mut storage_index = None;
            let mut contact_point_callback_delay = None;
            let mut constraints_master = None;
            let mut constraints_slave = None;
            let mut constraint_runtime = None;
            let mut simulation_island = None;
            let mut auto_remove_level = None;
            let mut num_shape_keys_in_contact_point_properties = None;
            let mut response_modifier_flags = None;
            let mut uid = None;
            let mut spu_collision_callback = None;
            let mut motion = None;
            let mut contact_listeners = None;
            let mut actions = None;
            let mut local_frame = None;
            let mut extended_listeners = None;
            let mut np_data = None;
            let mut world = None;
            let mut user_data = None;
            let mut collidable = None;
            let mut multi_thread_check = None;
            let mut name = None;
            let mut properties = None;
            let mut tree_data = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;


        for _value in _values {
            match _value {
                HkpRigidBodyVisitor::Material(m) => material = Some(m),
                HkpRigidBodyVisitor::LimitContactImpulseUtilAndFlag(m) => limit_contact_impulse_util_and_flag = Some(m),
                HkpRigidBodyVisitor::DamageMultiplier(m) => damage_multiplier = Some(m),
                HkpRigidBodyVisitor::BreakableBody(m) => breakable_body = Some(m),
                HkpRigidBodyVisitor::SolverData(m) => solver_data = Some(m),
                HkpRigidBodyVisitor::StorageIndex(m) => storage_index = Some(m),
                HkpRigidBodyVisitor::ContactPointCallbackDelay(m) => contact_point_callback_delay = Some(m),
                HkpRigidBodyVisitor::ConstraintsMaster(m) => constraints_master = Some(m),
                HkpRigidBodyVisitor::ConstraintsSlave(m) => constraints_slave = Some(m),
                HkpRigidBodyVisitor::ConstraintRuntime(m) => constraint_runtime = Some(m),
                HkpRigidBodyVisitor::SimulationIsland(m) => simulation_island = Some(m),
                HkpRigidBodyVisitor::AutoRemoveLevel(m) => auto_remove_level = Some(m),
                HkpRigidBodyVisitor::NumShapeKeysInContactPointProperties(m) => num_shape_keys_in_contact_point_properties = Some(m),
                HkpRigidBodyVisitor::ResponseModifierFlags(m) => response_modifier_flags = Some(m),
                HkpRigidBodyVisitor::Uid(m) => uid = Some(m),
                HkpRigidBodyVisitor::SpuCollisionCallback(m) => spu_collision_callback = Some(m),
                HkpRigidBodyVisitor::Motion(m) => motion = Some(m),
                HkpRigidBodyVisitor::ContactListeners(m) => contact_listeners = Some(m),
                HkpRigidBodyVisitor::Actions(m) => actions = Some(m),
                HkpRigidBodyVisitor::LocalFrame(m) => local_frame = Some(m),
                HkpRigidBodyVisitor::ExtendedListeners(m) => extended_listeners = Some(m),
                HkpRigidBodyVisitor::NpData(m) => np_data = Some(m),
                HkpRigidBodyVisitor::World(m) => world = Some(m),
                HkpRigidBodyVisitor::UserData(m) => user_data = Some(m),
                HkpRigidBodyVisitor::Collidable(m) => collidable = Some(m),
                HkpRigidBodyVisitor::MultiThreadCheck(m) => multi_thread_check = Some(m),
                HkpRigidBodyVisitor::Name(m) => name = Some(m),
                HkpRigidBodyVisitor::Properties(m) => properties = Some(m),
                HkpRigidBodyVisitor::TreeData(m) => tree_data = Some(m),
                HkpRigidBodyVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpRigidBodyVisitor::ReferenceCount(m) => reference_count = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            material: material.unwrap_or_default(),
            limit_contact_impulse_util_and_flag: limit_contact_impulse_util_and_flag.unwrap_or_default().into_inner(),
            damage_multiplier: damage_multiplier.unwrap_or_default().into_inner(),
            breakable_body: breakable_body.unwrap_or_default().into_inner(),
            solver_data: solver_data.unwrap_or_default().into_inner(),
            storage_index: storage_index.unwrap_or_default().into_inner(),
            contact_point_callback_delay: contact_point_callback_delay.unwrap_or_default().into_inner(),
            constraints_master: constraints_master.unwrap_or_default(),
            constraints_slave: constraints_slave.unwrap_or_default(),
            constraint_runtime: constraint_runtime.unwrap_or_default(),
            simulation_island: simulation_island.unwrap_or_default().into_inner(),
            auto_remove_level: auto_remove_level.unwrap_or_default().into_inner(),
            num_shape_keys_in_contact_point_properties: num_shape_keys_in_contact_point_properties.unwrap_or_default().into_inner(),
            response_modifier_flags: response_modifier_flags.unwrap_or_default().into_inner(),
            uid: uid.unwrap_or_default().into_inner(),
            spu_collision_callback: spu_collision_callback.unwrap_or_default(),
            motion: motion.unwrap_or_default(),
            contact_listeners: contact_listeners.unwrap_or_default(),
            actions: actions.unwrap_or_default(),
            local_frame: local_frame.unwrap_or_default().into_inner(),
            extended_listeners: extended_listeners.unwrap_or_default().into_inner(),
            np_data: np_data.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            collidable: collidable.unwrap_or_default(),
            multi_thread_check: multi_thread_check.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            properties: properties.unwrap_or_default(),
            tree_data: tree_data.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpRigidBody<'a>> for Vec<HkpRigidBodyVisitor<'a>> {
    fn from(data: &HkpRigidBody<'a>) -> Self {
        vec![
            HkpRigidBodyVisitor::Material(data.material.clone()),
            HkpRigidBodyVisitor::LimitContactImpulseUtilAndFlag(data.limit_contact_impulse_util_and_flag.clone().into()),
            HkpRigidBodyVisitor::DamageMultiplier(data.damage_multiplier.into()),
            HkpRigidBodyVisitor::BreakableBody(data.breakable_body.clone().into()),
            HkpRigidBodyVisitor::SolverData(data.solver_data.into()),
            HkpRigidBodyVisitor::StorageIndex(data.storage_index.into()),
            HkpRigidBodyVisitor::ContactPointCallbackDelay(data.contact_point_callback_delay.into()),
            HkpRigidBodyVisitor::ConstraintsMaster(data.constraints_master.clone()),
            HkpRigidBodyVisitor::ConstraintsSlave(data.constraints_slave.clone()),
            HkpRigidBodyVisitor::ConstraintRuntime(data.constraint_runtime.clone()),
            HkpRigidBodyVisitor::SimulationIsland(data.simulation_island.clone().into()),
            HkpRigidBodyVisitor::AutoRemoveLevel(data.auto_remove_level.into()),
            HkpRigidBodyVisitor::NumShapeKeysInContactPointProperties(data.num_shape_keys_in_contact_point_properties.into()),
            HkpRigidBodyVisitor::ResponseModifierFlags(data.response_modifier_flags.into()),
            HkpRigidBodyVisitor::Uid(data.uid.into()),
            HkpRigidBodyVisitor::SpuCollisionCallback(data.spu_collision_callback.clone()),
            HkpRigidBodyVisitor::Motion(data.motion.clone()),
            HkpRigidBodyVisitor::ContactListeners(data.contact_listeners.clone()),
            HkpRigidBodyVisitor::Actions(data.actions.clone()),
            HkpRigidBodyVisitor::LocalFrame(data.local_frame.clone().into()),
            HkpRigidBodyVisitor::ExtendedListeners(data.extended_listeners.clone().into()),
            HkpRigidBodyVisitor::NpData(data.np_data.into()),
            HkpRigidBodyVisitor::World(data.world.clone().into()),
            HkpRigidBodyVisitor::UserData(data.user_data.into()),
            HkpRigidBodyVisitor::Collidable(data.collidable.clone()),
            HkpRigidBodyVisitor::MultiThreadCheck(data.multi_thread_check.clone()),
            HkpRigidBodyVisitor::Name(data.name.clone().into()),
            HkpRigidBodyVisitor::Properties(data.properties.clone()),
            HkpRigidBodyVisitor::TreeData(data.tree_data.clone().into()),
            HkpRigidBodyVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpRigidBodyVisitor::ReferenceCount(data.reference_count.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRigidBody<'de> {
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
enum HkpRigidBodyVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "material")]
    Material(SingleClass<HkpMaterial>),
    /// Visitor fields
    #[serde(rename = "limitContactImpulseUtilAndFlag", skip_serializing)]
    LimitContactImpulseUtilAndFlag(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "breakableBody", skip_serializing)]
    BreakableBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "solverData", skip_serializing)]
    SolverData(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "storageIndex")]
    StorageIndex(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "contactPointCallbackDelay")]
    ContactPointCallbackDelay(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "constraintsMaster", skip_serializing)]
    ConstraintsMaster(SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "constraintsSlave", skip_serializing)]
    ConstraintsSlave(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraintRuntime", skip_serializing)]
    ConstraintRuntime(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "simulationIsland", skip_serializing)]
    SimulationIsland(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "autoRemoveLevel")]
    AutoRemoveLevel(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "numShapeKeysInContactPointProperties")]
    NumShapeKeysInContactPointProperties(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "responseModifierFlags")]
    ResponseModifierFlags(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "uid")]
    Uid(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "spuCollisionCallback")]
    SpuCollisionCallback(SingleClass<HkpEntitySpuCollisionCallback<'a>>),
    /// Visitor fields
    #[serde(rename = "motion")]
    Motion(SingleClass<HkpMaxSizeMotion<'a>>),
    /// Visitor fields
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "actions", skip_serializing)]
    Actions(SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "localFrame")]
    LocalFrame(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "extendedListeners", skip_serializing)]
    ExtendedListeners(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "npData")]
    NpData(Primitive<u32>),

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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRigidBodyVisitor<'de>, "@name",
    ("material" => Material(SingleClass<HkpMaterial>)),
    ("limitContactImpulseUtilAndFlag" => LimitContactImpulseUtilAndFlag(Primitive<Cow<'de, str>>)),
    ("damageMultiplier" => DamageMultiplier(Primitive<f32>)),
    ("breakableBody" => BreakableBody(Primitive<Cow<'de, str>>)),
    ("solverData" => SolverData(Primitive<u32>)),
    ("storageIndex" => StorageIndex(Primitive<u16>)),
    ("contactPointCallbackDelay" => ContactPointCallbackDelay(Primitive<u16>)),
    ("constraintsMaster" => ConstraintsMaster(SingleClass<HkpEntitySmallArraySerializeOverrideType<'de>>)),
    ("constraintsSlave" => ConstraintsSlave(HkArrayRef<Cow<'de, str>>)),
    ("constraintRuntime" => ConstraintRuntime(HkArrayNum<u8>)),
    ("simulationIsland" => SimulationIsland(Primitive<Cow<'de, str>>)),
    ("autoRemoveLevel" => AutoRemoveLevel(Primitive<i8>)),
    ("numShapeKeysInContactPointProperties" => NumShapeKeysInContactPointProperties(Primitive<u8>)),
    ("responseModifierFlags" => ResponseModifierFlags(Primitive<u8>)),
    ("uid" => Uid(Primitive<u32>)),
    ("spuCollisionCallback" => SpuCollisionCallback(SingleClass<HkpEntitySpuCollisionCallback<'de>>)),
    ("motion" => Motion(SingleClass<HkpMaxSizeMotion<'de>>)),
    ("contactListeners" => ContactListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'de>>)),
    ("actions" => Actions(SingleClass<HkpEntitySmallArraySerializeOverrideType<'de>>)),
    ("localFrame" => LocalFrame(Primitive<Cow<'de, str>>)),
    ("extendedListeners" => ExtendedListeners(Primitive<Cow<'de, str>>)),
    ("npData" => NpData(Primitive<u32>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(SingleClass<HkpLinkedCollidable<'de>>)),
    ("multiThreadCheck" => MultiThreadCheck(SingleClass<HkMultiThreadCheck>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
