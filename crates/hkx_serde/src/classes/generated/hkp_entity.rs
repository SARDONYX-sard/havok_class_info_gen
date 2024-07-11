//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntity`
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

/// `hkpEntity`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 544
/// -    vtable: true
/// -    parent: `hkpWorldObject`/`0x49fb6f2e`
/// - signature: `0xa03c774b`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpEntity<'a> {
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
    /// -   name:`"material"`
    /// -   type: `struct hkpMaterial`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub material: SingleClass<HkpMaterial>,
    /// # C++ Class Fields Info
    /// -   name:`"limitContactImpulseUtilAndFlag"`
    /// -   type: `void*`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub limit_contact_impulse_util_and_flag: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub damage_multiplier: f32,
    /// # C++ Class Fields Info
    /// -   name:`"breakableBody"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub breakable_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"solverData"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub solver_data: u32,
    /// # C++ Class Fields Info
    /// -   name:`"storageIndex"`
    /// -   type: `hkUint16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub storage_index: u16,
    /// # C++ Class Fields Info
    /// -   name:`"contactPointCallbackDelay"`
    /// -   type: `hkUint16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    pub contact_point_callback_delay: u16,
    /// # C++ Class Fields Info
    /// -   name:`"constraintsMaster"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub constraints_master: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintsSlave"`
    /// -   type: `hkArray<hkpConstraintInstance*>`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE|NOT_OWNED|SERIALIZE_IGNORED`
    pub constraints_slave: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintRuntime"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub constraint_runtime: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"simulationIsland"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub simulation_island: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"autoRemoveLevel"`
    /// -   type: `hkInt8`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub auto_remove_level: i8,
    /// # C++ Class Fields Info
    /// -   name:`"numShapeKeysInContactPointProperties"`
    /// -   type: `hkUint8`
    /// - offset: 209
    /// -  flags: `FLAGS_NONE`
    pub num_shape_keys_in_contact_point_properties: u8,
    /// # C++ Class Fields Info
    /// -   name:`"responseModifierFlags"`
    /// -   type: `hkUint8`
    /// - offset: 210
    /// -  flags: `FLAGS_NONE`
    pub response_modifier_flags: u8,
    /// # C++ Class Fields Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub uid: u32,
    /// # C++ Class Fields Info
    /// -   name:`"spuCollisionCallback"`
    /// -   type: `struct hkpEntitySpuCollisionCallback`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    pub spu_collision_callback: SingleClass<HkpEntitySpuCollisionCallback<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"motion"`
    /// -   type: `struct hkpMaxSizeMotion`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub motion: SingleClass<HkpMaxSizeMotion<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"contactListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 512
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_listeners: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"actions"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 520
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub actions: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 528
    /// -  flags: `FLAGS_NONE`
    pub local_frame: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"extendedListeners"`
    /// -   type: `struct hkpEntityExtendedListeners*`
    /// - offset: 532
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub extended_listeners: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"npData"`
    /// -   type: `hkUint32`
    /// - offset: 536
    /// -  flags: `FLAGS_NONE`
    pub np_data: u32,
}

impl Serialize for HkpEntity<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpEntityVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpEntity<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpEntityVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpEntityVisitor<'a>>> for HkpEntity<'a> {
    fn from(_values: Vec<HkpEntityVisitor<'a>>) -> Self {
            let mut world = None;
            let mut user_data = None;
            let mut collidable = None;
            let mut multi_thread_check = None;
            let mut name = None;
            let mut properties = None;
            let mut tree_data = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
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


        for _value in _values {
            match _value {
                HkpEntityVisitor::World(m) => world = Some(m),
                HkpEntityVisitor::UserData(m) => user_data = Some(m),
                HkpEntityVisitor::Collidable(m) => collidable = Some(m),
                HkpEntityVisitor::MultiThreadCheck(m) => multi_thread_check = Some(m),
                HkpEntityVisitor::Name(m) => name = Some(m),
                HkpEntityVisitor::Properties(m) => properties = Some(m),
                HkpEntityVisitor::TreeData(m) => tree_data = Some(m),
                HkpEntityVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpEntityVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpEntityVisitor::Material(m) => material = Some(m),
                HkpEntityVisitor::LimitContactImpulseUtilAndFlag(m) => limit_contact_impulse_util_and_flag = Some(m),
                HkpEntityVisitor::DamageMultiplier(m) => damage_multiplier = Some(m),
                HkpEntityVisitor::BreakableBody(m) => breakable_body = Some(m),
                HkpEntityVisitor::SolverData(m) => solver_data = Some(m),
                HkpEntityVisitor::StorageIndex(m) => storage_index = Some(m),
                HkpEntityVisitor::ContactPointCallbackDelay(m) => contact_point_callback_delay = Some(m),
                HkpEntityVisitor::ConstraintsMaster(m) => constraints_master = Some(m),
                HkpEntityVisitor::ConstraintsSlave(m) => constraints_slave = Some(m),
                HkpEntityVisitor::ConstraintRuntime(m) => constraint_runtime = Some(m),
                HkpEntityVisitor::SimulationIsland(m) => simulation_island = Some(m),
                HkpEntityVisitor::AutoRemoveLevel(m) => auto_remove_level = Some(m),
                HkpEntityVisitor::NumShapeKeysInContactPointProperties(m) => num_shape_keys_in_contact_point_properties = Some(m),
                HkpEntityVisitor::ResponseModifierFlags(m) => response_modifier_flags = Some(m),
                HkpEntityVisitor::Uid(m) => uid = Some(m),
                HkpEntityVisitor::SpuCollisionCallback(m) => spu_collision_callback = Some(m),
                HkpEntityVisitor::Motion(m) => motion = Some(m),
                HkpEntityVisitor::ContactListeners(m) => contact_listeners = Some(m),
                HkpEntityVisitor::Actions(m) => actions = Some(m),
                HkpEntityVisitor::LocalFrame(m) => local_frame = Some(m),
                HkpEntityVisitor::ExtendedListeners(m) => extended_listeners = Some(m),
                HkpEntityVisitor::NpData(m) => np_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            world: world.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            collidable: collidable.unwrap_or_default(),
            multi_thread_check: multi_thread_check.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            properties: properties.unwrap_or_default(),
            tree_data: tree_data.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
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

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpEntity<'a>> for Vec<HkpEntityVisitor<'a>> {
    fn from(data: &HkpEntity<'a>) -> Self {
        vec![
            HkpEntityVisitor::World(data.world.clone().into()),
            HkpEntityVisitor::UserData(data.user_data.into()),
            HkpEntityVisitor::Collidable(data.collidable.clone()),
            HkpEntityVisitor::MultiThreadCheck(data.multi_thread_check.clone()),
            HkpEntityVisitor::Name(data.name.clone().into()),
            HkpEntityVisitor::Properties(data.properties.clone()),
            HkpEntityVisitor::TreeData(data.tree_data.clone().into()),
            HkpEntityVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpEntityVisitor::ReferenceCount(data.reference_count.into()),
            HkpEntityVisitor::Material(data.material.clone()),
            HkpEntityVisitor::LimitContactImpulseUtilAndFlag(data.limit_contact_impulse_util_and_flag.clone().into()),
            HkpEntityVisitor::DamageMultiplier(data.damage_multiplier.into()),
            HkpEntityVisitor::BreakableBody(data.breakable_body.clone().into()),
            HkpEntityVisitor::SolverData(data.solver_data.into()),
            HkpEntityVisitor::StorageIndex(data.storage_index.into()),
            HkpEntityVisitor::ContactPointCallbackDelay(data.contact_point_callback_delay.into()),
            HkpEntityVisitor::ConstraintsMaster(data.constraints_master.clone()),
            HkpEntityVisitor::ConstraintsSlave(data.constraints_slave.clone()),
            HkpEntityVisitor::ConstraintRuntime(data.constraint_runtime.clone()),
            HkpEntityVisitor::SimulationIsland(data.simulation_island.clone().into()),
            HkpEntityVisitor::AutoRemoveLevel(data.auto_remove_level.into()),
            HkpEntityVisitor::NumShapeKeysInContactPointProperties(data.num_shape_keys_in_contact_point_properties.into()),
            HkpEntityVisitor::ResponseModifierFlags(data.response_modifier_flags.into()),
            HkpEntityVisitor::Uid(data.uid.into()),
            HkpEntityVisitor::SpuCollisionCallback(data.spu_collision_callback.clone()),
            HkpEntityVisitor::Motion(data.motion.clone()),
            HkpEntityVisitor::ContactListeners(data.contact_listeners.clone()),
            HkpEntityVisitor::Actions(data.actions.clone()),
            HkpEntityVisitor::LocalFrame(data.local_frame.clone().into()),
            HkpEntityVisitor::ExtendedListeners(data.extended_listeners.clone().into()),
            HkpEntityVisitor::NpData(data.np_data.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpEntity<'de> {
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
enum HkpEntityVisitor<'a> {
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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntityVisitor<'de>, "@name",
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(SingleClass<HkpLinkedCollidable<'de>>)),
    ("multiThreadCheck" => MultiThreadCheck(SingleClass<HkMultiThreadCheck>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
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
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SpuCollisionCallbackEventFilter {
    #[serde(rename = "SPU_SEND_NONE")]
    #[default]
    SpuSendNone = 0,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_ADDED")]
    SpuSendContactPointAdded = 1,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_PROCESS")]
    SpuSendContactPointProcess = 2,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_REMOVED")]
    SpuSendContactPointRemoved = 4,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_ADDED_OR_PROCESS")]
    SpuSendContactPointAddedOrProcess = 3,
}
