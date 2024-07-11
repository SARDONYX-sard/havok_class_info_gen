//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaFootstepAnalysisInfo`
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

/// `hkaFootstepAnalysisInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 152
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x824faf75`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaFootstepAnalysisInfo {
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
    /// -   name:`"name"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub name: HkArrayRef<char>,
    /// # C++ Class Fields Info
    /// -   name:`"nameStrike"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name_strike: HkArrayRef<char>,
    /// # C++ Class Fields Info
    /// -   name:`"nameLift"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name_lift: HkArrayRef<char>,
    /// # C++ Class Fields Info
    /// -   name:`"nameLock"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub name_lock: HkArrayRef<char>,
    /// # C++ Class Fields Info
    /// -   name:`"nameUnlock"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub name_unlock: HkArrayRef<char>,
    /// # C++ Class Fields Info
    /// -   name:`"minPos"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub min_pos: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"maxPos"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub max_pos: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"minVel"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub min_vel: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"maxVel"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub max_vel: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"allBonesDown"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub all_bones_down: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"anyBonesDown"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub any_bones_down: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"posTol"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub pos_tol: f32,
    /// # C++ Class Fields Info
    /// -   name:`"velTol"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub vel_tol: f32,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
}

impl Serialize for HkaFootstepAnalysisInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaFootstepAnalysisInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaFootstepAnalysisInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaFootstepAnalysisInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaFootstepAnalysisInfoVisitor>> for HkaFootstepAnalysisInfo {
    fn from(_values: Vec<HkaFootstepAnalysisInfoVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut name_strike = None;
            let mut name_lift = None;
            let mut name_lock = None;
            let mut name_unlock = None;
            let mut min_pos = None;
            let mut max_pos = None;
            let mut min_vel = None;
            let mut max_vel = None;
            let mut all_bones_down = None;
            let mut any_bones_down = None;
            let mut pos_tol = None;
            let mut vel_tol = None;
            let mut duration = None;


        for _value in _values {
            match _value {
                HkaFootstepAnalysisInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaFootstepAnalysisInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaFootstepAnalysisInfoVisitor::Name(m) => name = Some(m),
                HkaFootstepAnalysisInfoVisitor::NameStrike(m) => name_strike = Some(m),
                HkaFootstepAnalysisInfoVisitor::NameLift(m) => name_lift = Some(m),
                HkaFootstepAnalysisInfoVisitor::NameLock(m) => name_lock = Some(m),
                HkaFootstepAnalysisInfoVisitor::NameUnlock(m) => name_unlock = Some(m),
                HkaFootstepAnalysisInfoVisitor::MinPos(m) => min_pos = Some(m),
                HkaFootstepAnalysisInfoVisitor::MaxPos(m) => max_pos = Some(m),
                HkaFootstepAnalysisInfoVisitor::MinVel(m) => min_vel = Some(m),
                HkaFootstepAnalysisInfoVisitor::MaxVel(m) => max_vel = Some(m),
                HkaFootstepAnalysisInfoVisitor::AllBonesDown(m) => all_bones_down = Some(m),
                HkaFootstepAnalysisInfoVisitor::AnyBonesDown(m) => any_bones_down = Some(m),
                HkaFootstepAnalysisInfoVisitor::PosTol(m) => pos_tol = Some(m),
                HkaFootstepAnalysisInfoVisitor::VelTol(m) => vel_tol = Some(m),
                HkaFootstepAnalysisInfoVisitor::Duration(m) => duration = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default(),
            name_strike: name_strike.unwrap_or_default(),
            name_lift: name_lift.unwrap_or_default(),
            name_lock: name_lock.unwrap_or_default(),
            name_unlock: name_unlock.unwrap_or_default(),
            min_pos: min_pos.unwrap_or_default(),
            max_pos: max_pos.unwrap_or_default(),
            min_vel: min_vel.unwrap_or_default(),
            max_vel: max_vel.unwrap_or_default(),
            all_bones_down: all_bones_down.unwrap_or_default(),
            any_bones_down: any_bones_down.unwrap_or_default(),
            pos_tol: pos_tol.unwrap_or_default().into_inner(),
            vel_tol: vel_tol.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaFootstepAnalysisInfo> for Vec<HkaFootstepAnalysisInfoVisitor> {
    fn from(data: &HkaFootstepAnalysisInfo) -> Self {
        vec![
            HkaFootstepAnalysisInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaFootstepAnalysisInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkaFootstepAnalysisInfoVisitor::Name(data.name.clone()),
            HkaFootstepAnalysisInfoVisitor::NameStrike(data.name_strike.clone()),
            HkaFootstepAnalysisInfoVisitor::NameLift(data.name_lift.clone()),
            HkaFootstepAnalysisInfoVisitor::NameLock(data.name_lock.clone()),
            HkaFootstepAnalysisInfoVisitor::NameUnlock(data.name_unlock.clone()),
            HkaFootstepAnalysisInfoVisitor::MinPos(data.min_pos.clone()),
            HkaFootstepAnalysisInfoVisitor::MaxPos(data.max_pos.clone()),
            HkaFootstepAnalysisInfoVisitor::MinVel(data.min_vel.clone()),
            HkaFootstepAnalysisInfoVisitor::MaxVel(data.max_vel.clone()),
            HkaFootstepAnalysisInfoVisitor::AllBonesDown(data.all_bones_down.clone()),
            HkaFootstepAnalysisInfoVisitor::AnyBonesDown(data.any_bones_down.clone()),
            HkaFootstepAnalysisInfoVisitor::PosTol(data.pos_tol.into()),
            HkaFootstepAnalysisInfoVisitor::VelTol(data.vel_tol.into()),
            HkaFootstepAnalysisInfoVisitor::Duration(data.duration.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaFootstepAnalysisInfo {
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
enum HkaFootstepAnalysisInfoVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "name")]
    Name(HkArrayRef<char>),
    /// Visitor fields
    #[serde(rename = "nameStrike")]
    NameStrike(HkArrayRef<char>),
    /// Visitor fields
    #[serde(rename = "nameLift")]
    NameLift(HkArrayRef<char>),
    /// Visitor fields
    #[serde(rename = "nameLock")]
    NameLock(HkArrayRef<char>),
    /// Visitor fields
    #[serde(rename = "nameUnlock")]
    NameUnlock(HkArrayRef<char>),
    /// Visitor fields
    #[serde(rename = "minPos")]
    MinPos(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "maxPos")]
    MaxPos(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "minVel")]
    MinVel(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "maxVel")]
    MaxVel(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "allBonesDown")]
    AllBonesDown(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "anyBonesDown")]
    AnyBonesDown(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "posTol")]
    PosTol(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "velTol")]
    VelTol(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaFootstepAnalysisInfoVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(HkArrayRef<char>)),
    ("nameStrike" => NameStrike(HkArrayRef<char>)),
    ("nameLift" => NameLift(HkArrayRef<char>)),
    ("nameLock" => NameLock(HkArrayRef<char>)),
    ("nameUnlock" => NameUnlock(HkArrayRef<char>)),
    ("minPos" => MinPos(HkArrayNum<f32>)),
    ("maxPos" => MaxPos(HkArrayNum<f32>)),
    ("minVel" => MinVel(HkArrayNum<f32>)),
    ("maxVel" => MaxVel(HkArrayNum<f32>)),
    ("allBonesDown" => AllBonesDown(HkArrayNum<f32>)),
    ("anyBonesDown" => AnyBonesDown(HkArrayNum<f32>)),
    ("posTol" => PosTol(Primitive<f32>)),
    ("velTol" => VelTol(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
}
