//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStiffSpringChainDataConstraintInfo`
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

/// `hkpStiffSpringChainDataConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xc624a180`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStiffSpringChainDataConstraintInfo {
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
    /// # C++ Class Fields Info
    /// -   name:`"springLength"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub spring_length: f32,
}

impl Serialize for HkpStiffSpringChainDataConstraintInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStiffSpringChainDataConstraintInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStiffSpringChainDataConstraintInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStiffSpringChainDataConstraintInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpStiffSpringChainDataConstraintInfoVisitor>> for HkpStiffSpringChainDataConstraintInfo {
    fn from(_values: Vec<HkpStiffSpringChainDataConstraintInfoVisitor>) -> Self {
            let mut pivot_in_a = None;
            let mut pivot_in_b = None;
            let mut spring_length = None;


        for _value in _values {
            match _value {
                HkpStiffSpringChainDataConstraintInfoVisitor::PivotInA(m) => pivot_in_a = Some(m),
                HkpStiffSpringChainDataConstraintInfoVisitor::PivotInB(m) => pivot_in_b = Some(m),
                HkpStiffSpringChainDataConstraintInfoVisitor::SpringLength(m) => spring_length = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            pivot_in_a: pivot_in_a.unwrap_or_default().into_inner(),
            pivot_in_b: pivot_in_b.unwrap_or_default().into_inner(),
            spring_length: spring_length.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpStiffSpringChainDataConstraintInfo> for Vec<HkpStiffSpringChainDataConstraintInfoVisitor> {
    fn from(data: &HkpStiffSpringChainDataConstraintInfo) -> Self {
        vec![
            HkpStiffSpringChainDataConstraintInfoVisitor::PivotInA(data.pivot_in_a.into()),
            HkpStiffSpringChainDataConstraintInfoVisitor::PivotInB(data.pivot_in_b.into()),
            HkpStiffSpringChainDataConstraintInfoVisitor::SpringLength(data.spring_length.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStiffSpringChainDataConstraintInfo {
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
enum HkpStiffSpringChainDataConstraintInfoVisitor {
    /// Visitor fields
    #[serde(rename = "pivotInA")]
    PivotInA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "pivotInB")]
    PivotInB(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "springLength")]
    SpringLength(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringChainDataConstraintInfoVisitor, "@name",
    ("pivotInA" => PivotInA(Primitive<Vector4<f32>>)),
    ("pivotInB" => PivotInB(Primitive<Vector4<f32>>)),
    ("springLength" => SpringLength(Primitive<f32>)),
}
