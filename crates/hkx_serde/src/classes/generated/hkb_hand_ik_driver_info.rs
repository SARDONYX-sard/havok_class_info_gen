//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkDriverInfo`
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

/// `hkbHandIkDriverInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc299090a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbHandIkDriverInfo<'a> {
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
    /// -   name:`"hands"`
    /// -   type: `hkArray<struct hkbHandIkDriverInfoHand>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub hands: HkArrayClass<HkbHandIkDriverInfoHand<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"fadeInOutCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub fade_in_out_curve: BlendCurve,
}

impl Serialize for HkbHandIkDriverInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbHandIkDriverInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbHandIkDriverInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbHandIkDriverInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbHandIkDriverInfoVisitor<'a>>> for HkbHandIkDriverInfo<'a> {
    fn from(_values: Vec<HkbHandIkDriverInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut hands = None;
            let mut fade_in_out_curve = None;


        for _value in _values {
            match _value {
                HkbHandIkDriverInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbHandIkDriverInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbHandIkDriverInfoVisitor::Hands(m) => hands = Some(m),
                HkbHandIkDriverInfoVisitor::FadeInOutCurve(m) => fade_in_out_curve = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            hands: hands.unwrap_or_default(),
            fade_in_out_curve: fade_in_out_curve.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbHandIkDriverInfo<'a>> for Vec<HkbHandIkDriverInfoVisitor<'a>> {
    fn from(data: &HkbHandIkDriverInfo<'a>) -> Self {
        vec![
            HkbHandIkDriverInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbHandIkDriverInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbHandIkDriverInfoVisitor::Hands(data.hands.clone()),
            HkbHandIkDriverInfoVisitor::FadeInOutCurve(data.fade_in_out_curve.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbHandIkDriverInfo<'de> {
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
enum HkbHandIkDriverInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "hands")]
    Hands(HkArrayClass<HkbHandIkDriverInfoHand<'a>>),
    /// Visitor fields
    #[serde(rename = "fadeInOutCurve")]
    FadeInOutCurve(Primitive<BlendCurve>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkDriverInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("hands" => Hands(HkArrayClass<HkbHandIkDriverInfoHand<'de>>)),
    ("fadeInOutCurve" => FadeInOutCurve(Primitive<BlendCurve>)),
}
