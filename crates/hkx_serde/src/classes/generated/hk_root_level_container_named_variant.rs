//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRootLevelContainerNamedVariant`
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

/// `hkRootLevelContainerNamedVariant`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xb103a2cd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkRootLevelContainerNamedVariant<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"className"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub class_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variant: Cow<'a, str>,
}

impl Serialize for HkRootLevelContainerNamedVariant<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkRootLevelContainerNamedVariantVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkRootLevelContainerNamedVariant<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkRootLevelContainerNamedVariantVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkRootLevelContainerNamedVariantVisitor<'a>>>
    for HkRootLevelContainerNamedVariant<'a>
{
    fn from(_values: Vec<HkRootLevelContainerNamedVariantVisitor<'a>>) -> Self {
        let mut name = None;
        let mut class_name = None;
        let mut variant = None;

        for _value in _values {
            match _value {
                HkRootLevelContainerNamedVariantVisitor::Name(m) => name = Some(m),
                HkRootLevelContainerNamedVariantVisitor::ClassName(m) => class_name = Some(m),
                HkRootLevelContainerNamedVariantVisitor::Variant(m) => variant = Some(m),
            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            name: name.unwrap_or_default().into_inner(),
            class_name: class_name.unwrap_or_default().into_inner(),
            variant: variant.unwrap_or_default().into_inner(),
        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkRootLevelContainerNamedVariant<'a>>
    for Vec<HkRootLevelContainerNamedVariantVisitor<'a>>
{
    fn from(data: &HkRootLevelContainerNamedVariant<'a>) -> Self {
        vec![
            HkRootLevelContainerNamedVariantVisitor::Name(data.name.clone().into()),
            HkRootLevelContainerNamedVariantVisitor::ClassName(data.class_name.clone().into()),
            HkRootLevelContainerNamedVariantVisitor::Variant(data.variant.clone().into()),
        ]
    }
}

impl<'de> ByteDeSerialize<'de> for HkRootLevelContainerNamedVariant<'de> {
    fn from_bytes<D>(deserializer: &'de D, position: &mut u32) -> Result<Self>
    where
        D: ByteDeserializer,
        Self: Sized + 'de,
    {
        let name = deserializer.read_string_ptr(position)?;
        let class_name = deserializer.read_string_ptr(position)?;
        let variant = deserializer.read_class_ptr(position)?;

        Ok(Self {
            name,
            class_name,
            variant,
        })
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
enum HkRootLevelContainerNamedVariantVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "className")]
    ClassName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "variant")]
    Variant(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRootLevelContainerNamedVariantVisitor<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("className" => ClassName(Primitive<Cow<'de, str>>)),
    ("variant" => Variant(Primitive<Cow<'de, str>>)),
}
