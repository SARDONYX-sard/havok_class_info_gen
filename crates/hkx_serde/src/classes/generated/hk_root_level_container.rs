//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRootLevelContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(clippy::clone_on_copy, clippy::unit_arg)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkRootLevelContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2772c11e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkRootLevelContainer<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"namedVariants"`
    /// -   type: `hkArray<struct hkRootLevelContainerNamedVariant>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub named_variants: HkArrayClass<HkRootLevelContainerNamedVariant<'a>>,
}

impl Serialize for HkRootLevelContainer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkRootLevelContainerVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkRootLevelContainer<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkRootLevelContainerVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkRootLevelContainerVisitor<'a>>> for HkRootLevelContainer<'a> {
    fn from(_values: Vec<HkRootLevelContainerVisitor<'a>>) -> Self {
        let mut named_variants = None;

        for _value in _values {
            match _value {
                HkRootLevelContainerVisitor::NamedVariants(m) => named_variants = Some(m),
            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            named_variants: named_variants.unwrap_or_default(),
        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkRootLevelContainer<'a>> for Vec<HkRootLevelContainerVisitor<'a>> {
    fn from(data: &HkRootLevelContainer<'a>) -> Self {
        vec![HkRootLevelContainerVisitor::NamedVariants(
            data.named_variants.clone(),
        )]
    }
}

impl<'de> ByteDeSerialize<'de> for HkRootLevelContainer<'de> {
    fn from_bytes<D>(deserializer: &'de D, position: &mut u32) -> Result<Self>
    where
        D: ByteDeserializer,
        Self: Sized + 'de,
    {
        let named_variants = deserializer.read_class_array(position)?;

        Ok(Self { named_variants })
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
pub enum HkRootLevelContainerVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "namedVariants")]
    NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRootLevelContainerVisitor<'de>, "@name",
    ("namedVariants" => NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant<'de>>)),
}
