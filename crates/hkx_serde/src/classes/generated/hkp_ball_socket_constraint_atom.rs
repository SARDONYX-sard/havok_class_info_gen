//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallSocketConstraintAtom`
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

/// `hkpBallSocketConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xe70e4dfa`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBallSocketConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"solvingMethod"`
    /// -   type: `enum SolvingMethod`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub solving_method: SolvingMethod,
    /// # C++ Class Fields Info
    /// -   name:`"bodiesToNotify"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub bodies_to_notify: u8,
    /// # C++ Class Fields Info
    /// -   name:`"velocityStabilizationFactor"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub velocity_stabilization_factor: u8,
    /// # C++ Class Fields Info
    /// -   name:`"maxImpulse"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub max_impulse: f32,
    /// # C++ Class Fields Info
    /// -   name:`"inertiaStabilizationFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub inertia_stabilization_factor: f32,
}

impl Serialize for HkpBallSocketConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBallSocketConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBallSocketConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBallSocketConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpBallSocketConstraintAtomVisitor>> for HkpBallSocketConstraintAtom {
    fn from(_values: Vec<HkpBallSocketConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut solving_method = None;
            let mut bodies_to_notify = None;
            let mut velocity_stabilization_factor = None;
            let mut max_impulse = None;
            let mut inertia_stabilization_factor = None;


        for _value in _values {
            match _value {
                HkpBallSocketConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpBallSocketConstraintAtomVisitor::SolvingMethod(m) => solving_method = Some(m),
                HkpBallSocketConstraintAtomVisitor::BodiesToNotify(m) => bodies_to_notify = Some(m),
                HkpBallSocketConstraintAtomVisitor::VelocityStabilizationFactor(m) => velocity_stabilization_factor = Some(m),
                HkpBallSocketConstraintAtomVisitor::MaxImpulse(m) => max_impulse = Some(m),
                HkpBallSocketConstraintAtomVisitor::InertiaStabilizationFactor(m) => inertia_stabilization_factor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            solving_method: solving_method.unwrap_or_default().into_inner(),
            bodies_to_notify: bodies_to_notify.unwrap_or_default().into_inner(),
            velocity_stabilization_factor: velocity_stabilization_factor.unwrap_or_default().into_inner(),
            max_impulse: max_impulse.unwrap_or_default().into_inner(),
            inertia_stabilization_factor: inertia_stabilization_factor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpBallSocketConstraintAtom> for Vec<HkpBallSocketConstraintAtomVisitor> {
    fn from(data: &HkpBallSocketConstraintAtom) -> Self {
        vec![
            HkpBallSocketConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpBallSocketConstraintAtomVisitor::SolvingMethod(data.solving_method.clone().into()),
            HkpBallSocketConstraintAtomVisitor::BodiesToNotify(data.bodies_to_notify.into()),
            HkpBallSocketConstraintAtomVisitor::VelocityStabilizationFactor(data.velocity_stabilization_factor.into()),
            HkpBallSocketConstraintAtomVisitor::MaxImpulse(data.max_impulse.into()),
            HkpBallSocketConstraintAtomVisitor::InertiaStabilizationFactor(data.inertia_stabilization_factor.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBallSocketConstraintAtom {
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
enum HkpBallSocketConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "solvingMethod")]
    SolvingMethod(Primitive<SolvingMethod>),
    /// Visitor fields
    #[serde(rename = "bodiesToNotify")]
    BodiesToNotify(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "velocityStabilizationFactor")]
    VelocityStabilizationFactor(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "inertiaStabilizationFactor")]
    InertiaStabilizationFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("solvingMethod" => SolvingMethod(Primitive<SolvingMethod>)),
    ("bodiesToNotify" => BodiesToNotify(Primitive<u8>)),
    ("velocityStabilizationFactor" => VelocityStabilizationFactor(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<f32>)),
    ("inertiaStabilizationFactor" => InertiaStabilizationFactor(Primitive<f32>)),
}
