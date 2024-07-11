//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetupStabilizationAtom`
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

/// `hkpSetupStabilizationAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf05d137e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSetupStabilizationAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"enabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub enabled: bool,
    /// # C++ Class Fields Info
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub max_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkUint8[8]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub padding: CStyleArray<[u8; 8]>,
}

impl Serialize for HkpSetupStabilizationAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSetupStabilizationAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSetupStabilizationAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSetupStabilizationAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSetupStabilizationAtomVisitor>> for HkpSetupStabilizationAtom {
    fn from(_values: Vec<HkpSetupStabilizationAtomVisitor>) -> Self {
            let mut _type = None;
            let mut enabled = None;
            let mut max_angle = None;
            let mut padding = None;


        for _value in _values {
            match _value {
                HkpSetupStabilizationAtomVisitor::Type(m) => _type = Some(m),
                HkpSetupStabilizationAtomVisitor::Enabled(m) => enabled = Some(m),
                HkpSetupStabilizationAtomVisitor::MaxAngle(m) => max_angle = Some(m),
                HkpSetupStabilizationAtomVisitor::Padding(m) => padding = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            enabled: enabled.unwrap_or_default().into_inner(),
            max_angle: max_angle.unwrap_or_default().into_inner(),
            padding: padding.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSetupStabilizationAtom> for Vec<HkpSetupStabilizationAtomVisitor> {
    fn from(data: &HkpSetupStabilizationAtom) -> Self {
        vec![
            HkpSetupStabilizationAtomVisitor::Type(data._type.clone().into()),
            HkpSetupStabilizationAtomVisitor::Enabled(data.enabled.into()),
            HkpSetupStabilizationAtomVisitor::MaxAngle(data.max_angle.into()),
            HkpSetupStabilizationAtomVisitor::Padding(data.padding.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSetupStabilizationAtom {
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
enum HkpSetupStabilizationAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "enabled")]
    Enabled(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "padding")]
    Padding(CStyleArray<[u8; 8]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetupStabilizationAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("enabled" => Enabled(Primitive<bool>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("padding" => Padding(CStyleArray<[u8; 8]>)),
}
