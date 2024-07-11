//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkIndexedTransformSet`
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

/// `hkIndexedTransformSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x87fe6b5c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkIndexedTransformSet<'a> {
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
    /// -   name:`"matrices"`
    /// -   type: `hkArray<hkMatrix4>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub matrices: HkArrayMatrix4<Matrix4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"inverseMatrices"`
    /// -   type: `hkArray<hkMatrix4>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub inverse_matrices: HkArrayMatrix4<Matrix4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"matricesOrder"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub matrices_order: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"matricesNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub matrices_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"indexMappings"`
    /// -   type: `hkArray<struct hkMeshBoneIndexMapping>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub index_mappings: HkArrayClass<HkMeshBoneIndexMapping>,
    /// # C++ Class Fields Info
    /// -   name:`"allMatricesAreAffine"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub all_matrices_are_affine: bool,
}

impl Serialize for HkIndexedTransformSet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkIndexedTransformSetVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkIndexedTransformSet<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkIndexedTransformSetVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkIndexedTransformSetVisitor<'a>>> for HkIndexedTransformSet<'a> {
    fn from(_values: Vec<HkIndexedTransformSetVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut matrices = None;
            let mut inverse_matrices = None;
            let mut matrices_order = None;
            let mut matrices_names = None;
            let mut index_mappings = None;
            let mut all_matrices_are_affine = None;


        for _value in _values {
            match _value {
                HkIndexedTransformSetVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkIndexedTransformSetVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkIndexedTransformSetVisitor::Matrices(m) => matrices = Some(m),
                HkIndexedTransformSetVisitor::InverseMatrices(m) => inverse_matrices = Some(m),
                HkIndexedTransformSetVisitor::MatricesOrder(m) => matrices_order = Some(m),
                HkIndexedTransformSetVisitor::MatricesNames(m) => matrices_names = Some(m),
                HkIndexedTransformSetVisitor::IndexMappings(m) => index_mappings = Some(m),
                HkIndexedTransformSetVisitor::AllMatricesAreAffine(m) => all_matrices_are_affine = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            matrices: matrices.unwrap_or_default(),
            inverse_matrices: inverse_matrices.unwrap_or_default(),
            matrices_order: matrices_order.unwrap_or_default(),
            matrices_names: matrices_names.unwrap_or_default(),
            index_mappings: index_mappings.unwrap_or_default(),
            all_matrices_are_affine: all_matrices_are_affine.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkIndexedTransformSet<'a>> for Vec<HkIndexedTransformSetVisitor<'a>> {
    fn from(data: &HkIndexedTransformSet<'a>) -> Self {
        vec![
            HkIndexedTransformSetVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkIndexedTransformSetVisitor::ReferenceCount(data.reference_count.into()),
            HkIndexedTransformSetVisitor::Matrices(data.matrices.clone()),
            HkIndexedTransformSetVisitor::InverseMatrices(data.inverse_matrices.clone()),
            HkIndexedTransformSetVisitor::MatricesOrder(data.matrices_order.clone()),
            HkIndexedTransformSetVisitor::MatricesNames(data.matrices_names.clone()),
            HkIndexedTransformSetVisitor::IndexMappings(data.index_mappings.clone()),
            HkIndexedTransformSetVisitor::AllMatricesAreAffine(data.all_matrices_are_affine.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkIndexedTransformSet<'de> {
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
enum HkIndexedTransformSetVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "matrices")]
    Matrices(HkArrayMatrix4<Matrix4<f32>>),
    /// Visitor fields
    #[serde(rename = "inverseMatrices")]
    InverseMatrices(HkArrayMatrix4<Matrix4<f32>>),
    /// Visitor fields
    #[serde(rename = "matricesOrder")]
    MatricesOrder(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "matricesNames")]
    MatricesNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "indexMappings")]
    IndexMappings(HkArrayClass<HkMeshBoneIndexMapping>),
    /// Visitor fields
    #[serde(rename = "allMatricesAreAffine")]
    AllMatricesAreAffine(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkIndexedTransformSetVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("matrices" => Matrices(HkArrayMatrix4<Matrix4<f32>>)),
    ("inverseMatrices" => InverseMatrices(HkArrayMatrix4<Matrix4<f32>>)),
    ("matricesOrder" => MatricesOrder(HkArrayNum<i16>)),
    ("matricesNames" => MatricesNames(HkArrayStringPtr<'de>)),
    ("indexMappings" => IndexMappings(HkArrayClass<HkMeshBoneIndexMapping>)),
    ("allMatricesAreAffine" => AllMatricesAreAffine(Primitive<bool>)),
}
