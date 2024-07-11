//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorldCinfo`
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

/// `hkpWorldCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa5255445`
/// -   version: 11
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpWorldCinfo<'a> {
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
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub gravity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_query_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contactRestingVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub contact_resting_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorderBehaviour"`
    /// -   type: `enum BroadPhaseBorderBehaviour`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_border_behaviour: BroadPhaseBorderBehaviour,
    /// # C++ Class Fields Info
    /// -   name:`"mtPostponeAndSortBroadPhaseBorderCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    pub mt_postpone_and_sort_broad_phase_border_callbacks: bool,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseWorldAabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_world_aabb: SingleClass<HkAabb>,
    /// # C++ Class Fields Info
    /// -   name:`"useKdTree"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub use_kd_tree: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useMultipleTree"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    pub use_multiple_tree: bool,
    /// # C++ Class Fields Info
    /// -   name:`"treeUpdateType"`
    /// -   type: `enum TreeUpdateType`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    pub tree_update_type: TreeUpdateType,
    /// # C++ Class Fields Info
    /// -   name:`"autoUpdateKdTree"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    pub auto_update_kd_tree: bool,
    /// # C++ Class Fields Info
    /// -   name:`"collisionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub collision_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub collision_filter: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"convexListFilter"`
    /// -   type: `struct hkpConvexListFilter*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub convex_list_filter: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"expectedMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub expected_max_linear_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub size_of_toi_event_queue: i32,
    /// # C++ Class Fields Info
    /// -   name:`"expectedMinPsiDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub expected_min_psi_delta_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"memoryWatchDog"`
    /// -   type: `struct hkWorldMemoryAvailableWatchDog*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub memory_watch_dog: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_num_markers: i32,
    /// # C++ Class Fields Info
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum ContactPointGeneration`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub contact_point_generation: ContactPointGeneration,
    /// # C++ Class Fields Info
    /// -   name:`"allowToSkipConfirmedCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 117
    /// -  flags: `FLAGS_NONE`
    pub allow_to_skip_confirmed_callbacks: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 118
    /// -  flags: `FLAGS_NONE`
    pub use_hybrid_broadphase: bool,
    /// # C++ Class Fields Info
    /// -   name:`"solverTau"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub solver_tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"solverDamp"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub solver_damp: f32,
    /// # C++ Class Fields Info
    /// -   name:`"solverIterations"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub solver_iterations: i32,
    /// # C++ Class Fields Info
    /// -   name:`"solverMicrosteps"`
    /// -   type: `hkInt32`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub solver_microsteps: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxConstraintViolation"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub max_constraint_violation: f32,
    /// # C++ Class Fields Info
    /// -   name:`"forceCoherentConstraintOrderingInSolver"`
    /// -   type: `hkBool`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub force_coherent_constraint_ordering_in_solver: bool,
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub snap_collision_to_convex_edge_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub snap_collision_to_concave_edge_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub enable_toi_weld_rejection: bool,
    /// # C++ Class Fields Info
    /// -   name:`"enableDeprecatedWelding"`
    /// -   type: `hkBool`
    /// - offset: 153
    /// -  flags: `FLAGS_NONE`
    pub enable_deprecated_welding: bool,
    /// # C++ Class Fields Info
    /// -   name:`"iterativeLinearCastEarlyOutDistance"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub iterative_linear_cast_early_out_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"iterativeLinearCastMaxIterations"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub iterative_linear_cast_max_iterations: i32,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationNumInactiveFramesSelectFlag0"`
    /// -   type: `hkUint8`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub deactivation_num_inactive_frames_select_flag_0: u8,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationNumInactiveFramesSelectFlag1"`
    /// -   type: `hkUint8`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    pub deactivation_num_inactive_frames_select_flag_1: u8,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    pub deactivation_integrate_counter: u8,
    /// # C++ Class Fields Info
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 167
    /// -  flags: `FLAGS_NONE`
    pub should_activate_on_rigid_body_transform_change: bool,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub deactivation_reference_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub toi_collision_response_rotate_normal: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub max_sectors_per_midphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub max_sectors_per_narrowphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub process_tois_multithreaded: bool,
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    pub max_entries_per_toi_midphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub max_entries_per_toi_narrowphase_collide_task: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    pub max_num_toi_collision_pairs_singlethreaded: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_simplified_toi: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi_higher: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub num_tois_till_allowed_penetration_toi_forced: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enableDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    pub enable_deactivation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"simulationType"`
    /// -   type: `enum SimulationType`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    pub simulation_type: SimulationType,
    /// # C++ Class Fields Info
    /// -   name:`"enableSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    pub enable_simulation_islands: bool,
    /// # C++ Class Fields Info
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    pub min_desired_island_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub process_actions_in_single_thread: bool,
    /// # C++ Class Fields Info
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 225
    /// -  flags: `FLAGS_NONE`
    pub allow_integration_of_islands_without_constraints_in_a_separate_job: bool,
    /// # C++ Class Fields Info
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    pub frame_marker_psi_snap: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fireCollisionCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    pub fire_collision_callbacks: bool,
}

impl Serialize for HkpWorldCinfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpWorldCinfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpWorldCinfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpWorldCinfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpWorldCinfoVisitor<'a>>> for HkpWorldCinfo<'a> {
    fn from(_values: Vec<HkpWorldCinfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut gravity = None;
            let mut broad_phase_query_size = None;
            let mut contact_resting_velocity = None;
            let mut broad_phase_border_behaviour = None;
            let mut mt_postpone_and_sort_broad_phase_border_callbacks = None;
            let mut broad_phase_world_aabb = None;
            let mut use_kd_tree = None;
            let mut use_multiple_tree = None;
            let mut tree_update_type = None;
            let mut auto_update_kd_tree = None;
            let mut collision_tolerance = None;
            let mut collision_filter = None;
            let mut convex_list_filter = None;
            let mut expected_max_linear_velocity = None;
            let mut size_of_toi_event_queue = None;
            let mut expected_min_psi_delta_time = None;
            let mut memory_watch_dog = None;
            let mut broad_phase_num_markers = None;
            let mut contact_point_generation = None;
            let mut allow_to_skip_confirmed_callbacks = None;
            let mut use_hybrid_broadphase = None;
            let mut solver_tau = None;
            let mut solver_damp = None;
            let mut solver_iterations = None;
            let mut solver_microsteps = None;
            let mut max_constraint_violation = None;
            let mut force_coherent_constraint_ordering_in_solver = None;
            let mut snap_collision_to_convex_edge_threshold = None;
            let mut snap_collision_to_concave_edge_threshold = None;
            let mut enable_toi_weld_rejection = None;
            let mut enable_deprecated_welding = None;
            let mut iterative_linear_cast_early_out_distance = None;
            let mut iterative_linear_cast_max_iterations = None;
            let mut deactivation_num_inactive_frames_select_flag_0 = None;
            let mut deactivation_num_inactive_frames_select_flag_1 = None;
            let mut deactivation_integrate_counter = None;
            let mut should_activate_on_rigid_body_transform_change = None;
            let mut deactivation_reference_distance = None;
            let mut toi_collision_response_rotate_normal = None;
            let mut max_sectors_per_midphase_collide_task = None;
            let mut max_sectors_per_narrowphase_collide_task = None;
            let mut process_tois_multithreaded = None;
            let mut max_entries_per_toi_midphase_collide_task = None;
            let mut max_entries_per_toi_narrowphase_collide_task = None;
            let mut max_num_toi_collision_pairs_singlethreaded = None;
            let mut num_tois_till_allowed_penetration_simplified_toi = None;
            let mut num_tois_till_allowed_penetration_toi = None;
            let mut num_tois_till_allowed_penetration_toi_higher = None;
            let mut num_tois_till_allowed_penetration_toi_forced = None;
            let mut enable_deactivation = None;
            let mut simulation_type = None;
            let mut enable_simulation_islands = None;
            let mut min_desired_island_size = None;
            let mut process_actions_in_single_thread = None;
            let mut allow_integration_of_islands_without_constraints_in_a_separate_job = None;
            let mut frame_marker_psi_snap = None;
            let mut fire_collision_callbacks = None;


        for _value in _values {
            match _value {
                HkpWorldCinfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpWorldCinfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpWorldCinfoVisitor::Gravity(m) => gravity = Some(m),
                HkpWorldCinfoVisitor::BroadPhaseQuerySize(m) => broad_phase_query_size = Some(m),
                HkpWorldCinfoVisitor::ContactRestingVelocity(m) => contact_resting_velocity = Some(m),
                HkpWorldCinfoVisitor::BroadPhaseBorderBehaviour(m) => broad_phase_border_behaviour = Some(m),
                HkpWorldCinfoVisitor::MtPostponeAndSortBroadPhaseBorderCallbacks(m) => mt_postpone_and_sort_broad_phase_border_callbacks = Some(m),
                HkpWorldCinfoVisitor::BroadPhaseWorldAabb(m) => broad_phase_world_aabb = Some(m),
                HkpWorldCinfoVisitor::UseKdTree(m) => use_kd_tree = Some(m),
                HkpWorldCinfoVisitor::UseMultipleTree(m) => use_multiple_tree = Some(m),
                HkpWorldCinfoVisitor::TreeUpdateType(m) => tree_update_type = Some(m),
                HkpWorldCinfoVisitor::AutoUpdateKdTree(m) => auto_update_kd_tree = Some(m),
                HkpWorldCinfoVisitor::CollisionTolerance(m) => collision_tolerance = Some(m),
                HkpWorldCinfoVisitor::CollisionFilter(m) => collision_filter = Some(m),
                HkpWorldCinfoVisitor::ConvexListFilter(m) => convex_list_filter = Some(m),
                HkpWorldCinfoVisitor::ExpectedMaxLinearVelocity(m) => expected_max_linear_velocity = Some(m),
                HkpWorldCinfoVisitor::SizeOfToiEventQueue(m) => size_of_toi_event_queue = Some(m),
                HkpWorldCinfoVisitor::ExpectedMinPsiDeltaTime(m) => expected_min_psi_delta_time = Some(m),
                HkpWorldCinfoVisitor::MemoryWatchDog(m) => memory_watch_dog = Some(m),
                HkpWorldCinfoVisitor::BroadPhaseNumMarkers(m) => broad_phase_num_markers = Some(m),
                HkpWorldCinfoVisitor::ContactPointGeneration(m) => contact_point_generation = Some(m),
                HkpWorldCinfoVisitor::AllowToSkipConfirmedCallbacks(m) => allow_to_skip_confirmed_callbacks = Some(m),
                HkpWorldCinfoVisitor::UseHybridBroadphase(m) => use_hybrid_broadphase = Some(m),
                HkpWorldCinfoVisitor::SolverTau(m) => solver_tau = Some(m),
                HkpWorldCinfoVisitor::SolverDamp(m) => solver_damp = Some(m),
                HkpWorldCinfoVisitor::SolverIterations(m) => solver_iterations = Some(m),
                HkpWorldCinfoVisitor::SolverMicrosteps(m) => solver_microsteps = Some(m),
                HkpWorldCinfoVisitor::MaxConstraintViolation(m) => max_constraint_violation = Some(m),
                HkpWorldCinfoVisitor::ForceCoherentConstraintOrderingInSolver(m) => force_coherent_constraint_ordering_in_solver = Some(m),
                HkpWorldCinfoVisitor::SnapCollisionToConvexEdgeThreshold(m) => snap_collision_to_convex_edge_threshold = Some(m),
                HkpWorldCinfoVisitor::SnapCollisionToConcaveEdgeThreshold(m) => snap_collision_to_concave_edge_threshold = Some(m),
                HkpWorldCinfoVisitor::EnableToiWeldRejection(m) => enable_toi_weld_rejection = Some(m),
                HkpWorldCinfoVisitor::EnableDeprecatedWelding(m) => enable_deprecated_welding = Some(m),
                HkpWorldCinfoVisitor::IterativeLinearCastEarlyOutDistance(m) => iterative_linear_cast_early_out_distance = Some(m),
                HkpWorldCinfoVisitor::IterativeLinearCastMaxIterations(m) => iterative_linear_cast_max_iterations = Some(m),
                HkpWorldCinfoVisitor::DeactivationNumInactiveFramesSelectFlag0(m) => deactivation_num_inactive_frames_select_flag_0 = Some(m),
                HkpWorldCinfoVisitor::DeactivationNumInactiveFramesSelectFlag1(m) => deactivation_num_inactive_frames_select_flag_1 = Some(m),
                HkpWorldCinfoVisitor::DeactivationIntegrateCounter(m) => deactivation_integrate_counter = Some(m),
                HkpWorldCinfoVisitor::ShouldActivateOnRigidBodyTransformChange(m) => should_activate_on_rigid_body_transform_change = Some(m),
                HkpWorldCinfoVisitor::DeactivationReferenceDistance(m) => deactivation_reference_distance = Some(m),
                HkpWorldCinfoVisitor::ToiCollisionResponseRotateNormal(m) => toi_collision_response_rotate_normal = Some(m),
                HkpWorldCinfoVisitor::MaxSectorsPerMidphaseCollideTask(m) => max_sectors_per_midphase_collide_task = Some(m),
                HkpWorldCinfoVisitor::MaxSectorsPerNarrowphaseCollideTask(m) => max_sectors_per_narrowphase_collide_task = Some(m),
                HkpWorldCinfoVisitor::ProcessToisMultithreaded(m) => process_tois_multithreaded = Some(m),
                HkpWorldCinfoVisitor::MaxEntriesPerToiMidphaseCollideTask(m) => max_entries_per_toi_midphase_collide_task = Some(m),
                HkpWorldCinfoVisitor::MaxEntriesPerToiNarrowphaseCollideTask(m) => max_entries_per_toi_narrowphase_collide_task = Some(m),
                HkpWorldCinfoVisitor::MaxNumToiCollisionPairsSinglethreaded(m) => max_num_toi_collision_pairs_singlethreaded = Some(m),
                HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationSimplifiedToi(m) => num_tois_till_allowed_penetration_simplified_toi = Some(m),
                HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToi(m) => num_tois_till_allowed_penetration_toi = Some(m),
                HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToiHigher(m) => num_tois_till_allowed_penetration_toi_higher = Some(m),
                HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToiForced(m) => num_tois_till_allowed_penetration_toi_forced = Some(m),
                HkpWorldCinfoVisitor::EnableDeactivation(m) => enable_deactivation = Some(m),
                HkpWorldCinfoVisitor::SimulationType(m) => simulation_type = Some(m),
                HkpWorldCinfoVisitor::EnableSimulationIslands(m) => enable_simulation_islands = Some(m),
                HkpWorldCinfoVisitor::MinDesiredIslandSize(m) => min_desired_island_size = Some(m),
                HkpWorldCinfoVisitor::ProcessActionsInSingleThread(m) => process_actions_in_single_thread = Some(m),
                HkpWorldCinfoVisitor::AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(m) => allow_integration_of_islands_without_constraints_in_a_separate_job = Some(m),
                HkpWorldCinfoVisitor::FrameMarkerPsiSnap(m) => frame_marker_psi_snap = Some(m),
                HkpWorldCinfoVisitor::FireCollisionCallbacks(m) => fire_collision_callbacks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            gravity: gravity.unwrap_or_default().into_inner(),
            broad_phase_query_size: broad_phase_query_size.unwrap_or_default().into_inner(),
            contact_resting_velocity: contact_resting_velocity.unwrap_or_default().into_inner(),
            broad_phase_border_behaviour: broad_phase_border_behaviour.unwrap_or_default().into_inner(),
            mt_postpone_and_sort_broad_phase_border_callbacks: mt_postpone_and_sort_broad_phase_border_callbacks.unwrap_or_default().into_inner(),
            broad_phase_world_aabb: broad_phase_world_aabb.unwrap_or_default(),
            use_kd_tree: use_kd_tree.unwrap_or_default().into_inner(),
            use_multiple_tree: use_multiple_tree.unwrap_or_default().into_inner(),
            tree_update_type: tree_update_type.unwrap_or_default().into_inner(),
            auto_update_kd_tree: auto_update_kd_tree.unwrap_or_default().into_inner(),
            collision_tolerance: collision_tolerance.unwrap_or_default().into_inner(),
            collision_filter: collision_filter.unwrap_or_default().into_inner(),
            convex_list_filter: convex_list_filter.unwrap_or_default().into_inner(),
            expected_max_linear_velocity: expected_max_linear_velocity.unwrap_or_default().into_inner(),
            size_of_toi_event_queue: size_of_toi_event_queue.unwrap_or_default().into_inner(),
            expected_min_psi_delta_time: expected_min_psi_delta_time.unwrap_or_default().into_inner(),
            memory_watch_dog: memory_watch_dog.unwrap_or_default().into_inner(),
            broad_phase_num_markers: broad_phase_num_markers.unwrap_or_default().into_inner(),
            contact_point_generation: contact_point_generation.unwrap_or_default().into_inner(),
            allow_to_skip_confirmed_callbacks: allow_to_skip_confirmed_callbacks.unwrap_or_default().into_inner(),
            use_hybrid_broadphase: use_hybrid_broadphase.unwrap_or_default().into_inner(),
            solver_tau: solver_tau.unwrap_or_default().into_inner(),
            solver_damp: solver_damp.unwrap_or_default().into_inner(),
            solver_iterations: solver_iterations.unwrap_or_default().into_inner(),
            solver_microsteps: solver_microsteps.unwrap_or_default().into_inner(),
            max_constraint_violation: max_constraint_violation.unwrap_or_default().into_inner(),
            force_coherent_constraint_ordering_in_solver: force_coherent_constraint_ordering_in_solver.unwrap_or_default().into_inner(),
            snap_collision_to_convex_edge_threshold: snap_collision_to_convex_edge_threshold.unwrap_or_default().into_inner(),
            snap_collision_to_concave_edge_threshold: snap_collision_to_concave_edge_threshold.unwrap_or_default().into_inner(),
            enable_toi_weld_rejection: enable_toi_weld_rejection.unwrap_or_default().into_inner(),
            enable_deprecated_welding: enable_deprecated_welding.unwrap_or_default().into_inner(),
            iterative_linear_cast_early_out_distance: iterative_linear_cast_early_out_distance.unwrap_or_default().into_inner(),
            iterative_linear_cast_max_iterations: iterative_linear_cast_max_iterations.unwrap_or_default().into_inner(),
            deactivation_num_inactive_frames_select_flag_0: deactivation_num_inactive_frames_select_flag_0.unwrap_or_default().into_inner(),
            deactivation_num_inactive_frames_select_flag_1: deactivation_num_inactive_frames_select_flag_1.unwrap_or_default().into_inner(),
            deactivation_integrate_counter: deactivation_integrate_counter.unwrap_or_default().into_inner(),
            should_activate_on_rigid_body_transform_change: should_activate_on_rigid_body_transform_change.unwrap_or_default().into_inner(),
            deactivation_reference_distance: deactivation_reference_distance.unwrap_or_default().into_inner(),
            toi_collision_response_rotate_normal: toi_collision_response_rotate_normal.unwrap_or_default().into_inner(),
            max_sectors_per_midphase_collide_task: max_sectors_per_midphase_collide_task.unwrap_or_default().into_inner(),
            max_sectors_per_narrowphase_collide_task: max_sectors_per_narrowphase_collide_task.unwrap_or_default().into_inner(),
            process_tois_multithreaded: process_tois_multithreaded.unwrap_or_default().into_inner(),
            max_entries_per_toi_midphase_collide_task: max_entries_per_toi_midphase_collide_task.unwrap_or_default().into_inner(),
            max_entries_per_toi_narrowphase_collide_task: max_entries_per_toi_narrowphase_collide_task.unwrap_or_default().into_inner(),
            max_num_toi_collision_pairs_singlethreaded: max_num_toi_collision_pairs_singlethreaded.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_simplified_toi: num_tois_till_allowed_penetration_simplified_toi.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi: num_tois_till_allowed_penetration_toi.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi_higher: num_tois_till_allowed_penetration_toi_higher.unwrap_or_default().into_inner(),
            num_tois_till_allowed_penetration_toi_forced: num_tois_till_allowed_penetration_toi_forced.unwrap_or_default().into_inner(),
            enable_deactivation: enable_deactivation.unwrap_or_default().into_inner(),
            simulation_type: simulation_type.unwrap_or_default().into_inner(),
            enable_simulation_islands: enable_simulation_islands.unwrap_or_default().into_inner(),
            min_desired_island_size: min_desired_island_size.unwrap_or_default().into_inner(),
            process_actions_in_single_thread: process_actions_in_single_thread.unwrap_or_default().into_inner(),
            allow_integration_of_islands_without_constraints_in_a_separate_job: allow_integration_of_islands_without_constraints_in_a_separate_job.unwrap_or_default().into_inner(),
            frame_marker_psi_snap: frame_marker_psi_snap.unwrap_or_default().into_inner(),
            fire_collision_callbacks: fire_collision_callbacks.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpWorldCinfo<'a>> for Vec<HkpWorldCinfoVisitor<'a>> {
    fn from(data: &HkpWorldCinfo<'a>) -> Self {
        vec![
            HkpWorldCinfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpWorldCinfoVisitor::ReferenceCount(data.reference_count.into()),
            HkpWorldCinfoVisitor::Gravity(data.gravity.into()),
            HkpWorldCinfoVisitor::BroadPhaseQuerySize(data.broad_phase_query_size.into()),
            HkpWorldCinfoVisitor::ContactRestingVelocity(data.contact_resting_velocity.into()),
            HkpWorldCinfoVisitor::BroadPhaseBorderBehaviour(data.broad_phase_border_behaviour.clone().into()),
            HkpWorldCinfoVisitor::MtPostponeAndSortBroadPhaseBorderCallbacks(data.mt_postpone_and_sort_broad_phase_border_callbacks.into()),
            HkpWorldCinfoVisitor::BroadPhaseWorldAabb(data.broad_phase_world_aabb.clone()),
            HkpWorldCinfoVisitor::UseKdTree(data.use_kd_tree.into()),
            HkpWorldCinfoVisitor::UseMultipleTree(data.use_multiple_tree.into()),
            HkpWorldCinfoVisitor::TreeUpdateType(data.tree_update_type.clone().into()),
            HkpWorldCinfoVisitor::AutoUpdateKdTree(data.auto_update_kd_tree.into()),
            HkpWorldCinfoVisitor::CollisionTolerance(data.collision_tolerance.into()),
            HkpWorldCinfoVisitor::CollisionFilter(data.collision_filter.clone().into()),
            HkpWorldCinfoVisitor::ConvexListFilter(data.convex_list_filter.clone().into()),
            HkpWorldCinfoVisitor::ExpectedMaxLinearVelocity(data.expected_max_linear_velocity.into()),
            HkpWorldCinfoVisitor::SizeOfToiEventQueue(data.size_of_toi_event_queue.into()),
            HkpWorldCinfoVisitor::ExpectedMinPsiDeltaTime(data.expected_min_psi_delta_time.into()),
            HkpWorldCinfoVisitor::MemoryWatchDog(data.memory_watch_dog.clone().into()),
            HkpWorldCinfoVisitor::BroadPhaseNumMarkers(data.broad_phase_num_markers.into()),
            HkpWorldCinfoVisitor::ContactPointGeneration(data.contact_point_generation.clone().into()),
            HkpWorldCinfoVisitor::AllowToSkipConfirmedCallbacks(data.allow_to_skip_confirmed_callbacks.into()),
            HkpWorldCinfoVisitor::UseHybridBroadphase(data.use_hybrid_broadphase.into()),
            HkpWorldCinfoVisitor::SolverTau(data.solver_tau.into()),
            HkpWorldCinfoVisitor::SolverDamp(data.solver_damp.into()),
            HkpWorldCinfoVisitor::SolverIterations(data.solver_iterations.into()),
            HkpWorldCinfoVisitor::SolverMicrosteps(data.solver_microsteps.into()),
            HkpWorldCinfoVisitor::MaxConstraintViolation(data.max_constraint_violation.into()),
            HkpWorldCinfoVisitor::ForceCoherentConstraintOrderingInSolver(data.force_coherent_constraint_ordering_in_solver.into()),
            HkpWorldCinfoVisitor::SnapCollisionToConvexEdgeThreshold(data.snap_collision_to_convex_edge_threshold.into()),
            HkpWorldCinfoVisitor::SnapCollisionToConcaveEdgeThreshold(data.snap_collision_to_concave_edge_threshold.into()),
            HkpWorldCinfoVisitor::EnableToiWeldRejection(data.enable_toi_weld_rejection.into()),
            HkpWorldCinfoVisitor::EnableDeprecatedWelding(data.enable_deprecated_welding.into()),
            HkpWorldCinfoVisitor::IterativeLinearCastEarlyOutDistance(data.iterative_linear_cast_early_out_distance.into()),
            HkpWorldCinfoVisitor::IterativeLinearCastMaxIterations(data.iterative_linear_cast_max_iterations.into()),
            HkpWorldCinfoVisitor::DeactivationNumInactiveFramesSelectFlag0(data.deactivation_num_inactive_frames_select_flag_0.into()),
            HkpWorldCinfoVisitor::DeactivationNumInactiveFramesSelectFlag1(data.deactivation_num_inactive_frames_select_flag_1.into()),
            HkpWorldCinfoVisitor::DeactivationIntegrateCounter(data.deactivation_integrate_counter.into()),
            HkpWorldCinfoVisitor::ShouldActivateOnRigidBodyTransformChange(data.should_activate_on_rigid_body_transform_change.into()),
            HkpWorldCinfoVisitor::DeactivationReferenceDistance(data.deactivation_reference_distance.into()),
            HkpWorldCinfoVisitor::ToiCollisionResponseRotateNormal(data.toi_collision_response_rotate_normal.into()),
            HkpWorldCinfoVisitor::MaxSectorsPerMidphaseCollideTask(data.max_sectors_per_midphase_collide_task.into()),
            HkpWorldCinfoVisitor::MaxSectorsPerNarrowphaseCollideTask(data.max_sectors_per_narrowphase_collide_task.into()),
            HkpWorldCinfoVisitor::ProcessToisMultithreaded(data.process_tois_multithreaded.into()),
            HkpWorldCinfoVisitor::MaxEntriesPerToiMidphaseCollideTask(data.max_entries_per_toi_midphase_collide_task.into()),
            HkpWorldCinfoVisitor::MaxEntriesPerToiNarrowphaseCollideTask(data.max_entries_per_toi_narrowphase_collide_task.into()),
            HkpWorldCinfoVisitor::MaxNumToiCollisionPairsSinglethreaded(data.max_num_toi_collision_pairs_singlethreaded.into()),
            HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationSimplifiedToi(data.num_tois_till_allowed_penetration_simplified_toi.into()),
            HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToi(data.num_tois_till_allowed_penetration_toi.into()),
            HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToiHigher(data.num_tois_till_allowed_penetration_toi_higher.into()),
            HkpWorldCinfoVisitor::NumToisTillAllowedPenetrationToiForced(data.num_tois_till_allowed_penetration_toi_forced.into()),
            HkpWorldCinfoVisitor::EnableDeactivation(data.enable_deactivation.into()),
            HkpWorldCinfoVisitor::SimulationType(data.simulation_type.clone().into()),
            HkpWorldCinfoVisitor::EnableSimulationIslands(data.enable_simulation_islands.into()),
            HkpWorldCinfoVisitor::MinDesiredIslandSize(data.min_desired_island_size.into()),
            HkpWorldCinfoVisitor::ProcessActionsInSingleThread(data.process_actions_in_single_thread.into()),
            HkpWorldCinfoVisitor::AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(data.allow_integration_of_islands_without_constraints_in_a_separate_job.into()),
            HkpWorldCinfoVisitor::FrameMarkerPsiSnap(data.frame_marker_psi_snap.into()),
            HkpWorldCinfoVisitor::FireCollisionCallbacks(data.fire_collision_callbacks.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpWorldCinfo<'de> {
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
enum HkpWorldCinfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "gravity")]
    Gravity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contactRestingVelocity")]
    ContactRestingVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "broadPhaseBorderBehaviour")]
    BroadPhaseBorderBehaviour(Primitive<BroadPhaseBorderBehaviour>),
    /// Visitor fields
    #[serde(rename = "mtPostponeAndSortBroadPhaseBorderCallbacks")]
    MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "broadPhaseWorldAabb")]
    BroadPhaseWorldAabb(SingleClass<HkAabb>),
    /// Visitor fields
    #[serde(rename = "useKdTree")]
    UseKdTree(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useMultipleTree")]
    UseMultipleTree(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "treeUpdateType")]
    TreeUpdateType(Primitive<TreeUpdateType>),
    /// Visitor fields
    #[serde(rename = "autoUpdateKdTree")]
    AutoUpdateKdTree(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "collisionTolerance")]
    CollisionTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilter")]
    CollisionFilter(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "convexListFilter")]
    ConvexListFilter(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "expectedMaxLinearVelocity")]
    ExpectedMaxLinearVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "expectedMinPsiDeltaTime")]
    ExpectedMinPsiDeltaTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "memoryWatchDog")]
    MemoryWatchDog(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "contactPointGeneration")]
    ContactPointGeneration(Primitive<ContactPointGeneration>),
    /// Visitor fields
    #[serde(rename = "allowToSkipConfirmedCallbacks")]
    AllowToSkipConfirmedCallbacks(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useHybridBroadphase")]
    UseHybridBroadphase(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "solverTau")]
    SolverTau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "solverDamp")]
    SolverDamp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "solverIterations")]
    SolverIterations(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "solverMicrosteps")]
    SolverMicrosteps(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxConstraintViolation")]
    MaxConstraintViolation(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "forceCoherentConstraintOrderingInSolver")]
    ForceCoherentConstraintOrderingInSolver(Primitive<bool>),
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
    #[serde(rename = "enableDeprecatedWelding")]
    EnableDeprecatedWelding(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "iterativeLinearCastEarlyOutDistance")]
    IterativeLinearCastEarlyOutDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "iterativeLinearCastMaxIterations")]
    IterativeLinearCastMaxIterations(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag0")]
    DeactivationNumInactiveFramesSelectFlag0(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag1")]
    DeactivationNumInactiveFramesSelectFlag1(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
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
    #[serde(rename = "enableDeactivation")]
    EnableDeactivation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "simulationType")]
    SimulationType(Primitive<SimulationType>),
    /// Visitor fields
    #[serde(rename = "enableSimulationIslands")]
    EnableSimulationIslands(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fireCollisionCallbacks")]
    FireCollisionCallbacks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldCinfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("gravity" => Gravity(Primitive<Vector4<f32>>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("contactRestingVelocity" => ContactRestingVelocity(Primitive<f32>)),
    ("broadPhaseBorderBehaviour" => BroadPhaseBorderBehaviour(Primitive<BroadPhaseBorderBehaviour>)),
    ("mtPostponeAndSortBroadPhaseBorderCallbacks" => MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>)),
    ("broadPhaseWorldAabb" => BroadPhaseWorldAabb(SingleClass<HkAabb>)),
    ("useKdTree" => UseKdTree(Primitive<bool>)),
    ("useMultipleTree" => UseMultipleTree(Primitive<bool>)),
    ("treeUpdateType" => TreeUpdateType(Primitive<TreeUpdateType>)),
    ("autoUpdateKdTree" => AutoUpdateKdTree(Primitive<bool>)),
    ("collisionTolerance" => CollisionTolerance(Primitive<f32>)),
    ("collisionFilter" => CollisionFilter(Primitive<Cow<'de, str>>)),
    ("convexListFilter" => ConvexListFilter(Primitive<Cow<'de, str>>)),
    ("expectedMaxLinearVelocity" => ExpectedMaxLinearVelocity(Primitive<f32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("expectedMinPsiDeltaTime" => ExpectedMinPsiDeltaTime(Primitive<f32>)),
    ("memoryWatchDog" => MemoryWatchDog(Primitive<Cow<'de, str>>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(Primitive<ContactPointGeneration>)),
    ("allowToSkipConfirmedCallbacks" => AllowToSkipConfirmedCallbacks(Primitive<bool>)),
    ("useHybridBroadphase" => UseHybridBroadphase(Primitive<bool>)),
    ("solverTau" => SolverTau(Primitive<f32>)),
    ("solverDamp" => SolverDamp(Primitive<f32>)),
    ("solverIterations" => SolverIterations(Primitive<i32>)),
    ("solverMicrosteps" => SolverMicrosteps(Primitive<i32>)),
    ("maxConstraintViolation" => MaxConstraintViolation(Primitive<f32>)),
    ("forceCoherentConstraintOrderingInSolver" => ForceCoherentConstraintOrderingInSolver(Primitive<bool>)),
    ("snapCollisionToConvexEdgeThreshold" => SnapCollisionToConvexEdgeThreshold(Primitive<f32>)),
    ("snapCollisionToConcaveEdgeThreshold" => SnapCollisionToConcaveEdgeThreshold(Primitive<f32>)),
    ("enableToiWeldRejection" => EnableToiWeldRejection(Primitive<bool>)),
    ("enableDeprecatedWelding" => EnableDeprecatedWelding(Primitive<bool>)),
    ("iterativeLinearCastEarlyOutDistance" => IterativeLinearCastEarlyOutDistance(Primitive<f32>)),
    ("iterativeLinearCastMaxIterations" => IterativeLinearCastMaxIterations(Primitive<i32>)),
    ("deactivationNumInactiveFramesSelectFlag0" => DeactivationNumInactiveFramesSelectFlag0(Primitive<u8>)),
    ("deactivationNumInactiveFramesSelectFlag1" => DeactivationNumInactiveFramesSelectFlag1(Primitive<u8>)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("shouldActivateOnRigidBodyTransformChange" => ShouldActivateOnRigidBodyTransformChange(Primitive<bool>)),
    ("deactivationReferenceDistance" => DeactivationReferenceDistance(Primitive<f32>)),
    ("toiCollisionResponseRotateNormal" => ToiCollisionResponseRotateNormal(Primitive<f32>)),
    ("maxSectorsPerMidphaseCollideTask" => MaxSectorsPerMidphaseCollideTask(Primitive<i32>)),
    ("maxSectorsPerNarrowphaseCollideTask" => MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>)),
    ("processToisMultithreaded" => ProcessToisMultithreaded(Primitive<bool>)),
    ("maxEntriesPerToiMidphaseCollideTask" => MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>)),
    ("maxEntriesPerToiNarrowphaseCollideTask" => MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>)),
    ("maxNumToiCollisionPairsSinglethreaded" => MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(Primitive<f32>)),
    ("enableDeactivation" => EnableDeactivation(Primitive<bool>)),
    ("simulationType" => SimulationType(Primitive<SimulationType>)),
    ("enableSimulationIslands" => EnableSimulationIslands(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("fireCollisionCallbacks" => FireCollisionCallbacks(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SolverType {
    #[serde(rename = "SOLVER_TYPE_INVALID")]
    #[default]
    SolverTypeInvalid = 0,
    #[serde(rename = "SOLVER_TYPE_2ITERS_SOFT")]
    SolverType2ItersSoft = 1,
    #[serde(rename = "SOLVER_TYPE_2ITERS_MEDIUM")]
    SolverType2ItersMedium = 2,
    #[serde(rename = "SOLVER_TYPE_2ITERS_HARD")]
    SolverType2ItersHard = 3,
    #[serde(rename = "SOLVER_TYPE_4ITERS_SOFT")]
    SolverType4ItersSoft = 4,
    #[serde(rename = "SOLVER_TYPE_4ITERS_MEDIUM")]
    SolverType4ItersMedium = 5,
    #[serde(rename = "SOLVER_TYPE_4ITERS_HARD")]
    SolverType4ItersHard = 6,
    #[serde(rename = "SOLVER_TYPE_8ITERS_SOFT")]
    SolverType8ItersSoft = 7,
    #[serde(rename = "SOLVER_TYPE_8ITERS_MEDIUM")]
    SolverType8ItersMedium = 8,
    #[serde(rename = "SOLVER_TYPE_8ITERS_HARD")]
    SolverType8ItersHard = 9,
    #[serde(rename = "SOLVER_TYPE_MAX_ID")]
    SolverTypeMaxId = 10,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SimulationType {
    #[serde(rename = "SIMULATION_TYPE_INVALID")]
    #[default]
    SimulationTypeInvalid = 0,
    #[serde(rename = "SIMULATION_TYPE_DISCRETE")]
    SimulationTypeDiscrete = 1,
    #[serde(rename = "SIMULATION_TYPE_CONTINUOUS")]
    SimulationTypeContinuous = 2,
    #[serde(rename = "SIMULATION_TYPE_MULTITHREADED")]
    SimulationTypeMultithreaded = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ContactPointGeneration {
    #[serde(rename = "CONTACT_POINT_ACCEPT_ALWAYS")]
    #[default]
    ContactPointAcceptAlways = 0,
    #[serde(rename = "CONTACT_POINT_REJECT_DUBIOUS")]
    ContactPointRejectDubious = 1,
    #[serde(rename = "CONTACT_POINT_REJECT_MANY")]
    ContactPointRejectMany = 2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BroadPhaseBorderBehaviour {
    #[serde(rename = "BROADPHASE_BORDER_ASSERT")]
    #[default]
    BroadphaseBorderAssert = 0,
    #[serde(rename = "BROADPHASE_BORDER_FIX_ENTITY")]
    BroadphaseBorderFixEntity = 1,
    #[serde(rename = "BROADPHASE_BORDER_REMOVE_ENTITY")]
    BroadphaseBorderRemoveEntity = 2,
    #[serde(rename = "BROADPHASE_BORDER_DO_NOTHING")]
    BroadphaseBorderDoNothing = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum TreeUpdateType {
    #[serde(rename = "REBUILD_ACTIVE")]
    #[default]
    RebuildActive = 0,
    #[serde(rename = "REBUILD_ALL")]
    RebuildAll = 1,
}
