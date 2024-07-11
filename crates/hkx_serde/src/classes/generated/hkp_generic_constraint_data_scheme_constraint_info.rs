//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintDataSchemeConstraintInfo`
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

/// `hkpGenericConstraintDataSchemeConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xd6421f19`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpGenericConstraintDataSchemeConstraintInfo {
    /// # C++ Class Fields Info
    /// -   name:`"maxSizeOfSchema"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub max_size_of_schema: i32,
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfSchemas"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub size_of_schemas: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numSolverResults"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub num_solver_results: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numSolverElemTemps"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub num_solver_elem_temps: i32,
}

impl Serialize for HkpGenericConstraintDataSchemeConstraintInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpGenericConstraintDataSchemeConstraintInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpGenericConstraintDataSchemeConstraintInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpGenericConstraintDataSchemeConstraintInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpGenericConstraintDataSchemeConstraintInfoVisitor>> for HkpGenericConstraintDataSchemeConstraintInfo {
    fn from(_values: Vec<HkpGenericConstraintDataSchemeConstraintInfoVisitor>) -> Self {
            let mut max_size_of_schema = None;
            let mut size_of_schemas = None;
            let mut num_solver_results = None;
            let mut num_solver_elem_temps = None;


        for _value in _values {
            match _value {
                HkpGenericConstraintDataSchemeConstraintInfoVisitor::MaxSizeOfSchema(m) => max_size_of_schema = Some(m),
                HkpGenericConstraintDataSchemeConstraintInfoVisitor::SizeOfSchemas(m) => size_of_schemas = Some(m),
                HkpGenericConstraintDataSchemeConstraintInfoVisitor::NumSolverResults(m) => num_solver_results = Some(m),
                HkpGenericConstraintDataSchemeConstraintInfoVisitor::NumSolverElemTemps(m) => num_solver_elem_temps = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            max_size_of_schema: max_size_of_schema.unwrap_or_default().into_inner(),
            size_of_schemas: size_of_schemas.unwrap_or_default().into_inner(),
            num_solver_results: num_solver_results.unwrap_or_default().into_inner(),
            num_solver_elem_temps: num_solver_elem_temps.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpGenericConstraintDataSchemeConstraintInfo> for Vec<HkpGenericConstraintDataSchemeConstraintInfoVisitor> {
    fn from(data: &HkpGenericConstraintDataSchemeConstraintInfo) -> Self {
        vec![
            HkpGenericConstraintDataSchemeConstraintInfoVisitor::MaxSizeOfSchema(data.max_size_of_schema.into()),
            HkpGenericConstraintDataSchemeConstraintInfoVisitor::SizeOfSchemas(data.size_of_schemas.into()),
            HkpGenericConstraintDataSchemeConstraintInfoVisitor::NumSolverResults(data.num_solver_results.into()),
            HkpGenericConstraintDataSchemeConstraintInfoVisitor::NumSolverElemTemps(data.num_solver_elem_temps.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpGenericConstraintDataSchemeConstraintInfo {
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
enum HkpGenericConstraintDataSchemeConstraintInfoVisitor {
    /// Visitor fields
    #[serde(rename = "maxSizeOfSchema")]
    MaxSizeOfSchema(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "sizeOfSchemas")]
    SizeOfSchemas(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numSolverResults")]
    NumSolverResults(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numSolverElemTemps")]
    NumSolverElemTemps(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataSchemeConstraintInfoVisitor, "@name",
    ("maxSizeOfSchema" => MaxSizeOfSchema(Primitive<i32>)),
    ("sizeOfSchemas" => SizeOfSchemas(Primitive<i32>)),
    ("numSolverResults" => NumSolverResults(Primitive<i32>)),
    ("numSolverElemTemps" => NumSolverElemTemps(Primitive<i32>)),
}
