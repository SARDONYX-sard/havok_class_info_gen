//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallAndSocketConstraintDataAtoms`
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

/// `hkpBallAndSocketConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xc73dcaf9`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBallAndSocketConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"pivots"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub pivots: SingleClass<HkpSetLocalTranslationsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub setup_stabilization: SingleClass<HkpSetupStabilizationAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub ball_socket: SingleClass<HkpBallSocketConstraintAtom>,
}

impl Serialize for HkpBallAndSocketConstraintDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBallAndSocketConstraintDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBallAndSocketConstraintDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBallAndSocketConstraintDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpBallAndSocketConstraintDataAtomsVisitor>> for HkpBallAndSocketConstraintDataAtoms {
    fn from(_values: Vec<HkpBallAndSocketConstraintDataAtomsVisitor>) -> Self {
            let mut pivots = None;
            let mut setup_stabilization = None;
            let mut ball_socket = None;


        for _value in _values {
            match _value {
                HkpBallAndSocketConstraintDataAtomsVisitor::Pivots(m) => pivots = Some(m),
                HkpBallAndSocketConstraintDataAtomsVisitor::SetupStabilization(m) => setup_stabilization = Some(m),
                HkpBallAndSocketConstraintDataAtomsVisitor::BallSocket(m) => ball_socket = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            pivots: pivots.unwrap_or_default(),
            setup_stabilization: setup_stabilization.unwrap_or_default(),
            ball_socket: ball_socket.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpBallAndSocketConstraintDataAtoms> for Vec<HkpBallAndSocketConstraintDataAtomsVisitor> {
    fn from(data: &HkpBallAndSocketConstraintDataAtoms) -> Self {
        vec![
            HkpBallAndSocketConstraintDataAtomsVisitor::Pivots(data.pivots.clone()),
            HkpBallAndSocketConstraintDataAtomsVisitor::SetupStabilization(data.setup_stabilization.clone()),
            HkpBallAndSocketConstraintDataAtomsVisitor::BallSocket(data.ball_socket.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBallAndSocketConstraintDataAtoms {
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
enum HkpBallAndSocketConstraintDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "pivots")]
    Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// Visitor fields
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallAndSocketConstraintDataAtomsVisitor, "@name",
    ("pivots" => Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}
