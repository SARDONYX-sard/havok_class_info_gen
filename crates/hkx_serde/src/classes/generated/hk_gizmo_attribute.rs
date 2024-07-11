//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkGizmoAttribute`
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

/// `hkGizmoAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x23aadfb6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkGizmoAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub visible: bool,
    /// # C++ Class Fields Info
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub label: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum GizmoType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: GizmoType,
}

impl Serialize for HkGizmoAttribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkGizmoAttributeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkGizmoAttribute<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkGizmoAttributeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkGizmoAttributeVisitor<'a>>> for HkGizmoAttribute<'a> {
    fn from(_values: Vec<HkGizmoAttributeVisitor<'a>>) -> Self {
            let mut visible = None;
            let mut label = None;
            let mut _type = None;


        for _value in _values {
            match _value {
                HkGizmoAttributeVisitor::Visible(m) => visible = Some(m),
                HkGizmoAttributeVisitor::Label(m) => label = Some(m),
                HkGizmoAttributeVisitor::Type(m) => _type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            visible: visible.unwrap_or_default().into_inner(),
            label: label.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkGizmoAttribute<'a>> for Vec<HkGizmoAttributeVisitor<'a>> {
    fn from(data: &HkGizmoAttribute<'a>) -> Self {
        vec![
            HkGizmoAttributeVisitor::Visible(data.visible.into()),
            HkGizmoAttributeVisitor::Label(data.label.clone().into()),
            HkGizmoAttributeVisitor::Type(data._type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkGizmoAttribute<'de> {
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
enum HkGizmoAttributeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "label")]
    Label(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<GizmoType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGizmoAttributeVisitor<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("label" => Label(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<GizmoType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum GizmoType {
    #[serde(rename = "POINT")]
    #[default]
    Point = 0,
    #[serde(rename = "SPHERE")]
    Sphere = 1,
    #[serde(rename = "PLANE")]
    Plane = 2,
    #[serde(rename = "ARROW")]
    Arrow = 3,
}
