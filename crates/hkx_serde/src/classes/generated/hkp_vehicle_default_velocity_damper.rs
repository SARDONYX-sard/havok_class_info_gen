//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultVelocityDamper`
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

/// `hkpVehicleDefaultVelocityDamper`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpVehicleVelocityDamper`/`0xda8c7d7d`
/// - signature: `0x741b8d9e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultVelocityDamper {
    // C++ Parent class(`hkpVehicleVelocityDamper` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"normalSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub normal_spin_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub collision_spin_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionThreshold"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub collision_threshold: f32,
}

impl Serialize for HkpVehicleDefaultVelocityDamper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultVelocityDamperVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultVelocityDamper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultVelocityDamperVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultVelocityDamperVisitor>> for HkpVehicleDefaultVelocityDamper {
    fn from(_values: Vec<HkpVehicleDefaultVelocityDamperVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut normal_spin_damping = None;
            let mut collision_spin_damping = None;
            let mut collision_threshold = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultVelocityDamperVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultVelocityDamperVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultVelocityDamperVisitor::NormalSpinDamping(m) => normal_spin_damping = Some(m),
                HkpVehicleDefaultVelocityDamperVisitor::CollisionSpinDamping(m) => collision_spin_damping = Some(m),
                HkpVehicleDefaultVelocityDamperVisitor::CollisionThreshold(m) => collision_threshold = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            normal_spin_damping: normal_spin_damping.unwrap_or_default().into_inner(),
            collision_spin_damping: collision_spin_damping.unwrap_or_default().into_inner(),
            collision_threshold: collision_threshold.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultVelocityDamper> for Vec<HkpVehicleDefaultVelocityDamperVisitor> {
    fn from(data: &HkpVehicleDefaultVelocityDamper) -> Self {
        vec![
            HkpVehicleDefaultVelocityDamperVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultVelocityDamperVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultVelocityDamperVisitor::NormalSpinDamping(data.normal_spin_damping.into()),
            HkpVehicleDefaultVelocityDamperVisitor::CollisionSpinDamping(data.collision_spin_damping.into()),
            HkpVehicleDefaultVelocityDamperVisitor::CollisionThreshold(data.collision_threshold.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultVelocityDamper {
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
enum HkpVehicleDefaultVelocityDamperVisitor {
    // C++ Parent class(`hkpVehicleVelocityDamper` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "normalSpinDamping")]
    NormalSpinDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionSpinDamping")]
    CollisionSpinDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionThreshold")]
    CollisionThreshold(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultVelocityDamperVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("normalSpinDamping" => NormalSpinDamping(Primitive<f32>)),
    ("collisionSpinDamping" => CollisionSpinDamping(Primitive<f32>)),
    ("collisionThreshold" => CollisionThreshold(Primitive<f32>)),
}
