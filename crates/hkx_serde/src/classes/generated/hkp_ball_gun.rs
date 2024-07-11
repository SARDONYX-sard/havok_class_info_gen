//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallGun`
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

/// `hkpBallGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0x57b06d35`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBallGun<'a> {
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"keyboardKey"`
    /// -   type: `enum KeyboardKey`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub keyboard_key: KeyboardKey,
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"listeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub listeners: HkArrayRef<Cow<'a, str>>,

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
    /// -   name:`"bulletRadius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bullet_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bulletVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub bullet_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bulletMass"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub bullet_mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub damage_multiplier: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxBulletsInWorld"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub max_bullets_in_world: i32,
    /// # C++ Class Fields Info
    /// -   name:`"bulletOffsetFromCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub bullet_offset_from_center: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"addedBodies"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub added_bodies: Cow<'a, str>,
}

impl Serialize for HkpBallGun<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBallGunVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBallGun<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBallGunVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpBallGunVisitor<'a>>> for HkpBallGun<'a> {
    fn from(_values: Vec<HkpBallGunVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut name = None;
            let mut keyboard_key = None;
            let mut listeners = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut bullet_radius = None;
            let mut bullet_velocity = None;
            let mut bullet_mass = None;
            let mut damage_multiplier = None;
            let mut max_bullets_in_world = None;
            let mut bullet_offset_from_center = None;
            let mut added_bodies = None;


        for _value in _values {
            match _value {
                HkpBallGunVisitor::Type(m) => _type = Some(m),
                HkpBallGunVisitor::Name(m) => name = Some(m),
                HkpBallGunVisitor::KeyboardKey(m) => keyboard_key = Some(m),
                HkpBallGunVisitor::Listeners(m) => listeners = Some(m),
                HkpBallGunVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpBallGunVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpBallGunVisitor::BulletRadius(m) => bullet_radius = Some(m),
                HkpBallGunVisitor::BulletVelocity(m) => bullet_velocity = Some(m),
                HkpBallGunVisitor::BulletMass(m) => bullet_mass = Some(m),
                HkpBallGunVisitor::DamageMultiplier(m) => damage_multiplier = Some(m),
                HkpBallGunVisitor::MaxBulletsInWorld(m) => max_bullets_in_world = Some(m),
                HkpBallGunVisitor::BulletOffsetFromCenter(m) => bullet_offset_from_center = Some(m),
                HkpBallGunVisitor::AddedBodies(m) => added_bodies = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            keyboard_key: keyboard_key.unwrap_or_default().into_inner(),
            listeners: listeners.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            bullet_radius: bullet_radius.unwrap_or_default().into_inner(),
            bullet_velocity: bullet_velocity.unwrap_or_default().into_inner(),
            bullet_mass: bullet_mass.unwrap_or_default().into_inner(),
            damage_multiplier: damage_multiplier.unwrap_or_default().into_inner(),
            max_bullets_in_world: max_bullets_in_world.unwrap_or_default().into_inner(),
            bullet_offset_from_center: bullet_offset_from_center.unwrap_or_default().into_inner(),
            added_bodies: added_bodies.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpBallGun<'a>> for Vec<HkpBallGunVisitor<'a>> {
    fn from(data: &HkpBallGun<'a>) -> Self {
        vec![
            HkpBallGunVisitor::Type(data._type.into()),
            HkpBallGunVisitor::Name(data.name.clone().into()),
            HkpBallGunVisitor::KeyboardKey(data.keyboard_key.clone().into()),
            HkpBallGunVisitor::Listeners(data.listeners.clone()),
            HkpBallGunVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpBallGunVisitor::ReferenceCount(data.reference_count.into()),
            HkpBallGunVisitor::BulletRadius(data.bullet_radius.into()),
            HkpBallGunVisitor::BulletVelocity(data.bullet_velocity.into()),
            HkpBallGunVisitor::BulletMass(data.bullet_mass.into()),
            HkpBallGunVisitor::DamageMultiplier(data.damage_multiplier.into()),
            HkpBallGunVisitor::MaxBulletsInWorld(data.max_bullets_in_world.into()),
            HkpBallGunVisitor::BulletOffsetFromCenter(data.bullet_offset_from_center.into()),
            HkpBallGunVisitor::AddedBodies(data.added_bodies.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBallGun<'de> {
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
enum HkpBallGunVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "keyboardKey")]
    KeyboardKey(Primitive<KeyboardKey>),
    /// Visitor fields
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkArrayRef<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "bulletRadius")]
    BulletRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bulletVelocity")]
    BulletVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bulletMass")]
    BulletMass(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxBulletsInWorld")]
    MaxBulletsInWorld(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "bulletOffsetFromCenter")]
    BulletOffsetFromCenter(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "addedBodies", skip_serializing)]
    AddedBodies(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallGunVisitor<'de>, "@name",
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bulletRadius" => BulletRadius(Primitive<f32>)),
    ("bulletVelocity" => BulletVelocity(Primitive<f32>)),
    ("bulletMass" => BulletMass(Primitive<f32>)),
    ("damageMultiplier" => DamageMultiplier(Primitive<f32>)),
    ("maxBulletsInWorld" => MaxBulletsInWorld(Primitive<i32>)),
    ("bulletOffsetFromCenter" => BulletOffsetFromCenter(Primitive<Vector4<f32>>)),
    ("addedBodies" => AddedBodies(Primitive<Cow<'de, str>>)),
}
