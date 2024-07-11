//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpProjectileGun`
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

/// `hkpProjectileGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0xb4f30148`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpProjectileGun<'a> {
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
    /// -   name:`"maxProjectiles"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub max_projectiles: i32,
    /// # C++ Class Fields Info
    /// -   name:`"reloadTime"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub reload_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"reload"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reload: f32,
    /// # C++ Class Fields Info
    /// -   name:`"projectiles"`
    /// -   type: `hkArray<void*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub projectiles: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub destruction_world: Cow<'a, str>,
}

impl Serialize for HkpProjectileGun<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpProjectileGunVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpProjectileGun<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpProjectileGunVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpProjectileGunVisitor<'a>>> for HkpProjectileGun<'a> {
    fn from(_values: Vec<HkpProjectileGunVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut name = None;
            let mut keyboard_key = None;
            let mut listeners = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut max_projectiles = None;
            let mut reload_time = None;
            let mut reload = None;
            let mut projectiles = None;
            let mut world = None;
            let mut destruction_world = None;


        for _value in _values {
            match _value {
                HkpProjectileGunVisitor::Type(m) => _type = Some(m),
                HkpProjectileGunVisitor::Name(m) => name = Some(m),
                HkpProjectileGunVisitor::KeyboardKey(m) => keyboard_key = Some(m),
                HkpProjectileGunVisitor::Listeners(m) => listeners = Some(m),
                HkpProjectileGunVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpProjectileGunVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpProjectileGunVisitor::MaxProjectiles(m) => max_projectiles = Some(m),
                HkpProjectileGunVisitor::ReloadTime(m) => reload_time = Some(m),
                HkpProjectileGunVisitor::Reload(m) => reload = Some(m),
                HkpProjectileGunVisitor::Projectiles(m) => projectiles = Some(m),
                HkpProjectileGunVisitor::World(m) => world = Some(m),
                HkpProjectileGunVisitor::DestructionWorld(m) => destruction_world = Some(m),

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
            max_projectiles: max_projectiles.unwrap_or_default().into_inner(),
            reload_time: reload_time.unwrap_or_default().into_inner(),
            reload: reload.unwrap_or_default().into_inner(),
            projectiles: projectiles.unwrap_or_default(),
            world: world.unwrap_or_default().into_inner(),
            destruction_world: destruction_world.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpProjectileGun<'a>> for Vec<HkpProjectileGunVisitor<'a>> {
    fn from(data: &HkpProjectileGun<'a>) -> Self {
        vec![
            HkpProjectileGunVisitor::Type(data._type.into()),
            HkpProjectileGunVisitor::Name(data.name.clone().into()),
            HkpProjectileGunVisitor::KeyboardKey(data.keyboard_key.clone().into()),
            HkpProjectileGunVisitor::Listeners(data.listeners.clone()),
            HkpProjectileGunVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpProjectileGunVisitor::ReferenceCount(data.reference_count.into()),
            HkpProjectileGunVisitor::MaxProjectiles(data.max_projectiles.into()),
            HkpProjectileGunVisitor::ReloadTime(data.reload_time.into()),
            HkpProjectileGunVisitor::Reload(data.reload.into()),
            HkpProjectileGunVisitor::Projectiles(data.projectiles.clone()),
            HkpProjectileGunVisitor::World(data.world.clone().into()),
            HkpProjectileGunVisitor::DestructionWorld(data.destruction_world.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpProjectileGun<'de> {
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
enum HkpProjectileGunVisitor<'a> {
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
    #[serde(rename = "maxProjectiles")]
    MaxProjectiles(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "reloadTime")]
    ReloadTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "reload", skip_serializing)]
    Reload(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "projectiles", skip_serializing)]
    Projectiles(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpProjectileGunVisitor<'de>, "@name",
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("maxProjectiles" => MaxProjectiles(Primitive<i32>)),
    ("reloadTime" => ReloadTime(Primitive<f32>)),
    ("reload" => Reload(Primitive<f32>)),
    ("projectiles" => Projectiles(HkArrayRef<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("destructionWorld" => DestructionWorld(Primitive<Cow<'de, str>>)),
}
