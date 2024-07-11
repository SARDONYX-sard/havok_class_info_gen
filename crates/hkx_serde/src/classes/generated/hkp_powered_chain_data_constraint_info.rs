//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainDataConstraintInfo`
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

/// `hkpPoweredChainDataConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xf88aee25`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPoweredChainDataConstraintInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub pivot_in_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub pivot_in_b: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"aTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub a_tc: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"bTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub b_tc: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub motors: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"switchBodies"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub switch_bodies: bool,
}

impl Serialize for HkpPoweredChainDataConstraintInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPoweredChainDataConstraintInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPoweredChainDataConstraintInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPoweredChainDataConstraintInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPoweredChainDataConstraintInfoVisitor<'a>>> for HkpPoweredChainDataConstraintInfo<'a> {
    fn from(_values: Vec<HkpPoweredChainDataConstraintInfoVisitor<'a>>) -> Self {
            let mut pivot_in_a = None;
            let mut pivot_in_b = None;
            let mut a_tc = None;
            let mut b_tc = None;
            let mut motors = None;
            let mut switch_bodies = None;


        for _value in _values {
            match _value {
                HkpPoweredChainDataConstraintInfoVisitor::PivotInA(m) => pivot_in_a = Some(m),
                HkpPoweredChainDataConstraintInfoVisitor::PivotInB(m) => pivot_in_b = Some(m),
                HkpPoweredChainDataConstraintInfoVisitor::ATc(m) => a_tc = Some(m),
                HkpPoweredChainDataConstraintInfoVisitor::BTc(m) => b_tc = Some(m),
                HkpPoweredChainDataConstraintInfoVisitor::Motors(m) => motors = Some(m),
                HkpPoweredChainDataConstraintInfoVisitor::SwitchBodies(m) => switch_bodies = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            pivot_in_a: pivot_in_a.unwrap_or_default().into_inner(),
            pivot_in_b: pivot_in_b.unwrap_or_default().into_inner(),
            a_tc: a_tc.unwrap_or_default().into_inner(),
            b_tc: b_tc.unwrap_or_default().into_inner(),
            motors: motors.unwrap_or_default().into_inner(),
            switch_bodies: switch_bodies.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPoweredChainDataConstraintInfo<'a>> for Vec<HkpPoweredChainDataConstraintInfoVisitor<'a>> {
    fn from(data: &HkpPoweredChainDataConstraintInfo<'a>) -> Self {
        vec![
            HkpPoweredChainDataConstraintInfoVisitor::PivotInA(data.pivot_in_a.into()),
            HkpPoweredChainDataConstraintInfoVisitor::PivotInB(data.pivot_in_b.into()),
            HkpPoweredChainDataConstraintInfoVisitor::ATc(data.a_tc.clone().into()),
            HkpPoweredChainDataConstraintInfoVisitor::BTc(data.b_tc.clone().into()),
            HkpPoweredChainDataConstraintInfoVisitor::Motors(data.motors.clone().into()),
            HkpPoweredChainDataConstraintInfoVisitor::SwitchBodies(data.switch_bodies.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPoweredChainDataConstraintInfo<'de> {
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
enum HkpPoweredChainDataConstraintInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "pivotInA")]
    PivotInA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "pivotInB")]
    PivotInB(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "aTc")]
    ATc(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "bTc")]
    BTc(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "motors")]
    Motors(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "switchBodies")]
    SwitchBodies(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainDataConstraintInfoVisitor<'de>, "@name",
    ("pivotInA" => PivotInA(Primitive<Vector4<f32>>)),
    ("pivotInB" => PivotInB(Primitive<Vector4<f32>>)),
    ("aTc" => ATc(Primitive<Quaternion<f32>>)),
    ("bTc" => BTc(Primitive<Quaternion<f32>>)),
    ("motors" => Motors(Primitive<Cow<'de, str>>)),
    ("switchBodies" => SwitchBodies(Primitive<bool>)),
}
