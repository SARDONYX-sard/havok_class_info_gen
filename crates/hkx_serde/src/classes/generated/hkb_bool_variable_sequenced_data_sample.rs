//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBoolVariableSequencedDataSample`
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

/// `hkbBoolVariableSequencedDataSample`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x514763dc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBoolVariableSequencedDataSample {
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `hkBool`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub value: bool,
}

impl Serialize for HkbBoolVariableSequencedDataSample {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBoolVariableSequencedDataSampleVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBoolVariableSequencedDataSample {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBoolVariableSequencedDataSampleVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbBoolVariableSequencedDataSampleVisitor>> for HkbBoolVariableSequencedDataSample {
    fn from(_values: Vec<HkbBoolVariableSequencedDataSampleVisitor>) -> Self {
            let mut time = None;
            let mut value = None;


        for _value in _values {
            match _value {
                HkbBoolVariableSequencedDataSampleVisitor::Time(m) => time = Some(m),
                HkbBoolVariableSequencedDataSampleVisitor::Value(m) => value = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            time: time.unwrap_or_default().into_inner(),
            value: value.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbBoolVariableSequencedDataSample> for Vec<HkbBoolVariableSequencedDataSampleVisitor> {
    fn from(data: &HkbBoolVariableSequencedDataSample) -> Self {
        vec![
            HkbBoolVariableSequencedDataSampleVisitor::Time(data.time.into()),
            HkbBoolVariableSequencedDataSampleVisitor::Value(data.value.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBoolVariableSequencedDataSample {
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
enum HkbBoolVariableSequencedDataSampleVisitor {
    /// Visitor fields
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "value")]
    Value(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBoolVariableSequencedDataSampleVisitor, "@name",
    ("time" => Time(Primitive<f32>)),
    ("value" => Value(Primitive<bool>)),
}
