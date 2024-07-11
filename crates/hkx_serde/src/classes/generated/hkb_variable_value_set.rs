//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableValueSet`
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

/// `hkbVariableValueSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x27812d8d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbVariableValueSet<'a> {
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
    /// -   name:`"wordVariableValues"`
    /// -   type: `hkArray<struct hkbVariableValue>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub word_variable_values: HkArrayClass<HkbVariableValue>,
    /// # C++ Class Fields Info
    /// -   name:`"quadVariableValues"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub quad_variable_values: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"variantVariableValues"`
    /// -   type: `hkArray<hkReferencedObject*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub variant_variable_values: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkbVariableValueSet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbVariableValueSetVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbVariableValueSet<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbVariableValueSetVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbVariableValueSetVisitor<'a>>> for HkbVariableValueSet<'a> {
    fn from(_values: Vec<HkbVariableValueSetVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut word_variable_values = None;
            let mut quad_variable_values = None;
            let mut variant_variable_values = None;


        for _value in _values {
            match _value {
                HkbVariableValueSetVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbVariableValueSetVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbVariableValueSetVisitor::WordVariableValues(m) => word_variable_values = Some(m),
                HkbVariableValueSetVisitor::QuadVariableValues(m) => quad_variable_values = Some(m),
                HkbVariableValueSetVisitor::VariantVariableValues(m) => variant_variable_values = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            word_variable_values: word_variable_values.unwrap_or_default(),
            quad_variable_values: quad_variable_values.unwrap_or_default(),
            variant_variable_values: variant_variable_values.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbVariableValueSet<'a>> for Vec<HkbVariableValueSetVisitor<'a>> {
    fn from(data: &HkbVariableValueSet<'a>) -> Self {
        vec![
            HkbVariableValueSetVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbVariableValueSetVisitor::ReferenceCount(data.reference_count.into()),
            HkbVariableValueSetVisitor::WordVariableValues(data.word_variable_values.clone()),
            HkbVariableValueSetVisitor::QuadVariableValues(data.quad_variable_values.clone()),
            HkbVariableValueSetVisitor::VariantVariableValues(data.variant_variable_values.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbVariableValueSet<'de> {
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
enum HkbVariableValueSetVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "wordVariableValues")]
    WordVariableValues(HkArrayClass<HkbVariableValue>),
    /// Visitor fields
    #[serde(rename = "quadVariableValues")]
    QuadVariableValues(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "variantVariableValues")]
    VariantVariableValues(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueSetVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wordVariableValues" => WordVariableValues(HkArrayClass<HkbVariableValue>)),
    ("quadVariableValues" => QuadVariableValues(HkArrayVector<Vector4<f32>>)),
    ("variantVariableValues" => VariantVariableValues(HkArrayRef<Cow<'de, str>>)),
}
