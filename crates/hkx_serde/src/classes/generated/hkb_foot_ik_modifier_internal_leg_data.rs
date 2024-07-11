//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkModifierInternalLegData`
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

/// `hkbFootIkModifierInternalLegData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xe5ca3677`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkModifierInternalLegData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub ground_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"footIkSolver"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub foot_ik_solver: Cow<'a, str>,
}

impl Serialize for HkbFootIkModifierInternalLegData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkModifierInternalLegDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkModifierInternalLegData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkModifierInternalLegDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbFootIkModifierInternalLegDataVisitor<'a>>> for HkbFootIkModifierInternalLegData<'a> {
    fn from(_values: Vec<HkbFootIkModifierInternalLegDataVisitor<'a>>) -> Self {
            let mut ground_position = None;
            let mut foot_ik_solver = None;


        for _value in _values {
            match _value {
                HkbFootIkModifierInternalLegDataVisitor::GroundPosition(m) => ground_position = Some(m),
                HkbFootIkModifierInternalLegDataVisitor::FootIkSolver(m) => foot_ik_solver = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            ground_position: ground_position.unwrap_or_default().into_inner(),
            foot_ik_solver: foot_ik_solver.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbFootIkModifierInternalLegData<'a>> for Vec<HkbFootIkModifierInternalLegDataVisitor<'a>> {
    fn from(data: &HkbFootIkModifierInternalLegData<'a>) -> Self {
        vec![
            HkbFootIkModifierInternalLegDataVisitor::GroundPosition(data.ground_position.into()),
            HkbFootIkModifierInternalLegDataVisitor::FootIkSolver(data.foot_ik_solver.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkModifierInternalLegData<'de> {
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
enum HkbFootIkModifierInternalLegDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "groundPosition")]
    GroundPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "footIkSolver", skip_serializing)]
    FootIkSolver(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierInternalLegDataVisitor<'de>, "@name",
    ("groundPosition" => GroundPosition(Primitive<Vector4<f32>>)),
    ("footIkSolver" => FootIkSolver(Primitive<Cow<'de, str>>)),
}
