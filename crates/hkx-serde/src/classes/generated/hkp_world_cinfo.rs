//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWorldCinfo`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 240
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 11
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWorldCinfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWorldCinfo"`: Name of this class.
    #[serde(default = "HkpWorldCinfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa5255445`: Unique value of this class.
    #[serde(default = "HkpWorldCinfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWorldCinfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWorldCinfoHkParam<'a>>
}

impl HkpWorldCinfo<'_> {
    /// Return `"hkpWorldCinfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpWorldCinfo".into()
    }

    /// Return `"0xa5255445"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa5255445".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorldCinfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactRestingVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactRestingVelocity")]
    ContactRestingVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"broadPhaseBorderBehaviour"`
    /// -   type: `enum BroadPhaseBorderBehaviour`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseBorderBehaviour")]
    BroadPhaseBorderBehaviour(BroadPhaseBorderBehaviour),
    /// # Information on fields in the original C++ class
    /// -   name:`"mtPostponeAndSortBroadPhaseBorderCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mtPostponeAndSortBroadPhaseBorderCallbacks")]
    MtPostponeAndSortBroadPhaseBorderCallbacks(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"broadPhaseWorldAabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseWorldAabb")]
    BroadPhaseWorldAabb(HkAabb),
    /// # Information on fields in the original C++ class
    /// -   name:`"useKdTree"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useKdTree")]
    UseKdTree(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"useMultipleTree"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useMultipleTree")]
    UseMultipleTree(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"treeUpdateType"`
    /// -   type: `enum TreeUpdateType`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "treeUpdateType")]
    TreeUpdateType(TreeUpdateType),
    /// # Information on fields in the original C++ class
    /// -   name:`"autoUpdateKdTree"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoUpdateKdTree")]
    AutoUpdateKdTree(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionTolerance")]
    CollisionTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilter")]
    CollisionFilter(Box<HkpCollisionFilter>),
    /// # Information on fields in the original C++ class
    /// -   name:`"convexListFilter"`
    /// -   type: `struct hkpConvexListFilter*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexListFilter")]
    ConvexListFilter(Box<HkpConvexListFilter>),
    /// # Information on fields in the original C++ class
    /// -   name:`"expectedMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMaxLinearVelocity")]
    ExpectedMaxLinearVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"expectedMinPsiDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMinPsiDeltaTime")]
    ExpectedMinPsiDeltaTime(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"memoryWatchDog"`
    /// -   type: `struct hkWorldMemoryAvailableWatchDog*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memoryWatchDog")]
    MemoryWatchDog(Box<HkWorldMemoryAvailableWatchDog>),
    /// # Information on fields in the original C++ class
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum ContactPointGeneration`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointGeneration")]
    ContactPointGeneration(ContactPointGeneration),
    /// # Information on fields in the original C++ class
    /// -   name:`"allowToSkipConfirmedCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 117
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowToSkipConfirmedCallbacks")]
    AllowToSkipConfirmedCallbacks(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 118
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useHybridBroadphase")]
    UseHybridBroadphase(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverTau"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverTau")]
    SolverTau(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverDamp"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverDamp")]
    SolverDamp(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverIterations"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverIterations")]
    SolverIterations(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverMicrosteps"`
    /// -   type: `hkInt32`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverMicrosteps")]
    SolverMicrosteps(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxConstraintViolation"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxConstraintViolation")]
    MaxConstraintViolation(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"forceCoherentConstraintOrderingInSolver"`
    /// -   type: `hkBool`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCoherentConstraintOrderingInSolver")]
    ForceCoherentConstraintOrderingInSolver(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableDeprecatedWelding"`
    /// -   type: `hkBool`
    /// - offset: 153
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeprecatedWelding")]
    EnableDeprecatedWelding(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"iterativeLinearCastEarlyOutDistance"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastEarlyOutDistance")]
    IterativeLinearCastEarlyOutDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"iterativeLinearCastMaxIterations"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastMaxIterations")]
    IterativeLinearCastMaxIterations(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"deactivationNumInactiveFramesSelectFlag0"`
    /// -   type: `hkUint8`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag0")]
    DeactivationNumInactiveFramesSelectFlag0(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"deactivationNumInactiveFramesSelectFlag1"`
    /// -   type: `hkUint8`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag1")]
    DeactivationNumInactiveFramesSelectFlag1(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 167
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeactivation")]
    EnableDeactivation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"simulationType"`
    /// -   type: `enum SimulationType`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulationType")]
    SimulationType(SimulationType),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableSimulationIslands")]
    EnableSimulationIslands(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 225
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"fireCollisionCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fireCollisionCallbacks")]
    FireCollisionCallbacks(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldCinfoHkParam<'de>, "@name",
    ("gravity" => Gravity(cgmath::Vector4<f32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(i32)),
    ("contactRestingVelocity" => ContactRestingVelocity(f64)),
    ("broadPhaseBorderBehaviour" => BroadPhaseBorderBehaviour(BroadPhaseBorderBehaviour)),
    ("mtPostponeAndSortBroadPhaseBorderCallbacks" => MtPostponeAndSortBroadPhaseBorderCallbacks(bool)),
    ("broadPhaseWorldAabb" => BroadPhaseWorldAabb(HkAabb)),
    ("useKdTree" => UseKdTree(bool)),
    ("useMultipleTree" => UseMultipleTree(bool)),
    ("treeUpdateType" => TreeUpdateType(TreeUpdateType)),
    ("autoUpdateKdTree" => AutoUpdateKdTree(bool)),
    ("collisionTolerance" => CollisionTolerance(f64)),
    ("collisionFilter" => CollisionFilter(Box<HkpCollisionFilter>)),
    ("convexListFilter" => ConvexListFilter(Box<HkpConvexListFilter>)),
    ("expectedMaxLinearVelocity" => ExpectedMaxLinearVelocity(f64)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(i32)),
    ("expectedMinPsiDeltaTime" => ExpectedMinPsiDeltaTime(f64)),
    ("memoryWatchDog" => MemoryWatchDog(Box<HkWorldMemoryAvailableWatchDog>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(i32)),
    ("contactPointGeneration" => ContactPointGeneration(ContactPointGeneration)),
    ("allowToSkipConfirmedCallbacks" => AllowToSkipConfirmedCallbacks(bool)),
    ("useHybridBroadphase" => UseHybridBroadphase(bool)),
    ("solverTau" => SolverTau(f64)),
    ("solverDamp" => SolverDamp(f64)),
    ("solverIterations" => SolverIterations(i32)),
    ("solverMicrosteps" => SolverMicrosteps(i32)),
    ("maxConstraintViolation" => MaxConstraintViolation(f64)),
    ("forceCoherentConstraintOrderingInSolver" => ForceCoherentConstraintOrderingInSolver(bool)),
    ("snapCollisionToConvexEdgeThreshold" => SnapCollisionToConvexEdgeThreshold(f64)),
    ("snapCollisionToConcaveEdgeThreshold" => SnapCollisionToConcaveEdgeThreshold(f64)),
    ("enableToiWeldRejection" => EnableToiWeldRejection(bool)),
    ("enableDeprecatedWelding" => EnableDeprecatedWelding(bool)),
    ("iterativeLinearCastEarlyOutDistance" => IterativeLinearCastEarlyOutDistance(f64)),
    ("iterativeLinearCastMaxIterations" => IterativeLinearCastMaxIterations(i32)),
    ("deactivationNumInactiveFramesSelectFlag0" => DeactivationNumInactiveFramesSelectFlag0(u8)),
    ("deactivationNumInactiveFramesSelectFlag1" => DeactivationNumInactiveFramesSelectFlag1(u8)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(u8)),
    ("shouldActivateOnRigidBodyTransformChange" => ShouldActivateOnRigidBodyTransformChange(bool)),
    ("deactivationReferenceDistance" => DeactivationReferenceDistance(f64)),
    ("toiCollisionResponseRotateNormal" => ToiCollisionResponseRotateNormal(f64)),
    ("maxSectorsPerMidphaseCollideTask" => MaxSectorsPerMidphaseCollideTask(i32)),
    ("maxSectorsPerNarrowphaseCollideTask" => MaxSectorsPerNarrowphaseCollideTask(i32)),
    ("processToisMultithreaded" => ProcessToisMultithreaded(bool)),
    ("maxEntriesPerToiMidphaseCollideTask" => MaxEntriesPerToiMidphaseCollideTask(i32)),
    ("maxEntriesPerToiNarrowphaseCollideTask" => MaxEntriesPerToiNarrowphaseCollideTask(i32)),
    ("maxNumToiCollisionPairsSinglethreaded" => MaxNumToiCollisionPairsSinglethreaded(i32)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(f64)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(f64)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(f64)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(f64)),
    ("enableDeactivation" => EnableDeactivation(bool)),
    ("simulationType" => SimulationType(SimulationType)),
    ("enableSimulationIslands" => EnableSimulationIslands(bool)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(u32)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(bool)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(bool)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(f64)),
    ("fireCollisionCallbacks" => FireCollisionCallbacks(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SolverType {
    #[serde(rename = "SOLVER_TYPE_INVALID")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SimulationType {
    #[serde(rename = "SIMULATION_TYPE_INVALID")]
    SimulationTypeInvalid = 0,
    #[serde(rename = "SIMULATION_TYPE_DISCRETE")]
    SimulationTypeDiscrete = 1,
    #[serde(rename = "SIMULATION_TYPE_CONTINUOUS")]
    SimulationTypeContinuous = 2,
    #[serde(rename = "SIMULATION_TYPE_MULTITHREADED")]
    SimulationTypeMultithreaded = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContactPointGeneration {
    #[serde(rename = "CONTACT_POINT_ACCEPT_ALWAYS")]
    ContactPointAcceptAlways = 0,
    #[serde(rename = "CONTACT_POINT_REJECT_DUBIOUS")]
    ContactPointRejectDubious = 1,
    #[serde(rename = "CONTACT_POINT_REJECT_MANY")]
    ContactPointRejectMany = 2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BroadPhaseBorderBehaviour {
    #[serde(rename = "BROADPHASE_BORDER_ASSERT")]
    BroadphaseBorderAssert = 0,
    #[serde(rename = "BROADPHASE_BORDER_FIX_ENTITY")]
    BroadphaseBorderFixEntity = 1,
    #[serde(rename = "BROADPHASE_BORDER_REMOVE_ENTITY")]
    BroadphaseBorderRemoveEntity = 2,
    #[serde(rename = "BROADPHASE_BORDER_DO_NOTHING")]
    BroadphaseBorderDoNothing = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TreeUpdateType {
    #[serde(rename = "REBUILD_ACTIVE")]
    RebuildActive = 0,
    #[serde(rename = "REBUILD_ALL")]
    RebuildAll = 1,
}
