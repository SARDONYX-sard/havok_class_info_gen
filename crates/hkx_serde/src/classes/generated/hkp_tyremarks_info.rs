//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTyremarksInfo`
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

/// `hkpTyremarksInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3d0433d6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTyremarksInfo<'a> {
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
    /// -   name:`"minTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub min_tyremark_energy: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub max_tyremark_energy: f32,
    /// # C++ Class Fields Info
    /// -   name:`"tyremarksWheel"`
    /// -   type: `hkArray<hkpTyremarksWheel*>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub tyremarks_wheel: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpTyremarksInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTyremarksInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTyremarksInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTyremarksInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTyremarksInfoVisitor<'a>>> for HkpTyremarksInfo<'a> {
    fn from(_values: Vec<HkpTyremarksInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut min_tyremark_energy = None;
            let mut max_tyremark_energy = None;
            let mut tyremarks_wheel = None;


        for _value in _values {
            match _value {
                HkpTyremarksInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTyremarksInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTyremarksInfoVisitor::MinTyremarkEnergy(m) => min_tyremark_energy = Some(m),
                HkpTyremarksInfoVisitor::MaxTyremarkEnergy(m) => max_tyremark_energy = Some(m),
                HkpTyremarksInfoVisitor::TyremarksWheel(m) => tyremarks_wheel = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            min_tyremark_energy: min_tyremark_energy.unwrap_or_default().into_inner(),
            max_tyremark_energy: max_tyremark_energy.unwrap_or_default().into_inner(),
            tyremarks_wheel: tyremarks_wheel.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTyremarksInfo<'a>> for Vec<HkpTyremarksInfoVisitor<'a>> {
    fn from(data: &HkpTyremarksInfo<'a>) -> Self {
        vec![
            HkpTyremarksInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTyremarksInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkpTyremarksInfoVisitor::MinTyremarkEnergy(data.min_tyremark_energy.into()),
            HkpTyremarksInfoVisitor::MaxTyremarkEnergy(data.max_tyremark_energy.into()),
            HkpTyremarksInfoVisitor::TyremarksWheel(data.tyremarks_wheel.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTyremarksInfo<'de> {
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
enum HkpTyremarksInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "minTyremarkEnergy")]
    MinTyremarkEnergy(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxTyremarkEnergy")]
    MaxTyremarkEnergy(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "tyremarksWheel")]
    TyremarksWheel(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarksInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("minTyremarkEnergy" => MinTyremarkEnergy(Primitive<f32>)),
    ("maxTyremarkEnergy" => MaxTyremarkEnergy(Primitive<f32>)),
    ("tyremarksWheel" => TyremarksWheel(HkArrayRef<Cow<'de, str>>)),
}
