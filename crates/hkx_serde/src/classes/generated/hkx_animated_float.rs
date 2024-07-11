//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxAnimatedFloat`
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

/// `hkxAnimatedFloat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xce8b2fbd`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxAnimatedFloat {
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
    /// -   name:`"floats"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub floats: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"hint"`
    /// -   type: `enum Hint`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub hint: Hint,
}

impl Serialize for HkxAnimatedFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxAnimatedFloatVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxAnimatedFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxAnimatedFloatVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxAnimatedFloatVisitor>> for HkxAnimatedFloat {
    fn from(_values: Vec<HkxAnimatedFloatVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut floats = None;
            let mut hint = None;


        for _value in _values {
            match _value {
                HkxAnimatedFloatVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxAnimatedFloatVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxAnimatedFloatVisitor::Floats(m) => floats = Some(m),
                HkxAnimatedFloatVisitor::Hint(m) => hint = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            floats: floats.unwrap_or_default(),
            hint: hint.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxAnimatedFloat> for Vec<HkxAnimatedFloatVisitor> {
    fn from(data: &HkxAnimatedFloat) -> Self {
        vec![
            HkxAnimatedFloatVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxAnimatedFloatVisitor::ReferenceCount(data.reference_count.into()),
            HkxAnimatedFloatVisitor::Floats(data.floats.clone()),
            HkxAnimatedFloatVisitor::Hint(data.hint.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxAnimatedFloat {
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
enum HkxAnimatedFloatVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "floats")]
    Floats(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "hint")]
    Hint(Primitive<Hint>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAnimatedFloatVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("floats" => Floats(HkArrayNum<f32>)),
    ("hint" => Hint(Primitive<Hint>)),
}
