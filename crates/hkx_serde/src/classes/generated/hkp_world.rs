//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorld`
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

/// `hkpWorld`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 864
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xaadcec37`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpWorld<'a> {
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
    /// -   name:`"simulation"`
    /// -   type: `struct hkpSimulation*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub simulation: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub gravity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"fixedIsland"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub fixed_island: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"fixedRigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub fixed_rigid_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"activeSimulationIslands"`
    /// -   type: `hkArray<void*>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub active_simulation_islands: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"inactiveSimulationIslands"`
    /// -   type: `hkArray<void*>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub inactive_simulation_islands: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"dirtySimulationIslands"`
    /// -   type: `hkArray<void*>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub dirty_simulation_islands: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"maintenanceMgr"`
    /// -   type: `void*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub maintenance_mgr: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"memoryWatchDog"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub memory_watch_dog: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"assertOnRunningOutOfSolverMemory"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub assert_on_running_out_of_solver_memory: bool,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhase"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub broad_phase: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"kdTreeManager"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub kd_tree_manager: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"autoUpdateTree"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub auto_update_tree: bool,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseDispatcher"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub broad_phase_dispatcher: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"phantomBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub phantom_broad_phase_listener: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"entityEntityBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub entity_entity_broad_phase_listener: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorderListener"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub broad_phase_border_listener: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"multithreadedSimulationJobData"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub multithreaded_simulation_job_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"collisionInput"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub collision_input: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilter"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub collision_filter: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"collisionDispatcher"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub collision_dispatcher: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"convexListFilter"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub convex_list_filter: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperations"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pending_operations: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub pending_operations_count: i32,
    /// # C++ Class Fields Info
    /// -   name:`"pendingBodyOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pending_body_operations_count: i32,
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsLockCount"`
    /// -   type: `hkInt32`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub critical_operations_lock_count: i32,
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsLockCountForPhantoms"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub critical_operations_lock_count_for_phantoms: i32,
    /// # C++ Class Fields Info
    /// -   name:`"blockExecutingPendingOperations"`
    /// -   type: `hkBool`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub block_executing_pending_operations: bool,
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsAllowed"`
    /// -   type: `hkBool`
    /// - offset: 157
    /// -  flags: `FLAGS_NONE`
    pub critical_operations_allowed: bool,
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationQueues"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pending_operation_queues: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationQueueCount"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub pending_operation_queue_count: i32,
    /// # C++ Class Fields Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub multi_thread_check: SingleClass<HkMultiThreadCheck>,
    /// # C++ Class Fields Info
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub process_actions_in_single_thread: bool,
    /// # C++ Class Fields Info
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    pub allow_integration_of_islands_without_constraints_in_a_separate_job: bool,
    /// # C++ Class Fields Info
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub min_desired_island_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"modifyConstraintCriticalSection"`
    /// -   type: `void*`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub modify_constraint_critical_section: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub is_locked: i32,
    /// # C++ Class Fields Info
    /// -   name:`"islandDirtyListCriticalSection"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island_dirty_list_critical_section: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"propertyMasterLock"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub property_master_lock: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"wantSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    pub want_simulation_islands: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub use_hybrid_broadphase: bool,
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub snap_collision_to_convex_edge_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub snap_collision_to_concave_edge_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    pub enable_toi_weld_rejection: bool,
    /// # C++ Class Fields Info
    /// -   name:`"wantDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    pub want_deactivation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    pub should_activate_on_rigid_body_transform_change: bool,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    pub deactivation_reference_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub toi_collision_response_rotate_normal: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    pub max_sectors_per_midphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    pub max_sectors_per_narrowphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    pub process_tois_multithreaded: bool,
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    pub max_entries_per_toi_midphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE`
    pub max_entries_per_toi_narrowphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE`
    pub max_num_toi_collision_pairs_singlethreaded: i32,
    /// # C++ Class Fields Info
    /// -   name:`"simulationType"`
    /// -   type: `enum unknown`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub simulation_type: (),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_simplified_toi: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 260
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 264
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi_higher: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi_forced: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lastEntityUid"`
    /// -   type: `hkUint32`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    pub last_entity_uid: u32,
    /// # C++ Class Fields Info
    /// -   name:`"lastIslandUid"`
    /// -   type: `hkUint32`
    /// - offset: 276
    /// -  flags: `FLAGS_NONE`
    pub last_island_uid: u32,
    /// # C++ Class Fields Info
    /// -   name:`"lastConstraintUid"`
    /// -   type: `hkUint32`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    pub last_constraint_uid: u32,
    /// # C++ Class Fields Info
    /// -   name:`"phantoms"`
    /// -   type: `hkArray<hkpPhantom*>`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    pub phantoms: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"actionListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub action_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"entityListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub entity_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub phantom_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 332
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub constraint_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"worldDeletionListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 344
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_deletion_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"islandActivationListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 356
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island_activation_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"worldPostSimulationListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 368
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_post_simulation_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"worldPostIntegrateListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 380
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_post_integrate_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"worldPostCollideListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 392
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_post_collide_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"islandPostIntegrateListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 404
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island_post_integrate_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"islandPostCollideListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island_post_collide_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"contactListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 428
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"contactImpulseLimitBreachedListeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 440
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_impulse_limit_breached_listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"worldExtensions"`
    /// -   type: `hkArray<void*>`
    /// - offset: 452
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_extensions: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"violatedConstraintArray"`
    /// -   type: `void*`
    /// - offset: 464
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub violated_constraint_array: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorder"`
    /// -   type: `void*`
    /// - offset: 468
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub broad_phase_border: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 472
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub destruction_world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"npWorld"`
    /// -   type: `void*`
    /// - offset: 476
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub np_world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseExtents"`
    /// -   type: `hkVector4[2]`
    /// - offset: 800
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_extents: CStyleArrayVector<Vector4<f32>, 2>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 832
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_num_markers: i32,
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 836
    /// -  flags: `FLAGS_NONE`
    pub size_of_toi_event_queue: i32,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 840
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_query_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseUpdateSize"`
    /// -   type: `hkInt32`
    /// - offset: 844
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_update_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum unknown`
    /// - offset: 848
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_point_generation: (),
}

impl Serialize for HkpWorld<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpWorldVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpWorld<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpWorldVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpWorldVisitor<'a>>> for HkpWorld<'a> {
    fn from(_values: Vec<HkpWorldVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut simulation = None;
            let mut gravity = None;
            let mut fixed_island = None;
            let mut fixed_rigid_body = None;
            let mut active_simulation_islands = None;
            let mut inactive_simulation_islands = None;
            let mut dirty_simulation_islands = None;
            let mut maintenance_mgr = None;
            let mut memory_watch_dog = None;
            let mut assert_on_running_out_of_solver_memory = None;
            let mut broad_phase = None;
            let mut kd_tree_manager = None;
            let mut auto_update_tree = None;
            let mut broad_phase_dispatcher = None;
            let mut phantom_broad_phase_listener = None;
            let mut entity_entity_broad_phase_listener = None;
            let mut broad_phase_border_listener = None;
            let mut multithreaded_simulation_job_data = None;
            let mut collision_input = None;
            let mut collision_filter = None;
            let mut collision_dispatcher = None;
            let mut convex_list_filter = None;
            let mut pending_operations = None;
            let mut pending_operations_count = None;
            let mut pending_body_operations_count = None;
            let mut critical_operations_lock_count = None;
            let mut critical_operations_lock_count_for_phantoms = None;
            let mut block_executing_pending_operations = None;
            let mut critical_operations_allowed = None;
            let mut pending_operation_queues = None;
            let mut pending_operation_queue_count = None;
            let mut multi_thread_check = None;
            let mut process_actions_in_single_thread = None;
            let mut allow_integration_of_islands_without_constraints_in_a_separate_job = None;
            let mut min_desired_island_size = None;
            let mut modify_constraint_critical_section = None;
            let mut is_locked = None;
            let mut island_dirty_list_critical_section = None;
            let mut property_master_lock = None;
            let mut want_simulation_islands = None;
            let mut use_hybrid_broadphase = None;
            let mut snap_collision_to_convex_edge_threshold = None;
            let mut snap_collision_to_concave_edge_threshold = None;
            let mut enable_toi_weld_rejection = None;
            let mut want_deactivation = None;
            let mut should_activate_on_rigid_body_transform_change = None;
            let mut deactivation_reference_distance = None;
            let mut toi_collision_response_rotate_normal = None;
            let mut max_sectors_per_midphase_collide_task = None;
            let mut max_sectors_per_narrowphase_collide_task = None;
            let mut process_tois_multithreaded = None;
            let mut max_entries_per_toi_midphase_collide_task = None;
            let mut max_entries_per_toi_narrowphase_collide_task = None;
            let mut max_num_toi_collision_pairs_singlethreaded = None;
            let mut simulation_type = None;
            let mut num_tois_till_allowed_penetration_simplified_toi = None;
            let mut num_tois_till_allowed_penetration_toi = None;
            let mut num_tois_till_allowed_penetration_toi_higher = None;
            let mut num_tois_till_allowed_penetration_toi_forced = None;
            let mut last_entity_uid = None;
            let mut last_island_uid = None;
            let mut last_constraint_uid = None;
            let mut phantoms = None;
            let mut action_listeners = None;
            let mut entity_listeners = None;
            let mut phantom_listeners = None;
            let mut constraint_listeners = None;
            let mut world_deletion_listeners = None;
            let mut island_activation_listeners = None;
            let mut world_post_simulation_listeners = None;
            let mut world_post_integrate_listeners = None;
            let mut world_post_collide_listeners = None;
            let mut island_post_integrate_listeners = None;
            let mut island_post_collide_listeners = None;
            let mut contact_listeners = None;
            let mut contact_impulse_limit_breached_listeners = None;
            let mut world_extensions = None;
            let mut violated_constraint_array = None;
            let mut broad_phase_border = None;
            let mut destruction_world = None;
            let mut np_world = None;
            let mut broad_phase_extents = None;
            let mut broad_phase_num_markers = None;
            let mut size_of_toi_event_queue = None;
            let mut broad_phase_query_size = None;
            let mut broad_phase_update_size = None;
            let mut contact_point_generation = None;


        for _value in _values {
            match _value {
                HkpWorldVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpWorldVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpWorldVisitor::Simulation(m) => simulation = Some(m),
                HkpWorldVisitor::Gravity(m) => gravity = Some(m),
                HkpWorldVisitor::FixedIsland(m) => fixed_island = Some(m),
                HkpWorldVisitor::FixedRigidBody(m) => fixed_rigid_body = Some(m),
                HkpWorldVisitor::ActiveSimulationIslands(m) => active_simulation_islands = Some(m),
                HkpWorldVisitor::InactiveSimulationIslands(m) => inactive_simulation_islands = Some(m),
                HkpWorldVisitor::DirtySimulationIslands(m) => dirty_simulation_islands = Some(m),
                HkpWorldVisitor::MaintenanceMgr(m) => maintenance_mgr = Some(m),
                HkpWorldVisitor::MemoryWatchDog(m) => memory_watch_dog = Some(m),
                HkpWorldVisitor::AssertOnRunningOutOfSolverMemory(m) => assert_on_running_out_of_solver_memory = Some(m),
                HkpWorldVisitor::BroadPhase(m) => broad_phase = Some(m),
                HkpWorldVisitor::KdTreeManager(m) => kd_tree_manager = Some(m),
                HkpWorldVisitor::AutoUpdateTree(m) => auto_update_tree = Some(m),
                HkpWorldVisitor::BroadPhaseDispatcher(m) => broad_phase_dispatcher = Some(m),
                HkpWorldVisitor::PhantomBroadPhaseListener(m) => phantom_broad_phase_listener = Some(m),
                HkpWorldVisitor::EntityEntityBroadPhaseListener(m) => entity_entity_broad_phase_listener = Some(m),
                HkpWorldVisitor::BroadPhaseBorderListener(m) => broad_phase_border_listener = Some(m),
                HkpWorldVisitor::MultithreadedSimulationJobData(m) => multithreaded_simulation_job_data = Some(m),
                HkpWorldVisitor::CollisionInput(m) => collision_input = Some(m),
                HkpWorldVisitor::CollisionFilter(m) => collision_filter = Some(m),
                HkpWorldVisitor::CollisionDispatcher(m) => collision_dispatcher = Some(m),
                HkpWorldVisitor::ConvexListFilter(m) => convex_list_filter = Some(m),
                HkpWorldVisitor::PendingOperations(m) => pending_operations = Some(m),
                HkpWorldVisitor::PendingOperationsCount(m) => pending_operations_count = Some(m),
                HkpWorldVisitor::PendingBodyOperationsCount(m) => pending_body_operations_count = Some(m),
                HkpWorldVisitor::CriticalOperationsLockCount(m) => critical_operations_lock_count = Some(m),
                HkpWorldVisitor::CriticalOperationsLockCountForPhantoms(m) => critical_operations_lock_count_for_phantoms = Some(m),
                HkpWorldVisitor::BlockExecutingPendingOperations(m) => block_executing_pending_operations = Some(m),
                HkpWorldVisitor::CriticalOperationsAllowed(m) => critical_operations_allowed = Some(m),
                HkpWorldVisitor::PendingOperationQueues(m) => pending_operation_queues = Some(m),
                HkpWorldVisitor::PendingOperationQueueCount(m) => pending_operation_queue_count = Some(m),
                HkpWorldVisitor::MultiThreadCheck(m) => multi_thread_check = Some(m),
                HkpWorldVisitor::ProcessActionsInSingleThread(m) => process_actions_in_single_thread = Some(m),
                HkpWorldVisitor::AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(m) => allow_integration_of_islands_without_constraints_in_a_separate_job = Some(m),
                HkpWorldVisitor::MinDesiredIslandSize(m) => min_desired_island_size = Some(m),
                HkpWorldVisitor::ModifyConstraintCriticalSection(m) => modify_constraint_critical_section = Some(m),
                HkpWorldVisitor::IsLocked(m) => is_locked = Some(m),
                HkpWorldVisitor::IslandDirtyListCriticalSection(m) => island_dirty_list_critical_section = Some(m),
                HkpWorldVisitor::PropertyMasterLock(m) => property_master_lock = Some(m),
                HkpWorldVisitor::WantSimulationIslands(m) => want_simulation_islands = Some(m),
                HkpWorldVisitor::UseHybridBroadphase(m) => use_hybrid_broadphase = Some(m),
                HkpWorldVisitor::SnapCollisionToConvexEdgeThreshold(m) => snap_collision_to_convex_edge_threshold = Some(m),
                HkpWorldVisitor::SnapCollisionToConcaveEdgeThreshold(m) => snap_collision_to_concave_edge_threshold = Some(m),
                HkpWorldVisitor::EnableToiWeldRejection(m) => enable_toi_weld_rejection = Some(m),
                HkpWorldVisitor::WantDeactivation(m) => want_deactivation = Some(m),
                HkpWorldVisitor::ShouldActivateOnRigidBodyTransformChange(m) => should_activate_on_rigid_body_transform_change = Some(m),
                HkpWorldVisitor::DeactivationReferenceDistance(m) => deactivation_reference_distance = Some(m),
                HkpWorldVisitor::ToiCollisionResponseRotateNormal(m) => toi_collision_response_rotate_normal = Some(m),
                HkpWorldVisitor::MaxSectorsPerMidphaseCollideTask(m) => max_sectors_per_midphase_collide_task = Some(m),
                HkpWorldVisitor::MaxSectorsPerNarrowphaseCollideTask(m) => max_sectors_per_narrowphase_collide_task = Some(m),
                HkpWorldVisitor::ProcessToisMultithreaded(m) => process_tois_multithreaded = Some(m),
                HkpWorldVisitor::MaxEntriesPerToiMidphaseCollideTask(m) => max_entries_per_toi_midphase_collide_task = Some(m),
                HkpWorldVisitor::MaxEntriesPerToiNarrowphaseCollideTask(m) => max_entries_per_toi_narrowphase_collide_task = Some(m),
                HkpWorldVisitor::MaxNumToiCollisionPairsSinglethreaded(m) => max_num_toi_collision_pairs_singlethreaded = Some(m),
                HkpWorldVisitor::SimulationType(m) => simulation_type = Some(m),
                HkpWorldVisitor::NumToisTillAllowedPenetrationSimplifiedToi(m) => num_tois_till_allowed_penetration_simplified_toi = Some(m),
                HkpWorldVisitor::NumToisTillAllowedPenetrationToi(m) => num_tois_till_allowed_penetration_toi = Some(m),
                HkpWorldVisitor::NumToisTillAllowedPenetrationToiHigher(m) => num_tois_till_allowed_penetration_toi_higher = Some(m),
                HkpWorldVisitor::NumToisTillAllowedPenetrationToiForced(m) => num_tois_till_allowed_penetration_toi_forced = Some(m),
                HkpWorldVisitor::LastEntityUid(m) => last_entity_uid = Some(m),
                HkpWorldVisitor::LastIslandUid(m) => last_island_uid = Some(m),
                HkpWorldVisitor::LastConstraintUid(m) => last_constraint_uid = Some(m),
                HkpWorldVisitor::Phantoms(m) => phantoms = Some(m),
                HkpWorldVisitor::ActionListeners(m) => action_listeners = Some(m),
                HkpWorldVisitor::EntityListeners(m) => entity_listeners = Some(m),
                HkpWorldVisitor::PhantomListeners(m) => phantom_listeners = Some(m),
                HkpWorldVisitor::ConstraintListeners(m) => constraint_listeners = Some(m),
                HkpWorldVisitor::WorldDeletionListeners(m) => world_deletion_listeners = Some(m),
                HkpWorldVisitor::IslandActivationListeners(m) => island_activation_listeners = Some(m),
                HkpWorldVisitor::WorldPostSimulationListeners(m) => world_post_simulation_listeners = Some(m),
                HkpWorldVisitor::WorldPostIntegrateListeners(m) => world_post_integrate_listeners = Some(m),
                HkpWorldVisitor::WorldPostCollideListeners(m) => world_post_collide_listeners = Some(m),
                HkpWorldVisitor::IslandPostIntegrateListeners(m) => island_post_integrate_listeners = Some(m),
                HkpWorldVisitor::IslandPostCollideListeners(m) => island_post_collide_listeners = Some(m),
                HkpWorldVisitor::ContactListeners(m) => contact_listeners = Some(m),
                HkpWorldVisitor::ContactImpulseLimitBreachedListeners(m) => contact_impulse_limit_breached_listeners = Some(m),
                HkpWorldVisitor::WorldExtensions(m) => world_extensions = Some(m),
                HkpWorldVisitor::ViolatedConstraintArray(m) => violated_constraint_array = Some(m),
                HkpWorldVisitor::BroadPhaseBorder(m) => broad_phase_border = Some(m),
                HkpWorldVisitor::DestructionWorld(m) => destruction_world = Some(m),
                HkpWorldVisitor::NpWorld(m) => np_world = Some(m),
                HkpWorldVisitor::BroadPhaseExtents(m) => broad_phase_extents = Some(m),
                HkpWorldVisitor::BroadPhaseNumMarkers(m) => broad_phase_num_markers = Some(m),
                HkpWorldVisitor::SizeOfToiEventQueue(m) => size_of_toi_event_queue = Some(m),
                HkpWorldVisitor::BroadPhaseQuerySize(m) => broad_phase_query_size = Some(m),
                HkpWorldVisitor::BroadPhaseUpdateSize(m) => broad_phase_update_size = Some(m),
                HkpWorldVisitor::ContactPointGeneration(m) => contact_point_generation = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            simulation: simulation.unwrap_or_default().into_inner(),
            gravity: gravity.unwrap_or_default().into_inner(),
            fixed_island: fixed_island.unwrap_or_default().into_inner(),
            fixed_rigid_body: fixed_rigid_body.unwrap_or_default().into_inner(),
            active_simulation_islands: active_simulation_islands.unwrap_or_default(),
            inactive_simulation_islands: inactive_simulation_islands.unwrap_or_default(),
            dirty_simulation_islands: dirty_simulation_islands.unwrap_or_default(),
            maintenance_mgr: maintenance_mgr.unwrap_or_default().into_inner(),
            memory_watch_dog: memory_watch_dog.unwrap_or_default().into_inner(),
            assert_on_running_out_of_solver_memory: assert_on_running_out_of_solver_memory.unwrap_or_default().into_inner(),
            broad_phase: broad_phase.unwrap_or_default().into_inner(),
            kd_tree_manager: kd_tree_manager.unwrap_or_default().into_inner(),
            auto_update_tree: auto_update_tree.unwrap_or_default().into_inner(),
            broad_phase_dispatcher: broad_phase_dispatcher.unwrap_or_default().into_inner(),
            phantom_broad_phase_listener: phantom_broad_phase_listener.unwrap_or_default().into_inner(),
            entity_entity_broad_phase_listener: entity_entity_broad_phase_listener.unwrap_or_default().into_inner(),
            broad_phase_border_listener: broad_phase_border_listener.unwrap_or_default().into_inner(),
            multithreaded_simulation_job_data: multithreaded_simulation_job_data.unwrap_or_default().into_inner(),
            collision_input: collision_input.unwrap_or_default().into_inner(),
            collision_filter: collision_filter.unwrap_or_default().into_inner(),
            collision_dispatcher: collision_dispatcher.unwrap_or_default().into_inner(),
            convex_list_filter: convex_list_filter.unwrap_or_default().into_inner(),
            pending_operations: pending_operations.unwrap_or_default().into_inner(),
            pending_operations_count: pending_operations_count.unwrap_or_default().into_inner(),
            pending_body_operations_count: pending_body_operations_count.unwrap_or_default().into_inner(),
            critical_operations_lock_count: critical_operations_lock_count.unwrap_or_default().into_inner(),
            critical_operations_lock_count_for_phantoms: critical_operations_lock_count_for_phantoms.unwrap_or_default().into_inner(),
            block_executing_pending_operations: block_executing_pending_operations.unwrap_or_default().into_inner(),
            critical_operations_allowed: critical_operations_allowed.unwrap_or_default().into_inner(),
            pending_operation_queues: pending_operation_queues.unwrap_or_default().into_inner(),
            pending_operation_queue_count: pending_operation_queue_count.unwrap_or_default().into_inner(),
            multi_thread_check: multi_thread_check.unwrap_or_default(),
            process_actions_in_single_thread: process_actions_in_single_thread.unwrap_or_default().into_inner(),
            allow_integration_of_islands_without_constraints_in_a_separate_job: allow_integration_of_islands_without_constraints_in_a_separate_job.unwrap_or_default().into_inner(),
            min_desired_island_size: min_desired_island_size.unwrap_or_default().into_inner(),
            modify_constraint_critical_section: modify_constraint_critical_section.unwrap_or_default().into_inner(),
            is_locked: is_locked.unwrap_or_default().into_inner(),
            island_dirty_list_critical_section: island_dirty_list_critical_section.unwrap_or_default().into_inner(),
            property_master_lock: property_master_lock.unwrap_or_default().into_inner(),
            want_simulation_islands: want_simulation_islands.unwrap_or_default().into_inner(),
            use_hybrid_broadphase: use_hybrid_broadphase.unwrap_or_default().into_inner(),
            snap_collision_to_convex_edge_threshold: snap_collision_to_convex_edge_threshold.unwrap_or_default().into_inner(),
            snap_collision_to_concave_edge_threshold: snap_collision_to_concave_edge_threshold.unwrap_or_default().into_inner(),
            enable_toi_weld_rejection: enable_toi_weld_rejection.unwrap_or_default().into_inner(),
            want_deactivation: want_deactivation.unwrap_or_default().into_inner(),
            should_activate_on_rigid_body_transform_change: should_activate_on_rigid_body_transform_change.unwrap_or_default().into_inner(),
            deactivation_reference_distance: deactivation_reference_distance.unwrap_or_default().into_inner(),
            toi_collision_response_rotate_normal: toi_collision_response_rotate_normal.unwrap_or_default().into_inner(),
            max_sectors_per_midphase_collide_task: max_sectors_per_midphase_collide_task.unwrap_or_default().into_inner(),
            max_sectors_per_narrowphase_collide_task: max_sectors_per_narrowphase_collide_task.unwrap_or_default().into_inner(),
            process_tois_multithreaded: process_tois_multithreaded.unwrap_or_default().into_inner(),
            max_entries_per_toi_midphase_collide_task: max_entries_per_toi_midphase_collide_task.unwrap_or_default().into_inner(),
            max_entries_per_toi_narrowphase_collide_task: max_entries_per_toi_narrowphase_collide_task.unwrap_or_default().into_inner(),
            max_num_toi_collision_pairs_singlethreaded: max_num_toi_collision_pairs_singlethreaded.unwrap_or_default().into_inner(),
            simulation_type: simulation_type.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_simplified_toi: num_tois_till_allowed_penetration_simplified_toi.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi: num_tois_till_allowed_penetration_toi.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi_higher: num_tois_till_allowed_penetration_toi_higher.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi_forced: num_tois_till_allowed_penetration_toi_forced.unwrap_or_default().into_inner(),
            last_entity_uid: last_entity_uid.unwrap_or_default().into_inner(),
            last_island_uid: last_island_uid.unwrap_or_default().into_inner(),
            last_constraint_uid: last_constraint_uid.unwrap_or_default().into_inner(),
            phantoms: phantoms.unwrap_or_default(),
            action_listeners: action_listeners.unwrap_or_default(),
            entity_listeners: entity_listeners.unwrap_or_default(),
            phantom_listeners: phantom_listeners.unwrap_or_default(),
            constraint_listeners: constraint_listeners.unwrap_or_default(),
            world_deletion_listeners: world_deletion_listeners.unwrap_or_default(),
            island_activation_listeners: island_activation_listeners.unwrap_or_default(),
            world_post_simulation_listeners: world_post_simulation_listeners.unwrap_or_default(),
            world_post_integrate_listeners: world_post_integrate_listeners.unwrap_or_default(),
            world_post_collide_listeners: world_post_collide_listeners.unwrap_or_default(),
            island_post_integrate_listeners: island_post_integrate_listeners.unwrap_or_default(),
            island_post_collide_listeners: island_post_collide_listeners.unwrap_or_default(),
            contact_listeners: contact_listeners.unwrap_or_default(),
            contact_impulse_limit_breached_listeners: contact_impulse_limit_breached_listeners.unwrap_or_default(),
            world_extensions: world_extensions.unwrap_or_default(),
            violated_constraint_array: violated_constraint_array.unwrap_or_default().into_inner(),
            broad_phase_border: broad_phase_border.unwrap_or_default().into_inner(),
            destruction_world: destruction_world.unwrap_or_default().into_inner(),
            np_world: np_world.unwrap_or_default().into_inner(),
            broad_phase_extents: broad_phase_extents.unwrap_or_default(),
            broad_phase_num_markers: broad_phase_num_markers.unwrap_or_default().into_inner(),
            size_of_toi_event_queue: size_of_toi_event_queue.unwrap_or_default().into_inner(),
            broad_phase_query_size: broad_phase_query_size.unwrap_or_default().into_inner(),
            broad_phase_update_size: broad_phase_update_size.unwrap_or_default().into_inner(),
            contact_point_generation: contact_point_generation.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpWorld<'a>> for Vec<HkpWorldVisitor<'a>> {
    fn from(data: &HkpWorld<'a>) -> Self {
        vec![
            HkpWorldVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpWorldVisitor::ReferenceCount(data.reference_count.into()),
            HkpWorldVisitor::Simulation(data.simulation.clone().into()),
            HkpWorldVisitor::Gravity(data.gravity.into()),
            HkpWorldVisitor::FixedIsland(data.fixed_island.clone().into()),
            HkpWorldVisitor::FixedRigidBody(data.fixed_rigid_body.clone().into()),
            HkpWorldVisitor::ActiveSimulationIslands(data.active_simulation_islands.clone()),
            HkpWorldVisitor::InactiveSimulationIslands(data.inactive_simulation_islands.clone()),
            HkpWorldVisitor::DirtySimulationIslands(data.dirty_simulation_islands.clone()),
            HkpWorldVisitor::MaintenanceMgr(data.maintenance_mgr.clone().into()),
            HkpWorldVisitor::MemoryWatchDog(data.memory_watch_dog.clone().into()),
            HkpWorldVisitor::AssertOnRunningOutOfSolverMemory(data.assert_on_running_out_of_solver_memory.into()),
            HkpWorldVisitor::BroadPhase(data.broad_phase.clone().into()),
            HkpWorldVisitor::KdTreeManager(data.kd_tree_manager.clone().into()),
            HkpWorldVisitor::AutoUpdateTree(data.auto_update_tree.into()),
            HkpWorldVisitor::BroadPhaseDispatcher(data.broad_phase_dispatcher.clone().into()),
            HkpWorldVisitor::PhantomBroadPhaseListener(data.phantom_broad_phase_listener.clone().into()),
            HkpWorldVisitor::EntityEntityBroadPhaseListener(data.entity_entity_broad_phase_listener.clone().into()),
            HkpWorldVisitor::BroadPhaseBorderListener(data.broad_phase_border_listener.clone().into()),
            HkpWorldVisitor::MultithreadedSimulationJobData(data.multithreaded_simulation_job_data.clone().into()),
            HkpWorldVisitor::CollisionInput(data.collision_input.clone().into()),
            HkpWorldVisitor::CollisionFilter(data.collision_filter.clone().into()),
            HkpWorldVisitor::CollisionDispatcher(data.collision_dispatcher.clone().into()),
            HkpWorldVisitor::ConvexListFilter(data.convex_list_filter.clone().into()),
            HkpWorldVisitor::PendingOperations(data.pending_operations.clone().into()),
            HkpWorldVisitor::PendingOperationsCount(data.pending_operations_count.into()),
            HkpWorldVisitor::PendingBodyOperationsCount(data.pending_body_operations_count.into()),
            HkpWorldVisitor::CriticalOperationsLockCount(data.critical_operations_lock_count.into()),
            HkpWorldVisitor::CriticalOperationsLockCountForPhantoms(data.critical_operations_lock_count_for_phantoms.into()),
            HkpWorldVisitor::BlockExecutingPendingOperations(data.block_executing_pending_operations.into()),
            HkpWorldVisitor::CriticalOperationsAllowed(data.critical_operations_allowed.into()),
            HkpWorldVisitor::PendingOperationQueues(data.pending_operation_queues.clone().into()),
            HkpWorldVisitor::PendingOperationQueueCount(data.pending_operation_queue_count.into()),
            HkpWorldVisitor::MultiThreadCheck(data.multi_thread_check.clone()),
            HkpWorldVisitor::ProcessActionsInSingleThread(data.process_actions_in_single_thread.into()),
            HkpWorldVisitor::AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(data.allow_integration_of_islands_without_constraints_in_a_separate_job.into()),
            HkpWorldVisitor::MinDesiredIslandSize(data.min_desired_island_size.into()),
            HkpWorldVisitor::ModifyConstraintCriticalSection(data.modify_constraint_critical_section.clone().into()),
            HkpWorldVisitor::IsLocked(data.is_locked.into()),
            HkpWorldVisitor::IslandDirtyListCriticalSection(data.island_dirty_list_critical_section.clone().into()),
            HkpWorldVisitor::PropertyMasterLock(data.property_master_lock.clone().into()),
            HkpWorldVisitor::WantSimulationIslands(data.want_simulation_islands.into()),
            HkpWorldVisitor::UseHybridBroadphase(data.use_hybrid_broadphase.into()),
            HkpWorldVisitor::SnapCollisionToConvexEdgeThreshold(data.snap_collision_to_convex_edge_threshold.into()),
            HkpWorldVisitor::SnapCollisionToConcaveEdgeThreshold(data.snap_collision_to_concave_edge_threshold.into()),
            HkpWorldVisitor::EnableToiWeldRejection(data.enable_toi_weld_rejection.into()),
            HkpWorldVisitor::WantDeactivation(data.want_deactivation.into()),
            HkpWorldVisitor::ShouldActivateOnRigidBodyTransformChange(data.should_activate_on_rigid_body_transform_change.into()),
            HkpWorldVisitor::DeactivationReferenceDistance(data.deactivation_reference_distance.into()),
            HkpWorldVisitor::ToiCollisionResponseRotateNormal(data.toi_collision_response_rotate_normal.into()),
            HkpWorldVisitor::MaxSectorsPerMidphaseCollideTask(data.max_sectors_per_midphase_collide_task.into()),
            HkpWorldVisitor::MaxSectorsPerNarrowphaseCollideTask(data.max_sectors_per_narrowphase_collide_task.into()),
            HkpWorldVisitor::ProcessToisMultithreaded(data.process_tois_multithreaded.into()),
            HkpWorldVisitor::MaxEntriesPerToiMidphaseCollideTask(data.max_entries_per_toi_midphase_collide_task.into()),
            HkpWorldVisitor::MaxEntriesPerToiNarrowphaseCollideTask(data.max_entries_per_toi_narrowphase_collide_task.into()),
            HkpWorldVisitor::MaxNumToiCollisionPairsSinglethreaded(data.max_num_toi_collision_pairs_singlethreaded.into()),
            HkpWorldVisitor::SimulationType(data.simulation_type.into()),
            HkpWorldVisitor::NumToisTillAllowedPenetrationSimplifiedToi(data.num_tois_till_allowed_penetration_simplified_toi.into()),
            HkpWorldVisitor::NumToisTillAllowedPenetrationToi(data.num_tois_till_allowed_penetration_toi.into()),
            HkpWorldVisitor::NumToisTillAllowedPenetrationToiHigher(data.num_tois_till_allowed_penetration_toi_higher.into()),
            HkpWorldVisitor::NumToisTillAllowedPenetrationToiForced(data.num_tois_till_allowed_penetration_toi_forced.into()),
            HkpWorldVisitor::LastEntityUid(data.last_entity_uid.into()),
            HkpWorldVisitor::LastIslandUid(data.last_island_uid.into()),
            HkpWorldVisitor::LastConstraintUid(data.last_constraint_uid.into()),
            HkpWorldVisitor::Phantoms(data.phantoms.clone()),
            HkpWorldVisitor::ActionListeners(data.action_listeners.clone()),
            HkpWorldVisitor::EntityListeners(data.entity_listeners.clone()),
            HkpWorldVisitor::PhantomListeners(data.phantom_listeners.clone()),
            HkpWorldVisitor::ConstraintListeners(data.constraint_listeners.clone()),
            HkpWorldVisitor::WorldDeletionListeners(data.world_deletion_listeners.clone()),
            HkpWorldVisitor::IslandActivationListeners(data.island_activation_listeners.clone()),
            HkpWorldVisitor::WorldPostSimulationListeners(data.world_post_simulation_listeners.clone()),
            HkpWorldVisitor::WorldPostIntegrateListeners(data.world_post_integrate_listeners.clone()),
            HkpWorldVisitor::WorldPostCollideListeners(data.world_post_collide_listeners.clone()),
            HkpWorldVisitor::IslandPostIntegrateListeners(data.island_post_integrate_listeners.clone()),
            HkpWorldVisitor::IslandPostCollideListeners(data.island_post_collide_listeners.clone()),
            HkpWorldVisitor::ContactListeners(data.contact_listeners.clone()),
            HkpWorldVisitor::ContactImpulseLimitBreachedListeners(data.contact_impulse_limit_breached_listeners.clone()),
            HkpWorldVisitor::WorldExtensions(data.world_extensions.clone()),
            HkpWorldVisitor::ViolatedConstraintArray(data.violated_constraint_array.clone().into()),
            HkpWorldVisitor::BroadPhaseBorder(data.broad_phase_border.clone().into()),
            HkpWorldVisitor::DestructionWorld(data.destruction_world.clone().into()),
            HkpWorldVisitor::NpWorld(data.np_world.clone().into()),
            HkpWorldVisitor::BroadPhaseExtents(data.broad_phase_extents.clone()),
            HkpWorldVisitor::BroadPhaseNumMarkers(data.broad_phase_num_markers.into()),
            HkpWorldVisitor::SizeOfToiEventQueue(data.size_of_toi_event_queue.into()),
            HkpWorldVisitor::BroadPhaseQuerySize(data.broad_phase_query_size.into()),
            HkpWorldVisitor::BroadPhaseUpdateSize(data.broad_phase_update_size.into()),
            HkpWorldVisitor::ContactPointGeneration(data.contact_point_generation.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpWorld<'de> {
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
enum HkpWorldVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "simulation")]
    Simulation(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "gravity")]
    Gravity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "fixedIsland", skip_serializing)]
    FixedIsland(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "fixedRigidBody")]
    FixedRigidBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeSimulationIslands", skip_serializing)]
    ActiveSimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "inactiveSimulationIslands", skip_serializing)]
    InactiveSimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "dirtySimulationIslands", skip_serializing)]
    DirtySimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "maintenanceMgr", skip_serializing)]
    MaintenanceMgr(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "memoryWatchDog", skip_serializing)]
    MemoryWatchDog(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "assertOnRunningOutOfSolverMemory", skip_serializing)]
    AssertOnRunningOutOfSolverMemory(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "broadPhase", skip_serializing)]
    BroadPhase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "kdTreeManager", skip_serializing)]
    KdTreeManager(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "autoUpdateTree")]
    AutoUpdateTree(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "broadPhaseDispatcher", skip_serializing)]
    BroadPhaseDispatcher(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantomBroadPhaseListener", skip_serializing)]
    PhantomBroadPhaseListener(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "entityEntityBroadPhaseListener", skip_serializing)]
    EntityEntityBroadPhaseListener(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "broadPhaseBorderListener", skip_serializing)]
    BroadPhaseBorderListener(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "multithreadedSimulationJobData", skip_serializing)]
    MultithreadedSimulationJobData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "collisionInput", skip_serializing)]
    CollisionInput(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "collisionFilter", skip_serializing)]
    CollisionFilter(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "collisionDispatcher", skip_serializing)]
    CollisionDispatcher(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "convexListFilter", skip_serializing)]
    ConvexListFilter(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pendingOperations", skip_serializing)]
    PendingOperations(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pendingOperationsCount")]
    PendingOperationsCount(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "pendingBodyOperationsCount", skip_serializing)]
    PendingBodyOperationsCount(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "criticalOperationsLockCount")]
    CriticalOperationsLockCount(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "criticalOperationsLockCountForPhantoms")]
    CriticalOperationsLockCountForPhantoms(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "blockExecutingPendingOperations")]
    BlockExecutingPendingOperations(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "criticalOperationsAllowed")]
    CriticalOperationsAllowed(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "pendingOperationQueues", skip_serializing)]
    PendingOperationQueues(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pendingOperationQueueCount")]
    PendingOperationQueueCount(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "multiThreadCheck", skip_serializing)]
    MultiThreadCheck(SingleClass<HkMultiThreadCheck>),
    /// Visitor fields
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "modifyConstraintCriticalSection", skip_serializing)]
    ModifyConstraintCriticalSection(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "islandDirtyListCriticalSection", skip_serializing)]
    IslandDirtyListCriticalSection(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "propertyMasterLock", skip_serializing)]
    PropertyMasterLock(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "wantSimulationIslands")]
    WantSimulationIslands(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useHybridBroadphase", skip_serializing)]
    UseHybridBroadphase(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "wantDeactivation")]
    WantDeactivation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "simulationType", skip_serializing)]
    SimulationType(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lastEntityUid")]
    LastEntityUid(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "lastIslandUid")]
    LastIslandUid(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "lastConstraintUid")]
    LastConstraintUid(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "phantoms")]
    Phantoms(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "actionListeners", skip_serializing)]
    ActionListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "phantomListeners", skip_serializing)]
    PhantomListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraintListeners", skip_serializing)]
    ConstraintListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldDeletionListeners", skip_serializing)]
    WorldDeletionListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "islandActivationListeners", skip_serializing)]
    IslandActivationListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldPostSimulationListeners", skip_serializing)]
    WorldPostSimulationListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldPostIntegrateListeners", skip_serializing)]
    WorldPostIntegrateListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldPostCollideListeners", skip_serializing)]
    WorldPostCollideListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "islandPostIntegrateListeners", skip_serializing)]
    IslandPostIntegrateListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "islandPostCollideListeners", skip_serializing)]
    IslandPostCollideListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "contactImpulseLimitBreachedListeners", skip_serializing)]
    ContactImpulseLimitBreachedListeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldExtensions", skip_serializing)]
    WorldExtensions(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "violatedConstraintArray", skip_serializing)]
    ViolatedConstraintArray(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "broadPhaseBorder", skip_serializing)]
    BroadPhaseBorder(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "npWorld", skip_serializing)]
    NpWorld(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "broadPhaseExtents")]
    BroadPhaseExtents(CStyleArrayVector<Vector4<f32>, 2>),
    /// Visitor fields
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "broadPhaseUpdateSize")]
    BroadPhaseUpdateSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contactPointGeneration", skip_serializing)]
    ContactPointGeneration(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("simulation" => Simulation(Primitive<Cow<'de, str>>)),
    ("gravity" => Gravity(Primitive<Vector4<f32>>)),
    ("fixedIsland" => FixedIsland(Primitive<Cow<'de, str>>)),
    ("fixedRigidBody" => FixedRigidBody(Primitive<Cow<'de, str>>)),
    ("activeSimulationIslands" => ActiveSimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("inactiveSimulationIslands" => InactiveSimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("dirtySimulationIslands" => DirtySimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("maintenanceMgr" => MaintenanceMgr(Primitive<Cow<'de, str>>)),
    ("memoryWatchDog" => MemoryWatchDog(Primitive<Cow<'de, str>>)),
    ("assertOnRunningOutOfSolverMemory" => AssertOnRunningOutOfSolverMemory(Primitive<bool>)),
    ("broadPhase" => BroadPhase(Primitive<Cow<'de, str>>)),
    ("kdTreeManager" => KdTreeManager(Primitive<Cow<'de, str>>)),
    ("autoUpdateTree" => AutoUpdateTree(Primitive<bool>)),
    ("broadPhaseDispatcher" => BroadPhaseDispatcher(Primitive<Cow<'de, str>>)),
    ("phantomBroadPhaseListener" => PhantomBroadPhaseListener(Primitive<Cow<'de, str>>)),
    ("entityEntityBroadPhaseListener" => EntityEntityBroadPhaseListener(Primitive<Cow<'de, str>>)),
    ("broadPhaseBorderListener" => BroadPhaseBorderListener(Primitive<Cow<'de, str>>)),
    ("multithreadedSimulationJobData" => MultithreadedSimulationJobData(Primitive<Cow<'de, str>>)),
    ("collisionInput" => CollisionInput(Primitive<Cow<'de, str>>)),
    ("collisionFilter" => CollisionFilter(Primitive<Cow<'de, str>>)),
    ("collisionDispatcher" => CollisionDispatcher(Primitive<Cow<'de, str>>)),
    ("convexListFilter" => ConvexListFilter(Primitive<Cow<'de, str>>)),
    ("pendingOperations" => PendingOperations(Primitive<Cow<'de, str>>)),
    ("pendingOperationsCount" => PendingOperationsCount(Primitive<i32>)),
    ("pendingBodyOperationsCount" => PendingBodyOperationsCount(Primitive<i32>)),
    ("criticalOperationsLockCount" => CriticalOperationsLockCount(Primitive<i32>)),
    ("criticalOperationsLockCountForPhantoms" => CriticalOperationsLockCountForPhantoms(Primitive<i32>)),
    ("blockExecutingPendingOperations" => BlockExecutingPendingOperations(Primitive<bool>)),
    ("criticalOperationsAllowed" => CriticalOperationsAllowed(Primitive<bool>)),
    ("pendingOperationQueues" => PendingOperationQueues(Primitive<Cow<'de, str>>)),
    ("pendingOperationQueueCount" => PendingOperationQueueCount(Primitive<i32>)),
    ("multiThreadCheck" => MultiThreadCheck(SingleClass<HkMultiThreadCheck>)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("modifyConstraintCriticalSection" => ModifyConstraintCriticalSection(Primitive<Cow<'de, str>>)),
    ("isLocked" => IsLocked(Primitive<i32>)),
    ("islandDirtyListCriticalSection" => IslandDirtyListCriticalSection(Primitive<Cow<'de, str>>)),
    ("propertyMasterLock" => PropertyMasterLock(Primitive<Cow<'de, str>>)),
    ("wantSimulationIslands" => WantSimulationIslands(Primitive<bool>)),
    ("useHybridBroadphase" => UseHybridBroadphase(Primitive<bool>)),
    ("snapCollisionToConvexEdgeThreshold" => SnapCollisionToConvexEdgeThreshold(Primitive<f32>)),
    ("snapCollisionToConcaveEdgeThreshold" => SnapCollisionToConcaveEdgeThreshold(Primitive<f32>)),
    ("enableToiWeldRejection" => EnableToiWeldRejection(Primitive<bool>)),
    ("wantDeactivation" => WantDeactivation(Primitive<bool>)),
    ("shouldActivateOnRigidBodyTransformChange" => ShouldActivateOnRigidBodyTransformChange(Primitive<bool>)),
    ("deactivationReferenceDistance" => DeactivationReferenceDistance(Primitive<f32>)),
    ("toiCollisionResponseRotateNormal" => ToiCollisionResponseRotateNormal(Primitive<f32>)),
    ("maxSectorsPerMidphaseCollideTask" => MaxSectorsPerMidphaseCollideTask(Primitive<i32>)),
    ("maxSectorsPerNarrowphaseCollideTask" => MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>)),
    ("processToisMultithreaded" => ProcessToisMultithreaded(Primitive<bool>)),
    ("maxEntriesPerToiMidphaseCollideTask" => MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>)),
    ("maxEntriesPerToiNarrowphaseCollideTask" => MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>)),
    ("maxNumToiCollisionPairsSinglethreaded" => MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>)),
    ("simulationType" => SimulationType(Primitive<()>)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(Primitive<f32>)),
    ("lastEntityUid" => LastEntityUid(Primitive<u32>)),
    ("lastIslandUid" => LastIslandUid(Primitive<u32>)),
    ("lastConstraintUid" => LastConstraintUid(Primitive<u32>)),
    ("phantoms" => Phantoms(HkArrayRef<Cow<'de, str>>)),
    ("actionListeners" => ActionListeners(HkArrayRef<Cow<'de, str>>)),
    ("entityListeners" => EntityListeners(HkArrayRef<Cow<'de, str>>)),
    ("phantomListeners" => PhantomListeners(HkArrayRef<Cow<'de, str>>)),
    ("constraintListeners" => ConstraintListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldDeletionListeners" => WorldDeletionListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandActivationListeners" => IslandActivationListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostSimulationListeners" => WorldPostSimulationListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostIntegrateListeners" => WorldPostIntegrateListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostCollideListeners" => WorldPostCollideListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandPostIntegrateListeners" => IslandPostIntegrateListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandPostCollideListeners" => IslandPostCollideListeners(HkArrayRef<Cow<'de, str>>)),
    ("contactListeners" => ContactListeners(HkArrayRef<Cow<'de, str>>)),
    ("contactImpulseLimitBreachedListeners" => ContactImpulseLimitBreachedListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldExtensions" => WorldExtensions(HkArrayRef<Cow<'de, str>>)),
    ("violatedConstraintArray" => ViolatedConstraintArray(Primitive<Cow<'de, str>>)),
    ("broadPhaseBorder" => BroadPhaseBorder(Primitive<Cow<'de, str>>)),
    ("destructionWorld" => DestructionWorld(Primitive<Cow<'de, str>>)),
    ("npWorld" => NpWorld(Primitive<Cow<'de, str>>)),
    ("broadPhaseExtents" => BroadPhaseExtents(CStyleArrayVector<Vector4<f32>, 2>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("broadPhaseUpdateSize" => BroadPhaseUpdateSize(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ReintegrationRecollideMode {
    #[serde(rename = "RR_MODE_REINTEGRATE")]
    #[default]
    RrModeReintegrate = 1,
    #[serde(rename = "RR_MODE_RECOLLIDE_BROADPHASE")]
    RrModeRecollideBroadphase = 2,
    #[serde(rename = "RR_MODE_RECOLLIDE_NARROWPHASE")]
    RrModeRecollideNarrowphase = 4,
    #[serde(rename = "RR_MODE_ALL")]
    RrModeAll = 7,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MtAccessChecking {
    #[serde(rename = "MT_ACCESS_CHECKING_ENABLED")]
    #[default]
    MtAccessCheckingEnabled = 0,
    #[serde(rename = "MT_ACCESS_CHECKING_DISABLED")]
    MtAccessCheckingDisabled = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum CachedAabbUpdate {
    #[serde(rename = "SHIFT_BROADPHASE_UPDATE_ENTITY_AABBS")]
    #[default]
    ShiftBroadphaseUpdateEntityAabbs = 0,
    #[serde(rename = "SHIFT_BROADPHASE_IGNORE_ENTITY_AABBS")]
    ShiftBroadphaseIgnoreEntityAabbs = 1,
}
