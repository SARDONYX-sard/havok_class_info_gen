//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConstraintInstance`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConstraintInstance<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConstraintInstance"`: Name of this class.
    #[serde(default = "HkpConstraintInstance::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x34eba5f`: Unique value of this class.
    #[serde(default = "HkpConstraintInstance::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConstraintInstanceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConstraintInstanceHkParam<'a>>
}

impl HkpConstraintInstance<'_> {
    /// Return `"hkpConstraintInstance"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConstraintInstance".into()
    }

    /// Return `"0x34eba5f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x34eba5f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintInstanceHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"owner"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "owner", skip_serializing)]
    Owner(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Box<HkpConstraintData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraintModifiers"`
    /// -   type: `struct hkpModifierConstraintAtom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintModifiers")]
    ConstraintModifiers(Box<HkpModifierConstraintAtom>),
    /// # Information on fields in the original C++ class
    /// -   name:`"entities"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entities")]
    Entities(Box<HkpEntity>),
    /// # Information on fields in the original C++ class
    /// -   name:`"priority"`
    /// -   type: `enum ConstraintPriority`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(ConstraintPriority),
    /// # Information on fields in the original C++ class
    /// -   name:`"wantRuntime"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantRuntime")]
    WantRuntime(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"destructionRemapInfo"`
    /// -   type: `enum OnDestructionRemapInfo`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "destructionRemapInfo")]
    DestructionRemapInfo(OnDestructionRemapInfo),
    /// # Information on fields in the original C++ class
    /// -   name:`"listeners"`
    /// -   type: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"internal"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internal", skip_serializing)]
    Internal(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uid", skip_serializing)]
    Uid(u32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintInstanceHkParam<'de>, "@name",
    ("owner" => Owner(())),
    ("data" => Data(Box<HkpConstraintData>)),
    ("constraintModifiers" => ConstraintModifiers(Box<HkpModifierConstraintAtom>)),
    ("entities" => Entities(Box<HkpEntity>)),
    ("priority" => Priority(ConstraintPriority)),
    ("wantRuntime" => WantRuntime(bool)),
    ("destructionRemapInfo" => DestructionRemapInfo(OnDestructionRemapInfo)),
    ("listeners" => Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType)),
    ("name" => Name(String)),
    ("userData" => UserData(u64)),
    ("internal" => Internal(())),
    ("uid" => Uid(u32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ConstraintPriority {
    #[serde(rename = "PRIORITY_INVALID")]
    PriorityInvalid = 0,
    #[serde(rename = "PRIORITY_PSI")]
    PriorityPsi = 1,
    #[serde(rename = "PRIORITY_SIMPLIFIED_TOI_UNUSED")]
    PrioritySimplifiedToiUnused = 2,
    #[serde(rename = "PRIORITY_TOI")]
    PriorityToi = 3,
    #[serde(rename = "PRIORITY_TOI_HIGHER")]
    PriorityToiHigher = 4,
    #[serde(rename = "PRIORITY_TOI_FORCED")]
    PriorityToiForced = 5,
    #[serde(rename = "NUM_PRIORITIES")]
    NumPriorities = 6,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceType {
    #[serde(rename = "TYPE_NORMAL")]
    TypeNormal = 0,
    #[serde(rename = "TYPE_CHAIN")]
    TypeChain = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddReferences {
    #[serde(rename = "DO_NOT_ADD_REFERENCES")]
    DoNotAddReferences = 0,
    #[serde(rename = "DO_ADD_REFERENCES")]
    DoAddReferences = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CloningMode {
    #[serde(rename = "CLONE_SHALLOW_IF_NOT_CONSTRAINED_TO_WORLD")]
    CloneShallowIfNotConstrainedToWorld = 0,
    #[serde(rename = "CLONE_DATAS_WITH_MOTORS")]
    CloneDatasWithMotors = 1,
    #[serde(rename = "CLONE_FORCE_SHALLOW")]
    CloneForceShallow = 2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OnDestructionRemapInfo {
    #[serde(rename = "ON_DESTRUCTION_REMAP")]
    OnDestructionRemap = 0,
    #[serde(rename = "ON_DESTRUCTION_REMOVE")]
    OnDestructionRemove = 1,
    #[serde(rename = "ON_DESTRUCTION_RESET_REMOVE")]
    OnDestructionResetRemove = 2,
}
