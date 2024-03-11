//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWorld`, a class defined in C++
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
/// -    size: 864
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWorld<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWorld"`: The original C++ class name.
    #[serde(default = "HkpWorld::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xaadcec37`: Unique value of this class.
    #[serde(default = "HkpWorld::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWorldHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWorldHkParam<'a>>
}

impl HkpWorld<'_> {
    /// Return `"hkpWorld"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpWorld".into()
    }

    /// Return `"0xaadcec37"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xaadcec37".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorldHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"simulation"`
    /// -   type: `struct hkpSimulation*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulation")]
    Simulation(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fixedIsland"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fixedIsland", skip_serializing)]
    FixedIsland(()),
    /// # Field information in the original C++ class
    /// -   name:`"fixedRigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedRigidBody")]
    FixedRigidBody(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"activeSimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeSimulationIslands", skip_serializing)]
    ActiveSimulationIslands(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"inactiveSimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "inactiveSimulationIslands", skip_serializing)]
    InactiveSimulationIslands(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"dirtySimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "dirtySimulationIslands", skip_serializing)]
    DirtySimulationIslands(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"maintenanceMgr"`
    /// -   type: `void*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "maintenanceMgr", skip_serializing)]
    MaintenanceMgr(()),
    /// # Field information in the original C++ class
    /// -   name:`"memoryWatchDog"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memoryWatchDog", skip_serializing)]
    MemoryWatchDog(()),
    /// # Field information in the original C++ class
    /// -   name:`"assertOnRunningOutOfSolverMemory"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "assertOnRunningOutOfSolverMemory", skip_serializing)]
    AssertOnRunningOutOfSolverMemory(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhase"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhase", skip_serializing)]
    BroadPhase(()),
    /// # Field information in the original C++ class
    /// -   name:`"kdTreeManager"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "kdTreeManager", skip_serializing)]
    KdTreeManager(()),
    /// # Field information in the original C++ class
    /// -   name:`"autoUpdateTree"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoUpdateTree")]
    AutoUpdateTree(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseDispatcher"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseDispatcher", skip_serializing)]
    BroadPhaseDispatcher(()),
    /// # Field information in the original C++ class
    /// -   name:`"phantomBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomBroadPhaseListener", skip_serializing)]
    PhantomBroadPhaseListener(()),
    /// # Field information in the original C++ class
    /// -   name:`"entityEntityBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityEntityBroadPhaseListener", skip_serializing)]
    EntityEntityBroadPhaseListener(()),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseBorderListener"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseBorderListener", skip_serializing)]
    BroadPhaseBorderListener(()),
    /// # Field information in the original C++ class
    /// -   name:`"multithreadedSimulationJobData"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "multithreadedSimulationJobData", skip_serializing)]
    MultithreadedSimulationJobData(()),
    /// # Field information in the original C++ class
    /// -   name:`"collisionInput"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionInput", skip_serializing)]
    CollisionInput(()),
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilter"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionFilter", skip_serializing)]
    CollisionFilter(()),
    /// # Field information in the original C++ class
    /// -   name:`"collisionDispatcher"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDispatcher", skip_serializing)]
    CollisionDispatcher(()),
    /// # Field information in the original C++ class
    /// -   name:`"convexListFilter"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "convexListFilter", skip_serializing)]
    ConvexListFilter(()),
    /// # Field information in the original C++ class
    /// -   name:`"pendingOperations"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingOperations", skip_serializing)]
    PendingOperations(()),
    /// # Field information in the original C++ class
    /// -   name:`"pendingOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pendingOperationsCount")]
    PendingOperationsCount(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"pendingBodyOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingBodyOperationsCount", skip_serializing)]
    PendingBodyOperationsCount(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"criticalOperationsLockCount"`
    /// -   type: `hkInt32`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsLockCount")]
    CriticalOperationsLockCount(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"criticalOperationsLockCountForPhantoms"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsLockCountForPhantoms")]
    CriticalOperationsLockCountForPhantoms(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"blockExecutingPendingOperations"`
    /// -   type: `hkBool`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockExecutingPendingOperations")]
    BlockExecutingPendingOperations(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"criticalOperationsAllowed"`
    /// -   type: `hkBool`
    /// - offset: 157
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsAllowed")]
    CriticalOperationsAllowed(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"pendingOperationQueues"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingOperationQueues", skip_serializing)]
    PendingOperationQueues(()),
    /// # Field information in the original C++ class
    /// -   name:`"pendingOperationQueueCount"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pendingOperationQueueCount")]
    PendingOperationQueueCount(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "multiThreadCheck", skip_serializing)]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # Field information in the original C++ class
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"modifyConstraintCriticalSection"`
    /// -   type: `void*`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "modifyConstraintCriticalSection", skip_serializing)]
    ModifyConstraintCriticalSection(()),
    /// # Field information in the original C++ class
    /// -   name:`"isLocked"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"islandDirtyListCriticalSection"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandDirtyListCriticalSection", skip_serializing)]
    IslandDirtyListCriticalSection(()),
    /// # Field information in the original C++ class
    /// -   name:`"propertyMasterLock"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "propertyMasterLock", skip_serializing)]
    PropertyMasterLock(()),
    /// # Field information in the original C++ class
    /// -   name:`"wantSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantSimulationIslands")]
    WantSimulationIslands(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "useHybridBroadphase", skip_serializing)]
    UseHybridBroadphase(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"wantDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantDeactivation")]
    WantDeactivation(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"simulationType"`
    /// -   type: `enum unknown`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "simulationType", skip_serializing)]
    SimulationType(Unknown),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 260
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 264
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"lastEntityUid"`
    /// -   type: `hkUint32`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastEntityUid")]
    LastEntityUid(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"lastIslandUid"`
    /// -   type: `hkUint32`
    /// - offset: 276
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastIslandUid")]
    LastIslandUid(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"lastConstraintUid"`
    /// -   type: `hkUint32`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastConstraintUid")]
    LastConstraintUid(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"phantoms"`
    /// -   type: `hkArray&lt;hkpPhantom*&gt;`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantoms")]
    Phantoms(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"actionListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "actionListeners", skip_serializing)]
    ActionListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"entityListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomListeners", skip_serializing)]
    PhantomListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"constraintListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 332
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintListeners", skip_serializing)]
    ConstraintListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"worldDeletionListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 344
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldDeletionListeners", skip_serializing)]
    WorldDeletionListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"islandActivationListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 356
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandActivationListeners", skip_serializing)]
    IslandActivationListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"worldPostSimulationListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 368
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostSimulationListeners", skip_serializing)]
    WorldPostSimulationListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"worldPostIntegrateListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 380
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostIntegrateListeners", skip_serializing)]
    WorldPostIntegrateListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"worldPostCollideListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 392
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostCollideListeners", skip_serializing)]
    WorldPostCollideListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"islandPostIntegrateListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 404
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandPostIntegrateListeners", skip_serializing)]
    IslandPostIntegrateListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"islandPostCollideListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandPostCollideListeners", skip_serializing)]
    IslandPostCollideListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"contactListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 428
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"contactImpulseLimitBreachedListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 440
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactImpulseLimitBreachedListeners", skip_serializing)]
    ContactImpulseLimitBreachedListeners(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"worldExtensions"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 452
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldExtensions", skip_serializing)]
    WorldExtensions(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"violatedConstraintArray"`
    /// -   type: `void*`
    /// - offset: 464
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "violatedConstraintArray", skip_serializing)]
    ViolatedConstraintArray(()),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseBorder"`
    /// -   type: `void*`
    /// - offset: 468
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseBorder", skip_serializing)]
    BroadPhaseBorder(()),
    /// # Field information in the original C++ class
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 472
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(()),
    /// # Field information in the original C++ class
    /// -   name:`"npWorld"`
    /// -   type: `void*`
    /// - offset: 476
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "npWorld", skip_serializing)]
    NpWorld(()),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseExtents"`
    /// -   type: `hkVector4[2]`
    /// - offset: 800
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseExtents")]
    BroadPhaseExtents(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 832
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 836
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 840
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"broadPhaseUpdateSize"`
    /// -   type: `hkInt32`
    /// - offset: 844
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseUpdateSize")]
    BroadPhaseUpdateSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum unknown`
    /// - offset: 848
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactPointGeneration", skip_serializing)]
    ContactPointGeneration(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldHkParam<'de>, "@name",
    ("simulation" => Simulation(Cow<'a, str>)),
    ("gravity" => Gravity(Vector4<f32>)),
    ("fixedIsland" => FixedIsland(())),
    ("fixedRigidBody" => FixedRigidBody(Cow<'a, str>)),
    ("activeSimulationIslands" => ActiveSimulationIslands(Vec<()>)),
    ("inactiveSimulationIslands" => InactiveSimulationIslands(Vec<()>)),
    ("dirtySimulationIslands" => DirtySimulationIslands(Vec<()>)),
    ("maintenanceMgr" => MaintenanceMgr(())),
    ("memoryWatchDog" => MemoryWatchDog(())),
    ("assertOnRunningOutOfSolverMemory" => AssertOnRunningOutOfSolverMemory(Primitive<bool>)),
    ("broadPhase" => BroadPhase(())),
    ("kdTreeManager" => KdTreeManager(())),
    ("autoUpdateTree" => AutoUpdateTree(Primitive<bool>)),
    ("broadPhaseDispatcher" => BroadPhaseDispatcher(())),
    ("phantomBroadPhaseListener" => PhantomBroadPhaseListener(())),
    ("entityEntityBroadPhaseListener" => EntityEntityBroadPhaseListener(())),
    ("broadPhaseBorderListener" => BroadPhaseBorderListener(())),
    ("multithreadedSimulationJobData" => MultithreadedSimulationJobData(())),
    ("collisionInput" => CollisionInput(())),
    ("collisionFilter" => CollisionFilter(())),
    ("collisionDispatcher" => CollisionDispatcher(())),
    ("convexListFilter" => ConvexListFilter(())),
    ("pendingOperations" => PendingOperations(())),
    ("pendingOperationsCount" => PendingOperationsCount(Primitive<i32>)),
    ("pendingBodyOperationsCount" => PendingBodyOperationsCount(Primitive<i32>)),
    ("criticalOperationsLockCount" => CriticalOperationsLockCount(Primitive<i32>)),
    ("criticalOperationsLockCountForPhantoms" => CriticalOperationsLockCountForPhantoms(Primitive<i32>)),
    ("blockExecutingPendingOperations" => BlockExecutingPendingOperations(Primitive<bool>)),
    ("criticalOperationsAllowed" => CriticalOperationsAllowed(Primitive<bool>)),
    ("pendingOperationQueues" => PendingOperationQueues(())),
    ("pendingOperationQueueCount" => PendingOperationQueueCount(Primitive<i32>)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("modifyConstraintCriticalSection" => ModifyConstraintCriticalSection(())),
    ("isLocked" => IsLocked(Primitive<i32>)),
    ("islandDirtyListCriticalSection" => IslandDirtyListCriticalSection(())),
    ("propertyMasterLock" => PropertyMasterLock(())),
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
    ("simulationType" => SimulationType(Unknown)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(Primitive<f32>)),
    ("lastEntityUid" => LastEntityUid(Primitive<u32>)),
    ("lastIslandUid" => LastIslandUid(Primitive<u32>)),
    ("lastConstraintUid" => LastConstraintUid(Primitive<u32>)),
    ("phantoms" => Phantoms(Vec<Cow<'a, str>>)),
    ("actionListeners" => ActionListeners(Vec<()>)),
    ("entityListeners" => EntityListeners(Vec<()>)),
    ("phantomListeners" => PhantomListeners(Vec<()>)),
    ("constraintListeners" => ConstraintListeners(Vec<()>)),
    ("worldDeletionListeners" => WorldDeletionListeners(Vec<()>)),
    ("islandActivationListeners" => IslandActivationListeners(Vec<()>)),
    ("worldPostSimulationListeners" => WorldPostSimulationListeners(Vec<()>)),
    ("worldPostIntegrateListeners" => WorldPostIntegrateListeners(Vec<()>)),
    ("worldPostCollideListeners" => WorldPostCollideListeners(Vec<()>)),
    ("islandPostIntegrateListeners" => IslandPostIntegrateListeners(Vec<()>)),
    ("islandPostCollideListeners" => IslandPostCollideListeners(Vec<()>)),
    ("contactListeners" => ContactListeners(Vec<()>)),
    ("contactImpulseLimitBreachedListeners" => ContactImpulseLimitBreachedListeners(Vec<()>)),
    ("worldExtensions" => WorldExtensions(Vec<()>)),
    ("violatedConstraintArray" => ViolatedConstraintArray(())),
    ("broadPhaseBorder" => BroadPhaseBorder(())),
    ("destructionWorld" => DestructionWorld(())),
    ("npWorld" => NpWorld(())),
    ("broadPhaseExtents" => BroadPhaseExtents(Vector4<f32>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("broadPhaseUpdateSize" => BroadPhaseUpdateSize(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReintegrationRecollideMode {
    #[serde(rename = "RR_MODE_REINTEGRATE")]
    RrModeReintegrate = 1,
    #[serde(rename = "RR_MODE_RECOLLIDE_BROADPHASE")]
    RrModeRecollideBroadphase = 2,
    #[serde(rename = "RR_MODE_RECOLLIDE_NARROWPHASE")]
    RrModeRecollideNarrowphase = 4,
    #[serde(rename = "RR_MODE_ALL")]
    RrModeAll = 7,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MtAccessChecking {
    #[serde(rename = "MT_ACCESS_CHECKING_ENABLED")]
    MtAccessCheckingEnabled = 0,
    #[serde(rename = "MT_ACCESS_CHECKING_DISABLED")]
    MtAccessCheckingDisabled = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CachedAabbUpdate {
    #[serde(rename = "SHIFT_BROADPHASE_UPDATE_ENTITY_AABBS")]
    ShiftBroadphaseUpdateEntityAabbs = 0,
    #[serde(rename = "SHIFT_BROADPHASE_IGNORE_ENTITY_AABBS")]
    ShiftBroadphaseIgnoreEntityAabbs = 1,
}
