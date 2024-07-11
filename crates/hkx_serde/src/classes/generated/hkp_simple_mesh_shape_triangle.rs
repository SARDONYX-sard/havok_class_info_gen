//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleMeshShapeTriangle`
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

/// `hkpSimpleMeshShapeTriangle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xd38738c1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimpleMeshShapeTriangle {
    /// # C++ Class Fields Info
    /// -   name:`"a"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub a: i32,
    /// # C++ Class Fields Info
    /// -   name:`"b"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub b: i32,
    /// # C++ Class Fields Info
    /// -   name:`"c"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub c: i32,
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub welding_info: u16,
}

impl Serialize for HkpSimpleMeshShapeTriangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimpleMeshShapeTriangleVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimpleMeshShapeTriangle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimpleMeshShapeTriangleVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSimpleMeshShapeTriangleVisitor>> for HkpSimpleMeshShapeTriangle {
    fn from(_values: Vec<HkpSimpleMeshShapeTriangleVisitor>) -> Self {
            let mut a = None;
            let mut b = None;
            let mut c = None;
            let mut welding_info = None;


        for _value in _values {
            match _value {
                HkpSimpleMeshShapeTriangleVisitor::A(m) => a = Some(m),
                HkpSimpleMeshShapeTriangleVisitor::B(m) => b = Some(m),
                HkpSimpleMeshShapeTriangleVisitor::C(m) => c = Some(m),
                HkpSimpleMeshShapeTriangleVisitor::WeldingInfo(m) => welding_info = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            a: a.unwrap_or_default().into_inner(),
            b: b.unwrap_or_default().into_inner(),
            c: c.unwrap_or_default().into_inner(),
            welding_info: welding_info.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSimpleMeshShapeTriangle> for Vec<HkpSimpleMeshShapeTriangleVisitor> {
    fn from(data: &HkpSimpleMeshShapeTriangle) -> Self {
        vec![
            HkpSimpleMeshShapeTriangleVisitor::A(data.a.into()),
            HkpSimpleMeshShapeTriangleVisitor::B(data.b.into()),
            HkpSimpleMeshShapeTriangleVisitor::C(data.c.into()),
            HkpSimpleMeshShapeTriangleVisitor::WeldingInfo(data.welding_info.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimpleMeshShapeTriangle {
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
enum HkpSimpleMeshShapeTriangleVisitor {
    /// Visitor fields
    #[serde(rename = "a")]
    A(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "b")]
    B(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "c")]
    C(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleMeshShapeTriangleVisitor, "@name",
    ("a" => A(Primitive<i32>)),
    ("b" => B(Primitive<i32>)),
    ("c" => C(Primitive<i32>)),
    ("weldingInfo" => WeldingInfo(Primitive<u16>)),
}
