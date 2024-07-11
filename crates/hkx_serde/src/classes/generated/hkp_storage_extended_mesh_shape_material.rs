//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeMaterial`
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

/// `hkpStorageExtendedMeshShapeMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpMeshMaterial`/`0x886cde0c`
/// - signature: `0x2ca3e906`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStorageExtendedMeshShapeMaterial {
    /// # C++ Parent class(`hkpMeshMaterial` => parent: `None`) field Info
    /// -   name:`"filterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub filter_info: u32,

    /// # C++ Class Fields Info
    /// -   name:`"restitution"`
    /// -   type: `hkHalf`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub restitution: f32,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkHalf`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
}

impl Serialize for HkpStorageExtendedMeshShapeMaterial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStorageExtendedMeshShapeMaterialVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStorageExtendedMeshShapeMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStorageExtendedMeshShapeMaterialVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpStorageExtendedMeshShapeMaterialVisitor>> for HkpStorageExtendedMeshShapeMaterial {
    fn from(_values: Vec<HkpStorageExtendedMeshShapeMaterialVisitor>) -> Self {
            let mut filter_info = None;
            let mut restitution = None;
            let mut friction = None;
            let mut user_data = None;


        for _value in _values {
            match _value {
                HkpStorageExtendedMeshShapeMaterialVisitor::FilterInfo(m) => filter_info = Some(m),
                HkpStorageExtendedMeshShapeMaterialVisitor::Restitution(m) => restitution = Some(m),
                HkpStorageExtendedMeshShapeMaterialVisitor::Friction(m) => friction = Some(m),
                HkpStorageExtendedMeshShapeMaterialVisitor::UserData(m) => user_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            filter_info: filter_info.unwrap_or_default().into_inner(),
            restitution: restitution.unwrap_or_default().into_inner(),
            friction: friction.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpStorageExtendedMeshShapeMaterial> for Vec<HkpStorageExtendedMeshShapeMaterialVisitor> {
    fn from(data: &HkpStorageExtendedMeshShapeMaterial) -> Self {
        vec![
            HkpStorageExtendedMeshShapeMaterialVisitor::FilterInfo(data.filter_info.into()),
            HkpStorageExtendedMeshShapeMaterialVisitor::Restitution(data.restitution.into()),
            HkpStorageExtendedMeshShapeMaterialVisitor::Friction(data.friction.into()),
            HkpStorageExtendedMeshShapeMaterialVisitor::UserData(data.user_data.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStorageExtendedMeshShapeMaterial {
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
enum HkpStorageExtendedMeshShapeMaterialVisitor {
    /// Visitor fields
    #[serde(rename = "filterInfo")]
    FilterInfo(Primitive<u32>),

    /// Visitor fields
    #[serde(rename = "restitution")]
    Restitution(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMaterialVisitor, "@name",
    ("filterInfo" => FilterInfo(Primitive<u32>)),
    ("restitution" => Restitution(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
}
