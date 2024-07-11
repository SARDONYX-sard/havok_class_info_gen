//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallSocketChainDataConstraintInfo`
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

/// `hkpBallSocketChainDataConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xc9cbedf2`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBallSocketChainDataConstraintInfo {
    /// # C++ Class Fields Info
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub pivot_in_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub pivot_in_b: Vector4<f32>,
}

impl Serialize for HkpBallSocketChainDataConstraintInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBallSocketChainDataConstraintInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBallSocketChainDataConstraintInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBallSocketChainDataConstraintInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpBallSocketChainDataConstraintInfoVisitor>> for HkpBallSocketChainDataConstraintInfo {
    fn from(_values: Vec<HkpBallSocketChainDataConstraintInfoVisitor>) -> Self {
            let mut pivot_in_a = None;
            let mut pivot_in_b = None;


        for _value in _values {
            match _value {
                HkpBallSocketChainDataConstraintInfoVisitor::PivotInA(m) => pivot_in_a = Some(m),
                HkpBallSocketChainDataConstraintInfoVisitor::PivotInB(m) => pivot_in_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            pivot_in_a: pivot_in_a.unwrap_or_default().into_inner(),
            pivot_in_b: pivot_in_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpBallSocketChainDataConstraintInfo> for Vec<HkpBallSocketChainDataConstraintInfoVisitor> {
    fn from(data: &HkpBallSocketChainDataConstraintInfo) -> Self {
        vec![
            HkpBallSocketChainDataConstraintInfoVisitor::PivotInA(data.pivot_in_a.into()),
            HkpBallSocketChainDataConstraintInfoVisitor::PivotInB(data.pivot_in_b.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBallSocketChainDataConstraintInfo {
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
enum HkpBallSocketChainDataConstraintInfoVisitor {
    /// Visitor fields
    #[serde(rename = "pivotInA")]
    PivotInA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "pivotInB")]
    PivotInB(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketChainDataConstraintInfoVisitor, "@name",
    ("pivotInA" => PivotInA(Primitive<Vector4<f32>>)),
    ("pivotInB" => PivotInB(Primitive<Vector4<f32>>)),
}
