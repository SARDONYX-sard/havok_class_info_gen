//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainMapperLinkInfo`
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

/// `hkpPoweredChainMapperLinkInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xcf071a1b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPoweredChainMapperLinkInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"firstTargetIdx"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub first_target_idx: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numTargets"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub num_targets: i32,
    /// # C++ Class Fields Info
    /// -   name:`"limitConstraint"`
    /// -   type: `struct hkpConstraintInstance*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub limit_constraint: Cow<'a, str>,
}

impl Serialize for HkpPoweredChainMapperLinkInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPoweredChainMapperLinkInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPoweredChainMapperLinkInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPoweredChainMapperLinkInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPoweredChainMapperLinkInfoVisitor<'a>>> for HkpPoweredChainMapperLinkInfo<'a> {
    fn from(_values: Vec<HkpPoweredChainMapperLinkInfoVisitor<'a>>) -> Self {
            let mut first_target_idx = None;
            let mut num_targets = None;
            let mut limit_constraint = None;


        for _value in _values {
            match _value {
                HkpPoweredChainMapperLinkInfoVisitor::FirstTargetIdx(m) => first_target_idx = Some(m),
                HkpPoweredChainMapperLinkInfoVisitor::NumTargets(m) => num_targets = Some(m),
                HkpPoweredChainMapperLinkInfoVisitor::LimitConstraint(m) => limit_constraint = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            first_target_idx: first_target_idx.unwrap_or_default().into_inner(),
            num_targets: num_targets.unwrap_or_default().into_inner(),
            limit_constraint: limit_constraint.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPoweredChainMapperLinkInfo<'a>> for Vec<HkpPoweredChainMapperLinkInfoVisitor<'a>> {
    fn from(data: &HkpPoweredChainMapperLinkInfo<'a>) -> Self {
        vec![
            HkpPoweredChainMapperLinkInfoVisitor::FirstTargetIdx(data.first_target_idx.into()),
            HkpPoweredChainMapperLinkInfoVisitor::NumTargets(data.num_targets.into()),
            HkpPoweredChainMapperLinkInfoVisitor::LimitConstraint(data.limit_constraint.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPoweredChainMapperLinkInfo<'de> {
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
enum HkpPoweredChainMapperLinkInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "firstTargetIdx")]
    FirstTargetIdx(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numTargets")]
    NumTargets(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "limitConstraint")]
    LimitConstraint(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperLinkInfoVisitor<'de>, "@name",
    ("firstTargetIdx" => FirstTargetIdx(Primitive<i32>)),
    ("numTargets" => NumTargets(Primitive<i32>)),
    ("limitConstraint" => LimitConstraint(Primitive<Cow<'de, str>>)),
}
