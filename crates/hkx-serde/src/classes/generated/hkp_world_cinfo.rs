//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWorldCinfo`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
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

    /// `"hkpWorldCinfo"`: The original C++ class name.
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
    /// Return `"hkpWorldCinfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpWorldCinfo".into()
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
    /// # Field information in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contactRestingVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactRestingVelocity")]
    ContactRestingVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseBorderBehaviour"`
    /// -   type: `enum BroadPhaseBorderBehaviour`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseBorderBehaviour")]
    BroadPhaseBorderBehaviour(BroadPhaseBorderBehaviour),
    /// # Field information in the original C++ class
    /// -   name:`"mtPostponeAndSortBroadPhaseBorderCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mtPostponeAndSortBroadPhaseBorderCallbacks")]
    MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseWorldAabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseWorldAabb")]
    BroadPhaseWorldAabb(HkAabb),
    /// # Field information in the original C++ class
    /// -   name:`"useKdTree"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useKdTree")]
    UseKdTree(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"useMultipleTree"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useMultipleTree")]
    UseMultipleTree(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"treeUpdateType"`
    /// -   type: `enum TreeUpdateType`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "treeUpdateType")]
    TreeUpdateType(TreeUpdateType),
    /// # Field information in the original C++ class
    /// -   name:`"autoUpdateKdTree"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoUpdateKdTree")]
    AutoUpdateKdTree(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionTolerance")]
    CollisionTolerance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilter")]
    CollisionFilter(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"convexListFilter"`
    /// -   type: `struct hkpConvexListFilter*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexListFilter")]
    ConvexListFilter(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"expectedMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMaxLinearVelocity")]
    ExpectedMaxLinearVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"expectedMinPsiDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMinPsiDeltaTime")]
    ExpectedMinPsiDeltaTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"memoryWatchDog"`
    /// -   type: `struct hkWorldMemoryAvailableWatchDog*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memoryWatchDog")]
    MemoryWatchDog(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum ContactPointGeneration`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointGeneration")]
    ContactPointGeneration(ContactPointGeneration),
    /// # Field information in the original C++ class
    /// -   name:`"allowToSkipConfirmedCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 117
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowToSkipConfirmedCallbacks")]
    AllowToSkipConfirmedCallbacks(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 118
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useHybridBroadphase")]
    UseHybridBroadphase(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"solverTau"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverTau")]
    SolverTau(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"solverDamp"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverDamp")]
    SolverDamp(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"solverIterations"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverIterations")]
    SolverIterations(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"solverMicrosteps"`
    /// -   type: `hkInt32`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverMicrosteps")]
    SolverMicrosteps(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxConstraintViolation"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxConstraintViolation")]
    MaxConstraintViolation(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"forceCoherentConstraintOrderingInSolver"`
    /// -   type: `hkBool`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCoherentConstraintOrderingInSolver")]
    ForceCoherentConstraintOrderingInSolver(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"enableDeprecatedWelding"`
    /// -   type: `hkBool`
    /// - offset: 153
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeprecatedWelding")]
    EnableDeprecatedWelding(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"iterativeLinearCastEarlyOutDistance"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastEarlyOutDistance")]
    IterativeLinearCastEarlyOutDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"iterativeLinearCastMaxIterations"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastMaxIterations")]
    IterativeLinearCastMaxIterations(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationNumInactiveFramesSelectFlag0"`
    /// -   type: `hkUint8`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag0")]
    DeactivationNumInactiveFramesSelectFlag0(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationNumInactiveFramesSelectFlag1"`
    /// -   type: `hkUint8`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag1")]
    DeactivationNumInactiveFramesSelectFlag1(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 167
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enableDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeactivation")]
    EnableDeactivation(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"simulationType"`
    /// -   type: `enum SimulationType`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulationType")]
    SimulationType(SimulationType),
    /// # Field information in the original C++ class
    /// -   name:`"enableSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableSimulationIslands")]
    EnableSimulationIslands(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 225
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fireCollisionCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fireCollisionCallbacks")]
    FireCollisionCallbacks(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldCinfoHkParam<'de>, "@name",
    ("gravity" => Gravity(Vector4<f32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("contactRestingVelocity" => ContactRestingVelocity(Primitive<f32>)),
    ("broadPhaseBorderBehaviour" => BroadPhaseBorderBehaviour(BroadPhaseBorderBehaviour)),
    ("mtPostponeAndSortBroadPhaseBorderCallbacks" => MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>)),
    ("broadPhaseWorldAabb" => BroadPhaseWorldAabb(HkAabb)),
    ("useKdTree" => UseKdTree(Primitive<bool>)),
    ("useMultipleTree" => UseMultipleTree(Primitive<bool>)),
    ("treeUpdateType" => TreeUpdateType(TreeUpdateType)),
    ("autoUpdateKdTree" => AutoUpdateKdTree(Primitive<bool>)),
    ("collisionTolerance" => CollisionTolerance(Primitive<f32>)),
    ("collisionFilter" => CollisionFilter(Cow<'a, str>)),
    ("convexListFilter" => ConvexListFilter(Cow<'a, str>)),
    ("expectedMaxLinearVelocity" => ExpectedMaxLinearVelocity(Primitive<f32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("expectedMinPsiDeltaTime" => ExpectedMinPsiDeltaTime(Primitive<f32>)),
    ("memoryWatchDog" => MemoryWatchDog(Cow<'a, str>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(ContactPointGeneration)),
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
    ("simulationType" => SimulationType(SimulationType)),
    ("enableSimulationIslands" => EnableSimulationIslands(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("fireCollisionCallbacks" => FireCollisionCallbacks(Primitive<bool>)),
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
