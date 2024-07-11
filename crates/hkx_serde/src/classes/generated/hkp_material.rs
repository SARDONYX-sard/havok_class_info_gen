//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMaterial`
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

/// `hkpMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x33be6570`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMaterial {
    /// # C++ Class Fields Info
    /// -   name:`"responseType"`
    /// -   type: `enum ResponseType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub response_type: ResponseType,
    /// # C++ Class Fields Info
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub rolling_friction_multiplier: f32,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"restitution"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub restitution: f32,
}

impl Serialize for HkpMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMaterialVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMaterialVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpMaterialVisitor>> for HkpMaterial {
    fn from(_values: Vec<HkpMaterialVisitor>) -> Self {
            let mut response_type = None;
            let mut rolling_friction_multiplier = None;
            let mut friction = None;
            let mut restitution = None;


        for _value in _values {
            match _value {
                HkpMaterialVisitor::ResponseType(m) => response_type = Some(m),
                HkpMaterialVisitor::RollingFrictionMultiplier(m) => rolling_friction_multiplier = Some(m),
                HkpMaterialVisitor::Friction(m) => friction = Some(m),
                HkpMaterialVisitor::Restitution(m) => restitution = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            response_type: response_type.unwrap_or_default().into_inner(),
            rolling_friction_multiplier: rolling_friction_multiplier.unwrap_or_default().into_inner(),
            friction: friction.unwrap_or_default().into_inner(),
            restitution: restitution.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpMaterial> for Vec<HkpMaterialVisitor> {
    fn from(data: &HkpMaterial) -> Self {
        vec![
            HkpMaterialVisitor::ResponseType(data.response_type.clone().into()),
            HkpMaterialVisitor::RollingFrictionMultiplier(data.rolling_friction_multiplier.into()),
            HkpMaterialVisitor::Friction(data.friction.into()),
            HkpMaterialVisitor::Restitution(data.restitution.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMaterial {
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
enum HkpMaterialVisitor {
    /// Visitor fields
    #[serde(rename = "responseType")]
    ResponseType(Primitive<ResponseType>),
    /// Visitor fields
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "restitution")]
    Restitution(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMaterialVisitor, "@name",
    ("responseType" => ResponseType(Primitive<ResponseType>)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("restitution" => Restitution(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ResponseType {
    #[serde(rename = "RESPONSE_INVALID")]
    #[default]
    ResponseInvalid = 0,
    #[serde(rename = "RESPONSE_SIMPLE_CONTACT")]
    ResponseSimpleContact = 1,
    #[serde(rename = "RESPONSE_REPORTING")]
    ResponseReporting = 2,
    #[serde(rename = "RESPONSE_NONE")]
    ResponseNone = 3,
    #[serde(rename = "RESPONSE_MAX_ID")]
    ResponseMaxId = 4,
}
