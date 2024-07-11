//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTyremarkPoint`
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

/// `hkpTyremarkPoint`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x6bb7c5e8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTyremarkPoint {
    /// # C++ Class Fields Info
    /// -   name:`"pointLeft"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub point_left: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"pointRight"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub point_right: Vector4<f32>,
}

impl Serialize for HkpTyremarkPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTyremarkPointVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTyremarkPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTyremarkPointVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpTyremarkPointVisitor>> for HkpTyremarkPoint {
    fn from(_values: Vec<HkpTyremarkPointVisitor>) -> Self {
            let mut point_left = None;
            let mut point_right = None;


        for _value in _values {
            match _value {
                HkpTyremarkPointVisitor::PointLeft(m) => point_left = Some(m),
                HkpTyremarkPointVisitor::PointRight(m) => point_right = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            point_left: point_left.unwrap_or_default().into_inner(),
            point_right: point_right.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpTyremarkPoint> for Vec<HkpTyremarkPointVisitor> {
    fn from(data: &HkpTyremarkPoint) -> Self {
        vec![
            HkpTyremarkPointVisitor::PointLeft(data.point_left.into()),
            HkpTyremarkPointVisitor::PointRight(data.point_right.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTyremarkPoint {
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
enum HkpTyremarkPointVisitor {
    /// Visitor fields
    #[serde(rename = "pointLeft")]
    PointLeft(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "pointRight")]
    PointRight(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarkPointVisitor, "@name",
    ("pointLeft" => PointLeft(Primitive<Vector4<f32>>)),
    ("pointRight" => PointRight(Primitive<Vector4<f32>>)),
}
