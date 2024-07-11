//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleContactConstraintDataInfo`
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

/// `hkpSimpleContactConstraintDataInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xb59d1734`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimpleContactConstraintDataInfo {
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub flags: u16,
    /// # C++ Class Fields Info
    /// -   name:`"index"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub index: u16,
    /// # C++ Class Fields Info
    /// -   name:`"internalData0"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub internal_data_0: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rolling_friction_multiplier: f32,
    /// # C++ Class Fields Info
    /// -   name:`"internalData1"`
    /// -   type: `hkHalf`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub internal_data_1: f32,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint32[5]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub data: CStyleArray<[u32; 5]>,
}

impl Serialize for HkpSimpleContactConstraintDataInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimpleContactConstraintDataInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimpleContactConstraintDataInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimpleContactConstraintDataInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSimpleContactConstraintDataInfoVisitor>> for HkpSimpleContactConstraintDataInfo {
    fn from(_values: Vec<HkpSimpleContactConstraintDataInfoVisitor>) -> Self {
            let mut flags = None;
            let mut index = None;
            let mut internal_data_0 = None;
            let mut rolling_friction_multiplier = None;
            let mut internal_data_1 = None;
            let mut data = None;


        for _value in _values {
            match _value {
                HkpSimpleContactConstraintDataInfoVisitor::Flags(m) => flags = Some(m),
                HkpSimpleContactConstraintDataInfoVisitor::Index(m) => index = Some(m),
                HkpSimpleContactConstraintDataInfoVisitor::InternalData0(m) => internal_data_0 = Some(m),
                HkpSimpleContactConstraintDataInfoVisitor::RollingFrictionMultiplier(m) => rolling_friction_multiplier = Some(m),
                HkpSimpleContactConstraintDataInfoVisitor::InternalData1(m) => internal_data_1 = Some(m),
                HkpSimpleContactConstraintDataInfoVisitor::Data(m) => data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            flags: flags.unwrap_or_default().into_inner(),
            index: index.unwrap_or_default().into_inner(),
            internal_data_0: internal_data_0.unwrap_or_default().into_inner(),
            rolling_friction_multiplier: rolling_friction_multiplier.unwrap_or_default().into_inner(),
            internal_data_1: internal_data_1.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSimpleContactConstraintDataInfo> for Vec<HkpSimpleContactConstraintDataInfoVisitor> {
    fn from(data: &HkpSimpleContactConstraintDataInfo) -> Self {
        vec![
            HkpSimpleContactConstraintDataInfoVisitor::Flags(data.flags.into()),
            HkpSimpleContactConstraintDataInfoVisitor::Index(data.index.into()),
            HkpSimpleContactConstraintDataInfoVisitor::InternalData0(data.internal_data_0.into()),
            HkpSimpleContactConstraintDataInfoVisitor::RollingFrictionMultiplier(data.rolling_friction_multiplier.into()),
            HkpSimpleContactConstraintDataInfoVisitor::InternalData1(data.internal_data_1.into()),
            HkpSimpleContactConstraintDataInfoVisitor::Data(data.data.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimpleContactConstraintDataInfo {
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
enum HkpSimpleContactConstraintDataInfoVisitor {
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "index")]
    Index(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "internalData0")]
    InternalData0(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "internalData1")]
    InternalData1(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(CStyleArray<[u32; 5]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintDataInfoVisitor, "@name",
    ("flags" => Flags(Primitive<u16>)),
    ("index" => Index(Primitive<u16>)),
    ("internalData0" => InternalData0(Primitive<f32>)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("internalData1" => InternalData1(Primitive<f32>)),
    ("data" => Data(CStyleArray<[u32; 5]>)),
}
