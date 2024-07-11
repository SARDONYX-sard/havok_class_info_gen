//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableInfo`
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

/// `hkbVariableInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 6
/// -    vtable: false
/// - signature: `0x9e746ba2`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbVariableInfo {
    /// # C++ Class Fields Info
    /// -   name:`"role"`
    /// -   type: `struct hkbRoleAttribute`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub role: SingleClass<HkbRoleAttribute>,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub _type: VariableType,
}

impl Serialize for HkbVariableInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbVariableInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbVariableInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbVariableInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbVariableInfoVisitor>> for HkbVariableInfo {
    fn from(_values: Vec<HkbVariableInfoVisitor>) -> Self {
            let mut role = None;
            let mut _type = None;


        for _value in _values {
            match _value {
                HkbVariableInfoVisitor::Role(m) => role = Some(m),
                HkbVariableInfoVisitor::Type(m) => _type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            role: role.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbVariableInfo> for Vec<HkbVariableInfoVisitor> {
    fn from(data: &HkbVariableInfo) -> Self {
        vec![
            HkbVariableInfoVisitor::Role(data.role.clone()),
            HkbVariableInfoVisitor::Type(data._type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbVariableInfo {
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
enum HkbVariableInfoVisitor {
    /// Visitor fields
    #[serde(rename = "role")]
    Role(SingleClass<HkbRoleAttribute>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<VariableType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableInfoVisitor, "@name",
    ("role" => Role(SingleClass<HkbRoleAttribute>)),
    ("type" => Type(Primitive<VariableType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum VariableType {
    #[serde(rename = "VARIABLE_TYPE_INVALID")]
    #[default]
    VariableTypeInvalid = -1,
    #[serde(rename = "VARIABLE_TYPE_BOOL")]
    VariableTypeBool = 0,
    #[serde(rename = "VARIABLE_TYPE_INT8")]
    VariableTypeInt8 = 1,
    #[serde(rename = "VARIABLE_TYPE_INT16")]
    VariableTypeInt16 = 2,
    #[serde(rename = "VARIABLE_TYPE_INT32")]
    VariableTypeInt32 = 3,
    #[serde(rename = "VARIABLE_TYPE_REAL")]
    VariableTypeReal = 4,
    #[serde(rename = "VARIABLE_TYPE_POINTER")]
    VariableTypePointer = 5,
    #[serde(rename = "VARIABLE_TYPE_VECTOR3")]
    VariableTypeVector3 = 6,
    #[serde(rename = "VARIABLE_TYPE_VECTOR4")]
    VariableTypeVector4 = 7,
    #[serde(rename = "VARIABLE_TYPE_QUATERNION")]
    VariableTypeQuaternion = 8,
}
