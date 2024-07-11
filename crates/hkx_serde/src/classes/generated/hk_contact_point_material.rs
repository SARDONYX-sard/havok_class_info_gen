//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkContactPointMaterial`
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

/// `hkContactPointMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x4e32287c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkContactPointMaterial {
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub friction: u8,
    /// # C++ Class Fields Info
    /// -   name:`"restitution"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    pub restitution: u8,
    /// # C++ Class Fields Info
    /// -   name:`"maxImpulse"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub max_impulse: u8,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    pub flags: u8,
}

impl Serialize for HkContactPointMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkContactPointMaterialVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkContactPointMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkContactPointMaterialVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkContactPointMaterialVisitor>> for HkContactPointMaterial {
    fn from(_values: Vec<HkContactPointMaterialVisitor>) -> Self {
            let mut user_data = None;
            let mut friction = None;
            let mut restitution = None;
            let mut max_impulse = None;
            let mut flags = None;


        for _value in _values {
            match _value {
                HkContactPointMaterialVisitor::UserData(m) => user_data = Some(m),
                HkContactPointMaterialVisitor::Friction(m) => friction = Some(m),
                HkContactPointMaterialVisitor::Restitution(m) => restitution = Some(m),
                HkContactPointMaterialVisitor::MaxImpulse(m) => max_impulse = Some(m),
                HkContactPointMaterialVisitor::Flags(m) => flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            user_data: user_data.unwrap_or_default().into_inner(),
            friction: friction.unwrap_or_default().into_inner(),
            restitution: restitution.unwrap_or_default().into_inner(),
            max_impulse: max_impulse.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkContactPointMaterial> for Vec<HkContactPointMaterialVisitor> {
    fn from(data: &HkContactPointMaterial) -> Self {
        vec![
            HkContactPointMaterialVisitor::UserData(data.user_data.into()),
            HkContactPointMaterialVisitor::Friction(data.friction.into()),
            HkContactPointMaterialVisitor::Restitution(data.restitution.into()),
            HkContactPointMaterialVisitor::MaxImpulse(data.max_impulse.into()),
            HkContactPointMaterialVisitor::Flags(data.flags.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkContactPointMaterial {
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
enum HkContactPointMaterialVisitor {
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "restitution")]
    Restitution(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkContactPointMaterialVisitor, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("friction" => Friction(Primitive<u8>)),
    ("restitution" => Restitution(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<u8>)),
    ("flags" => Flags(Primitive<u8>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum FlagEnum {
    #[serde(rename = "CONTACT_IS_NEW")]
    #[default]
    ContactIsNew = 1,
    #[serde(rename = "CONTACT_USES_SOLVER_PATH2")]
    ContactUsesSolverPath2 = 2,
    #[serde(rename = "CONTACT_BREAKOFF_OBJECT_ID")]
    ContactBreakoffObjectId = 4,
    #[serde(rename = "CONTACT_IS_DISABLED")]
    ContactIsDisabled = 8,
}
