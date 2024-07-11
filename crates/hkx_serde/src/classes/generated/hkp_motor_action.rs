//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMotorAction`
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

/// `hkpMotorAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x8ff131d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMotorAction<'a> {
    /// # C++ Parent class(`hkpUnaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entity"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entity: Cow<'a, str>,

    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,

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
    /// -   name:`"axis"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub axis: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"spinRate"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub spin_rate: f32,
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub active: bool,
}

impl Serialize for HkpMotorAction<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMotorActionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMotorAction<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMotorActionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpMotorActionVisitor<'a>>> for HkpMotorAction<'a> {
    fn from(_values: Vec<HkpMotorActionVisitor<'a>>) -> Self {
            let mut entity = None;
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut axis = None;
            let mut spin_rate = None;
            let mut gain = None;
            let mut active = None;


        for _value in _values {
            match _value {
                HkpMotorActionVisitor::Entity(m) => entity = Some(m),
                HkpMotorActionVisitor::World(m) => world = Some(m),
                HkpMotorActionVisitor::Island(m) => island = Some(m),
                HkpMotorActionVisitor::UserData(m) => user_data = Some(m),
                HkpMotorActionVisitor::Name(m) => name = Some(m),
                HkpMotorActionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpMotorActionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpMotorActionVisitor::Axis(m) => axis = Some(m),
                HkpMotorActionVisitor::SpinRate(m) => spin_rate = Some(m),
                HkpMotorActionVisitor::Gain(m) => gain = Some(m),
                HkpMotorActionVisitor::Active(m) => active = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            entity: entity.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            axis: axis.unwrap_or_default().into_inner(),
            spin_rate: spin_rate.unwrap_or_default().into_inner(),
            gain: gain.unwrap_or_default().into_inner(),
            active: active.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpMotorAction<'a>> for Vec<HkpMotorActionVisitor<'a>> {
    fn from(data: &HkpMotorAction<'a>) -> Self {
        vec![
            HkpMotorActionVisitor::Entity(data.entity.clone().into()),
            HkpMotorActionVisitor::World(data.world.clone().into()),
            HkpMotorActionVisitor::Island(data.island.clone().into()),
            HkpMotorActionVisitor::UserData(data.user_data.into()),
            HkpMotorActionVisitor::Name(data.name.clone().into()),
            HkpMotorActionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpMotorActionVisitor::ReferenceCount(data.reference_count.into()),
            HkpMotorActionVisitor::Axis(data.axis.into()),
            HkpMotorActionVisitor::SpinRate(data.spin_rate.into()),
            HkpMotorActionVisitor::Gain(data.gain.into()),
            HkpMotorActionVisitor::Active(data.active.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMotorAction<'de> {
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
enum HkpMotorActionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "entity")]
    Entity(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "axis")]
    Axis(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "spinRate")]
    SpinRate(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "active")]
    Active(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMotorActionVisitor<'de>, "@name",
    ("entity" => Entity(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("axis" => Axis(Primitive<Vector4<f32>>)),
    ("spinRate" => SpinRate(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("active" => Active(Primitive<bool>)),
}
