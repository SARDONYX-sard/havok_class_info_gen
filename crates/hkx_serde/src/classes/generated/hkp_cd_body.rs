//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCdBody`
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

/// `hkpCdBody`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x54a4b841`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCdBody<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub shape_key: u32,
    /// # C++ Class Fields Info
    /// -   name:`"motion"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub motion: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"parent"`
    /// -   type: `struct hkpCdBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub parent: Cow<'a, str>,
}

impl Serialize for HkpCdBody<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCdBodyVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCdBody<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCdBodyVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCdBodyVisitor<'a>>> for HkpCdBody<'a> {
    fn from(_values: Vec<HkpCdBodyVisitor<'a>>) -> Self {
            let mut shape = None;
            let mut shape_key = None;
            let mut motion = None;
            let mut parent = None;


        for _value in _values {
            match _value {
                HkpCdBodyVisitor::Shape(m) => shape = Some(m),
                HkpCdBodyVisitor::ShapeKey(m) => shape_key = Some(m),
                HkpCdBodyVisitor::Motion(m) => motion = Some(m),
                HkpCdBodyVisitor::Parent(m) => parent = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            shape: shape.unwrap_or_default().into_inner(),
            shape_key: shape_key.unwrap_or_default().into_inner(),
            motion: motion.unwrap_or_default().into_inner(),
            parent: parent.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCdBody<'a>> for Vec<HkpCdBodyVisitor<'a>> {
    fn from(data: &HkpCdBody<'a>) -> Self {
        vec![
            HkpCdBodyVisitor::Shape(data.shape.clone().into()),
            HkpCdBodyVisitor::ShapeKey(data.shape_key.into()),
            HkpCdBodyVisitor::Motion(data.motion.clone().into()),
            HkpCdBodyVisitor::Parent(data.parent.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCdBody<'de> {
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
enum HkpCdBodyVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "shapeKey")]
    ShapeKey(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "motion", skip_serializing)]
    Motion(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "parent", skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCdBodyVisitor<'de>, "@name",
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("motion" => Motion(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
}
