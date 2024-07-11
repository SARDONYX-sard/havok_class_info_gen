//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableBindingSet`
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

/// `hkbVariableBindingSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x338ad4ff`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbVariableBindingSet<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"bindings"`
    /// -   type: `hkArray<struct hkbVariableBindingSetBinding>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub bindings: HkArrayClass<HkbVariableBindingSetBinding<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"indexOfBindingToEnable"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub index_of_binding_to_enable: i32,
    /// # C++ Class Fields Info
    /// -   name:`"hasOutputBinding"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub has_output_binding: bool,
}

impl Serialize for HkbVariableBindingSet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbVariableBindingSetVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbVariableBindingSet<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbVariableBindingSetVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbVariableBindingSetVisitor<'a>>> for HkbVariableBindingSet<'a> {
    fn from(_values: Vec<HkbVariableBindingSetVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut bindings = None;
            let mut index_of_binding_to_enable = None;
            let mut has_output_binding = None;


        for _value in _values {
            match _value {
                HkbVariableBindingSetVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbVariableBindingSetVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbVariableBindingSetVisitor::Bindings(m) => bindings = Some(m),
                HkbVariableBindingSetVisitor::IndexOfBindingToEnable(m) => index_of_binding_to_enable = Some(m),
                HkbVariableBindingSetVisitor::HasOutputBinding(m) => has_output_binding = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            bindings: bindings.unwrap_or_default(),
            index_of_binding_to_enable: index_of_binding_to_enable.unwrap_or_default().into_inner(),
            has_output_binding: has_output_binding.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbVariableBindingSet<'a>> for Vec<HkbVariableBindingSetVisitor<'a>> {
    fn from(data: &HkbVariableBindingSet<'a>) -> Self {
        vec![
            HkbVariableBindingSetVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbVariableBindingSetVisitor::ReferenceCount(data.reference_count.into()),
            HkbVariableBindingSetVisitor::Bindings(data.bindings.clone()),
            HkbVariableBindingSetVisitor::IndexOfBindingToEnable(data.index_of_binding_to_enable.into()),
            HkbVariableBindingSetVisitor::HasOutputBinding(data.has_output_binding.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbVariableBindingSet<'de> {
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
enum HkbVariableBindingSetVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "bindings")]
    Bindings(HkArrayClass<HkbVariableBindingSetBinding<'a>>),
    /// Visitor fields
    #[serde(rename = "indexOfBindingToEnable")]
    IndexOfBindingToEnable(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "hasOutputBinding", skip_serializing)]
    HasOutputBinding(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSetVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bindings" => Bindings(HkArrayClass<HkbVariableBindingSetBinding<'de>>)),
    ("indexOfBindingToEnable" => IndexOfBindingToEnable(Primitive<i32>)),
    ("hasOutputBinding" => HasOutputBinding(Primitive<bool>)),
}
