//! A Rust structure that implements a serializer/deserializer corresponding to `hkpEntity`, a class defined in C++
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
/// -    size: 544
/// -  vtable: true
/// -  parent: hkpWorldObject/`49fb6f2e`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpEntity<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpEntity"`: Name of this class.
    #[serde(default = "HkpEntity::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa03c774b`: Unique value of this class.
    #[serde(default = "HkpEntity::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpEntityHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpEntityHkParam<'a>>
}

impl HkpEntity<'_> {
    /// Return `"hkpEntity"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpEntity".into()
    }

    /// Return `"0xa03c774b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa03c774b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntityHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"material"`
    /// -   type: `struct hkpMaterial`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(HkpMaterial),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitContactImpulseUtilAndFlag"`
    /// -   type: `void*`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "limitContactImpulseUtilAndFlag", skip_serializing)]
    LimitContactImpulseUtilAndFlag(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"breakableBody"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "breakableBody", skip_serializing)]
    BreakableBody(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"solverData"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "solverData", skip_serializing)]
    SolverData(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"storageIndex"`
    /// -   type: `hkUint16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storageIndex")]
    StorageIndex(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactPointCallbackDelay"`
    /// -   type: `hkUint16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointCallbackDelay")]
    ContactPointCallbackDelay(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraintsMaster"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintsMaster", skip_serializing)]
    ConstraintsMaster(HkpEntitySmallArraySerializeOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraintsSlave"`
    /// -   type: `hkArray&lt;hkpConstraintInstance*&gt;`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | NOT_OWNED | SERIALIZE_IGNORED`
    #[serde(rename = "constraintsSlave", skip_serializing)]
    ConstraintsSlave(Vec<Box<HkpConstraintInstance>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraintRuntime"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintRuntime", skip_serializing)]
    ConstraintRuntime(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"simulationIsland"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "simulationIsland", skip_serializing)]
    SimulationIsland(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"autoRemoveLevel"`
    /// -   type: `hkInt8`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoRemoveLevel")]
    AutoRemoveLevel(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"numShapeKeysInContactPointProperties"`
    /// -   type: `hkUint8`
    /// - offset: 209
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numShapeKeysInContactPointProperties")]
    NumShapeKeysInContactPointProperties(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"responseModifierFlags"`
    /// -   type: `hkUint8`
    /// - offset: 210
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "responseModifierFlags")]
    ResponseModifierFlags(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uid")]
    Uid(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"spuCollisionCallback"`
    /// -   type: `struct hkpEntitySpuCollisionCallback`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spuCollisionCallback")]
    SpuCollisionCallback(HkpEntitySpuCollisionCallback),
    /// # Information on fields in the original C++ class
    /// -   name:`"motion"`
    /// -   type: `struct hkpMaxSizeMotion`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motion")]
    Motion(HkpMaxSizeMotion),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 512
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(HkpEntitySmallArraySerializeOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"actions"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 520
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "actions", skip_serializing)]
    Actions(HkpEntitySmallArraySerializeOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 528
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrame")]
    LocalFrame(Box<HkLocalFrame>),
    /// # Information on fields in the original C++ class
    /// -   name:`"extendedListeners"`
    /// -   type: `struct hkpEntityExtendedListeners*`
    /// - offset: 532
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "extendedListeners", skip_serializing)]
    ExtendedListeners(Box<HkpEntityExtendedListeners>),
    /// # Information on fields in the original C++ class
    /// -   name:`"npData"`
    /// -   type: `hkUint32`
    /// - offset: 536
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "npData")]
    NpData(u32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntityHkParam<'de>, "@name",
    ("material" => Material(HkpMaterial)),
    ("limitContactImpulseUtilAndFlag" => LimitContactImpulseUtilAndFlag(())),
    ("damageMultiplier" => DamageMultiplier(f64)),
    ("breakableBody" => BreakableBody(())),
    ("solverData" => SolverData(u32)),
    ("storageIndex" => StorageIndex(u16)),
    ("contactPointCallbackDelay" => ContactPointCallbackDelay(u16)),
    ("constraintsMaster" => ConstraintsMaster(HkpEntitySmallArraySerializeOverrideType)),
    ("constraintsSlave" => ConstraintsSlave(Vec<Box<HkpConstraintInstance>>)),
    ("constraintRuntime" => ConstraintRuntime(Vec<u8>)),
    ("simulationIsland" => SimulationIsland(())),
    ("autoRemoveLevel" => AutoRemoveLevel(i8)),
    ("numShapeKeysInContactPointProperties" => NumShapeKeysInContactPointProperties(u8)),
    ("responseModifierFlags" => ResponseModifierFlags(u8)),
    ("uid" => Uid(u32)),
    ("spuCollisionCallback" => SpuCollisionCallback(HkpEntitySpuCollisionCallback)),
    ("motion" => Motion(HkpMaxSizeMotion)),
    ("contactListeners" => ContactListeners(HkpEntitySmallArraySerializeOverrideType)),
    ("actions" => Actions(HkpEntitySmallArraySerializeOverrideType)),
    ("localFrame" => LocalFrame(Box<HkLocalFrame>)),
    ("extendedListeners" => ExtendedListeners(Box<HkpEntityExtendedListeners>)),
    ("npData" => NpData(u32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SpuCollisionCallbackEventFilter {
    #[serde(rename = "SPU_SEND_NONE")]
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
