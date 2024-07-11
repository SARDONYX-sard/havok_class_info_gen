//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintInstance`
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

/// `hkpConstraintInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x34eba5f`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConstraintInstance<'a> {
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
    /// -   name:`"owner"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub owner: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintModifiers"`
    /// -   type: `struct hkpModifierConstraintAtom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub constraint_modifiers: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"entities"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub entities: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"priority"`
    /// -   type: `enum ConstraintPriority`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub priority: ConstraintPriority,
    /// # C++ Class Fields Info
    /// -   name:`"wantRuntime"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    pub want_runtime: bool,
    /// # C++ Class Fields Info
    /// -   name:`"destructionRemapInfo"`
    /// -   type: `enum OnDestructionRemapInfo`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    pub destruction_remap_info: OnDestructionRemapInfo,
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub listeners: SingleClass<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Class Fields Info
    /// -   name:`"internal"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub internal: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub uid: u32,
}

impl Serialize for HkpConstraintInstance<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConstraintInstanceVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConstraintInstance<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConstraintInstanceVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConstraintInstanceVisitor<'a>>> for HkpConstraintInstance<'a> {
    fn from(_values: Vec<HkpConstraintInstanceVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut owner = None;
            let mut data = None;
            let mut constraint_modifiers = None;
            let mut entities = None;
            let mut priority = None;
            let mut want_runtime = None;
            let mut destruction_remap_info = None;
            let mut listeners = None;
            let mut name = None;
            let mut user_data = None;
            let mut internal = None;
            let mut uid = None;


        for _value in _values {
            match _value {
                HkpConstraintInstanceVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConstraintInstanceVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConstraintInstanceVisitor::Owner(m) => owner = Some(m),
                HkpConstraintInstanceVisitor::Data(m) => data = Some(m),
                HkpConstraintInstanceVisitor::ConstraintModifiers(m) => constraint_modifiers = Some(m),
                HkpConstraintInstanceVisitor::Entities(m) => entities = Some(m),
                HkpConstraintInstanceVisitor::Priority(m) => priority = Some(m),
                HkpConstraintInstanceVisitor::WantRuntime(m) => want_runtime = Some(m),
                HkpConstraintInstanceVisitor::DestructionRemapInfo(m) => destruction_remap_info = Some(m),
                HkpConstraintInstanceVisitor::Listeners(m) => listeners = Some(m),
                HkpConstraintInstanceVisitor::Name(m) => name = Some(m),
                HkpConstraintInstanceVisitor::UserData(m) => user_data = Some(m),
                HkpConstraintInstanceVisitor::Internal(m) => internal = Some(m),
                HkpConstraintInstanceVisitor::Uid(m) => uid = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            owner: owner.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default().into_inner(),
            constraint_modifiers: constraint_modifiers.unwrap_or_default().into_inner(),
            entities: entities.unwrap_or_default().into_inner(),
            priority: priority.unwrap_or_default().into_inner(),
            want_runtime: want_runtime.unwrap_or_default().into_inner(),
            destruction_remap_info: destruction_remap_info.unwrap_or_default().into_inner(),
            listeners: listeners.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            internal: internal.unwrap_or_default().into_inner(),
            uid: uid.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConstraintInstance<'a>> for Vec<HkpConstraintInstanceVisitor<'a>> {
    fn from(data: &HkpConstraintInstance<'a>) -> Self {
        vec![
            HkpConstraintInstanceVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConstraintInstanceVisitor::ReferenceCount(data.reference_count.into()),
            HkpConstraintInstanceVisitor::Owner(data.owner.clone().into()),
            HkpConstraintInstanceVisitor::Data(data.data.clone().into()),
            HkpConstraintInstanceVisitor::ConstraintModifiers(data.constraint_modifiers.clone().into()),
            HkpConstraintInstanceVisitor::Entities(data.entities.clone().into()),
            HkpConstraintInstanceVisitor::Priority(data.priority.clone().into()),
            HkpConstraintInstanceVisitor::WantRuntime(data.want_runtime.into()),
            HkpConstraintInstanceVisitor::DestructionRemapInfo(data.destruction_remap_info.clone().into()),
            HkpConstraintInstanceVisitor::Listeners(data.listeners.clone()),
            HkpConstraintInstanceVisitor::Name(data.name.clone().into()),
            HkpConstraintInstanceVisitor::UserData(data.user_data.into()),
            HkpConstraintInstanceVisitor::Internal(data.internal.clone().into()),
            HkpConstraintInstanceVisitor::Uid(data.uid.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConstraintInstance<'de> {
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
enum HkpConstraintInstanceVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "owner", skip_serializing)]
    Owner(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraintModifiers")]
    ConstraintModifiers(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "entities")]
    Entities(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "priority")]
    Priority(Primitive<ConstraintPriority>),
    /// Visitor fields
    #[serde(rename = "wantRuntime")]
    WantRuntime(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "destructionRemapInfo")]
    DestructionRemapInfo(Primitive<OnDestructionRemapInfo>),
    /// Visitor fields
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(SingleClass<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "internal", skip_serializing)]
    Internal(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "uid", skip_serializing)]
    Uid(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintInstanceVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("owner" => Owner(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("constraintModifiers" => ConstraintModifiers(Primitive<Cow<'de, str>>)),
    ("entities" => Entities(Primitive<Cow<'de, str>>)),
    ("priority" => Priority(Primitive<ConstraintPriority>)),
    ("wantRuntime" => WantRuntime(Primitive<bool>)),
    ("destructionRemapInfo" => DestructionRemapInfo(Primitive<OnDestructionRemapInfo>)),
    ("listeners" => Listeners(SingleClass<HkpConstraintInstanceSmallArraySerializeOverrideType<'de>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("internal" => Internal(Primitive<Cow<'de, str>>)),
    ("uid" => Uid(Primitive<u32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ConstraintPriority {
    #[serde(rename = "PRIORITY_INVALID")]
    #[default]
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum InstanceType {
    #[serde(rename = "TYPE_NORMAL")]
    #[default]
    TypeNormal = 0,
    #[serde(rename = "TYPE_CHAIN")]
    TypeChain = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AddReferences {
    #[serde(rename = "DO_NOT_ADD_REFERENCES")]
    #[default]
    DoNotAddReferences = 0,
    #[serde(rename = "DO_ADD_REFERENCES")]
    DoAddReferences = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum CloningMode {
    #[serde(rename = "CLONE_SHALLOW_IF_NOT_CONSTRAINED_TO_WORLD")]
    #[default]
    CloneShallowIfNotConstrainedToWorld = 0,
    #[serde(rename = "CLONE_DATAS_WITH_MOTORS")]
    CloneDatasWithMotors = 1,
    #[serde(rename = "CLONE_FORCE_SHALLOW")]
    CloneForceShallow = 2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum OnDestructionRemapInfo {
    #[serde(rename = "ON_DESTRUCTION_REMAP")]
    #[default]
    OnDestructionRemap = 0,
    #[serde(rename = "ON_DESTRUCTION_REMOVE")]
    OnDestructionRemove = 1,
    #[serde(rename = "ON_DESTRUCTION_RESET_REMOVE")]
    OnDestructionResetRemove = 2,
}
