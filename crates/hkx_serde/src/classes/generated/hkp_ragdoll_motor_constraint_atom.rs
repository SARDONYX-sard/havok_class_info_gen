//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollMotorConstraintAtom`
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

/// `hkpRagdollMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x71013826`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRagdollMotorConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: bool,
    /// # C++ Class Fields Info
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub initialized_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"previousTargetAnglesOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub previous_target_angles_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"target_bRca"`
    /// -   type: `hkMatrix3`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub target_b_rca: Matrix3<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub motors: Cow<'a, str>,
}

impl Serialize for HkpRagdollMotorConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRagdollMotorConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRagdollMotorConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRagdollMotorConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpRagdollMotorConstraintAtomVisitor<'a>>> for HkpRagdollMotorConstraintAtom<'a> {
    fn from(_values: Vec<HkpRagdollMotorConstraintAtomVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut initialized_offset = None;
            let mut previous_target_angles_offset = None;
            let mut target_b_rca = None;
            let mut motors = None;


        for _value in _values {
            match _value {
                HkpRagdollMotorConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpRagdollMotorConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpRagdollMotorConstraintAtomVisitor::InitializedOffset(m) => initialized_offset = Some(m),
                HkpRagdollMotorConstraintAtomVisitor::PreviousTargetAnglesOffset(m) => previous_target_angles_offset = Some(m),
                HkpRagdollMotorConstraintAtomVisitor::TargetBRca(m) => target_b_rca = Some(m),
                HkpRagdollMotorConstraintAtomVisitor::Motors(m) => motors = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            initialized_offset: initialized_offset.unwrap_or_default().into_inner(),
            previous_target_angles_offset: previous_target_angles_offset.unwrap_or_default().into_inner(),
            target_b_rca: target_b_rca.unwrap_or_default().into_inner(),
            motors: motors.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpRagdollMotorConstraintAtom<'a>> for Vec<HkpRagdollMotorConstraintAtomVisitor<'a>> {
    fn from(data: &HkpRagdollMotorConstraintAtom<'a>) -> Self {
        vec![
            HkpRagdollMotorConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpRagdollMotorConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpRagdollMotorConstraintAtomVisitor::InitializedOffset(data.initialized_offset.into()),
            HkpRagdollMotorConstraintAtomVisitor::PreviousTargetAnglesOffset(data.previous_target_angles_offset.into()),
            HkpRagdollMotorConstraintAtomVisitor::TargetBRca(data.target_b_rca.into()),
            HkpRagdollMotorConstraintAtomVisitor::Motors(data.motors.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRagdollMotorConstraintAtom<'de> {
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
enum HkpRagdollMotorConstraintAtomVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "initializedOffset")]
    InitializedOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "previousTargetAnglesOffset")]
    PreviousTargetAnglesOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "target_bRca")]
    TargetBRca(Primitive<Matrix3<f32>>),
    /// Visitor fields
    #[serde(rename = "motors")]
    Motors(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollMotorConstraintAtomVisitor<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetAnglesOffset" => PreviousTargetAnglesOffset(Primitive<i16>)),
    ("target_bRca" => TargetBRca(Primitive<Matrix3<f32>>)),
    ("motors" => Motors(Primitive<Cow<'de, str>>)),
}
