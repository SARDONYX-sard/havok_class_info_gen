//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvaluateExpressionModifierInternalExpressionData`
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

/// `hkbEvaluateExpressionModifierInternalExpressionData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0xb8686f6b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEvaluateExpressionModifierInternalExpressionData {
    /// # C++ Class Fields Info
    /// -   name:`"raisedEvent"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub raised_event: bool,
    /// # C++ Class Fields Info
    /// -   name:`"wasTrueInPreviousFrame"`
    /// -   type: `hkBool`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub was_true_in_previous_frame: bool,
}

impl Serialize for HkbEvaluateExpressionModifierInternalExpressionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEvaluateExpressionModifierInternalExpressionDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEvaluateExpressionModifierInternalExpressionData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEvaluateExpressionModifierInternalExpressionDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbEvaluateExpressionModifierInternalExpressionDataVisitor>> for HkbEvaluateExpressionModifierInternalExpressionData {
    fn from(_values: Vec<HkbEvaluateExpressionModifierInternalExpressionDataVisitor>) -> Self {
            let mut raised_event = None;
            let mut was_true_in_previous_frame = None;


        for _value in _values {
            match _value {
                HkbEvaluateExpressionModifierInternalExpressionDataVisitor::RaisedEvent(m) => raised_event = Some(m),
                HkbEvaluateExpressionModifierInternalExpressionDataVisitor::WasTrueInPreviousFrame(m) => was_true_in_previous_frame = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            raised_event: raised_event.unwrap_or_default().into_inner(),
            was_true_in_previous_frame: was_true_in_previous_frame.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbEvaluateExpressionModifierInternalExpressionData> for Vec<HkbEvaluateExpressionModifierInternalExpressionDataVisitor> {
    fn from(data: &HkbEvaluateExpressionModifierInternalExpressionData) -> Self {
        vec![
            HkbEvaluateExpressionModifierInternalExpressionDataVisitor::RaisedEvent(data.raised_event.into()),
            HkbEvaluateExpressionModifierInternalExpressionDataVisitor::WasTrueInPreviousFrame(data.was_true_in_previous_frame.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEvaluateExpressionModifierInternalExpressionData {
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
enum HkbEvaluateExpressionModifierInternalExpressionDataVisitor {
    /// Visitor fields
    #[serde(rename = "raisedEvent")]
    RaisedEvent(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "wasTrueInPreviousFrame")]
    WasTrueInPreviousFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateExpressionModifierInternalExpressionDataVisitor, "@name",
    ("raisedEvent" => RaisedEvent(Primitive<bool>)),
    ("wasTrueInPreviousFrame" => WasTrueInPreviousFrame(Primitive<bool>)),
}
