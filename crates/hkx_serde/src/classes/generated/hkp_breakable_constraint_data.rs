//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBreakableConstraintData`
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

/// `hkpBreakableConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x7d6310c8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBreakableConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub atoms: SingleClass<HkpBridgeAtoms<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub constraint_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"childRuntimeSize"`
    /// -   type: `hkUint16`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub child_runtime_size: u16,
    /// # C++ Class Fields Info
    /// -   name:`"childNumSolverResults"`
    /// -   type: `hkUint16`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    pub child_num_solver_results: u16,
    /// # C++ Class Fields Info
    /// -   name:`"solverResultLimit"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub solver_result_limit: f32,
    /// # C++ Class Fields Info
    /// -   name:`"removeWhenBroken"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub remove_when_broken: bool,
    /// # C++ Class Fields Info
    /// -   name:`"revertBackVelocityOnBreak"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    pub revert_back_velocity_on_break: bool,
}

impl Serialize for HkpBreakableConstraintData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBreakableConstraintDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBreakableConstraintData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBreakableConstraintDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpBreakableConstraintDataVisitor<'a>>> for HkpBreakableConstraintData<'a> {
    fn from(_values: Vec<HkpBreakableConstraintDataVisitor<'a>>) -> Self {
            let mut atoms = None;
            let mut constraint_data = None;
            let mut child_runtime_size = None;
            let mut child_num_solver_results = None;
            let mut solver_result_limit = None;
            let mut remove_when_broken = None;
            let mut revert_back_velocity_on_break = None;


        for _value in _values {
            match _value {
                HkpBreakableConstraintDataVisitor::Atoms(m) => atoms = Some(m),
                HkpBreakableConstraintDataVisitor::ConstraintData(m) => constraint_data = Some(m),
                HkpBreakableConstraintDataVisitor::ChildRuntimeSize(m) => child_runtime_size = Some(m),
                HkpBreakableConstraintDataVisitor::ChildNumSolverResults(m) => child_num_solver_results = Some(m),
                HkpBreakableConstraintDataVisitor::SolverResultLimit(m) => solver_result_limit = Some(m),
                HkpBreakableConstraintDataVisitor::RemoveWhenBroken(m) => remove_when_broken = Some(m),
                HkpBreakableConstraintDataVisitor::RevertBackVelocityOnBreak(m) => revert_back_velocity_on_break = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            atoms: atoms.unwrap_or_default(),
            constraint_data: constraint_data.unwrap_or_default().into_inner(),
            child_runtime_size: child_runtime_size.unwrap_or_default().into_inner(),
            child_num_solver_results: child_num_solver_results.unwrap_or_default().into_inner(),
            solver_result_limit: solver_result_limit.unwrap_or_default().into_inner(),
            remove_when_broken: remove_when_broken.unwrap_or_default().into_inner(),
            revert_back_velocity_on_break: revert_back_velocity_on_break.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpBreakableConstraintData<'a>> for Vec<HkpBreakableConstraintDataVisitor<'a>> {
    fn from(data: &HkpBreakableConstraintData<'a>) -> Self {
        vec![
            HkpBreakableConstraintDataVisitor::Atoms(data.atoms.clone()),
            HkpBreakableConstraintDataVisitor::ConstraintData(data.constraint_data.clone().into()),
            HkpBreakableConstraintDataVisitor::ChildRuntimeSize(data.child_runtime_size.into()),
            HkpBreakableConstraintDataVisitor::ChildNumSolverResults(data.child_num_solver_results.into()),
            HkpBreakableConstraintDataVisitor::SolverResultLimit(data.solver_result_limit.into()),
            HkpBreakableConstraintDataVisitor::RemoveWhenBroken(data.remove_when_broken.into()),
            HkpBreakableConstraintDataVisitor::RevertBackVelocityOnBreak(data.revert_back_velocity_on_break.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBreakableConstraintData<'de> {
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
enum HkpBreakableConstraintDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// Visitor fields
    #[serde(rename = "constraintData")]
    ConstraintData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "childRuntimeSize")]
    ChildRuntimeSize(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "childNumSolverResults")]
    ChildNumSolverResults(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "solverResultLimit")]
    SolverResultLimit(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "removeWhenBroken")]
    RemoveWhenBroken(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "revertBackVelocityOnBreak")]
    RevertBackVelocityOnBreak(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBreakableConstraintDataVisitor<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
    ("childRuntimeSize" => ChildRuntimeSize(Primitive<u16>)),
    ("childNumSolverResults" => ChildNumSolverResults(Primitive<u16>)),
    ("solverResultLimit" => SolverResultLimit(Primitive<f32>)),
    ("removeWhenBroken" => RemoveWhenBroken(Primitive<bool>)),
    ("revertBackVelocityOnBreak" => RevertBackVelocityOnBreak(Primitive<bool>)),
}
