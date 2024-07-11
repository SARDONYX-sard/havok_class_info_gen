//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpFirstPersonGun`
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

/// `hkpFirstPersonGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x852ab70b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpFirstPersonGun<'a> {
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
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"keyboardKey"`
    /// -   type: `enum KeyboardKey`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub keyboard_key: KeyboardKey,
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub listeners: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpFirstPersonGun<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpFirstPersonGunVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpFirstPersonGun<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpFirstPersonGunVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpFirstPersonGunVisitor<'a>>> for HkpFirstPersonGun<'a> {
    fn from(_values: Vec<HkpFirstPersonGunVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _type = None;
            let mut name = None;
            let mut keyboard_key = None;
            let mut listeners = None;


        for _value in _values {
            match _value {
                HkpFirstPersonGunVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpFirstPersonGunVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpFirstPersonGunVisitor::Type(m) => _type = Some(m),
                HkpFirstPersonGunVisitor::Name(m) => name = Some(m),
                HkpFirstPersonGunVisitor::KeyboardKey(m) => keyboard_key = Some(m),
                HkpFirstPersonGunVisitor::Listeners(m) => listeners = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            keyboard_key: keyboard_key.unwrap_or_default().into_inner(),
            listeners: listeners.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpFirstPersonGun<'a>> for Vec<HkpFirstPersonGunVisitor<'a>> {
    fn from(data: &HkpFirstPersonGun<'a>) -> Self {
        vec![
            HkpFirstPersonGunVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpFirstPersonGunVisitor::ReferenceCount(data.reference_count.into()),
            HkpFirstPersonGunVisitor::Type(data._type.into()),
            HkpFirstPersonGunVisitor::Name(data.name.clone().into()),
            HkpFirstPersonGunVisitor::KeyboardKey(data.keyboard_key.clone().into()),
            HkpFirstPersonGunVisitor::Listeners(data.listeners.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpFirstPersonGun<'de> {
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
enum HkpFirstPersonGunVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpFirstPersonGunVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Type {
    #[serde(rename = "WEAPON_TYPE_INVALID")]
    #[default]
    WeaponTypeInvalid = 0,
    #[serde(rename = "WEAPON_TYPE_BALLGUN")]
    WeaponTypeBallgun = 1,
    #[serde(rename = "WEAPON_TYPE_GRENADEGUN")]
    WeaponTypeGrenadegun = 2,
    #[serde(rename = "WEAPON_TYPE_GRAVITYGUN")]
    WeaponTypeGravitygun = 3,
    #[serde(rename = "WEAPON_TYPE_MOUNTEDBALLGUN")]
    WeaponTypeMountedballgun = 4,
    #[serde(rename = "WEAPON_TYPE_TWEAKERGUN")]
    WeaponTypeTweakergun = 5,
    #[serde(rename = "WEAPON_TYPE_MISSILEGUN")]
    WeaponTypeMissilegun = 6,
    #[serde(rename = "WEAPON_TYPE_RAYCASTGUN")]
    WeaponTypeRaycastgun = 7,
    #[serde(rename = "WEAPON_TYPE_SPHEREGUN")]
    WeaponTypeSpheregun = 8,
    #[serde(rename = "WEAPON_TYPE_STICKYGUN")]
    WeaponTypeStickygun = 9,
    #[serde(rename = "WEAPON_TYPE_NUM_TYPES")]
    WeaponTypeNumTypes = 10,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum KeyboardKey {
    #[serde(rename = "KEY_F1")]
    #[default]
    KeyF1 = 112,
    #[serde(rename = "KEY_F2")]
    KeyF2 = 113,
    #[serde(rename = "KEY_F3")]
    KeyF3 = 114,
    #[serde(rename = "KEY_F4")]
    KeyF4 = 115,
    #[serde(rename = "KEY_F5")]
    KeyF5 = 116,
    #[serde(rename = "KEY_F6")]
    KeyF6 = 117,
    #[serde(rename = "KEY_F7")]
    KeyF7 = 118,
    #[serde(rename = "KEY_F8")]
    KeyF8 = 119,
    #[serde(rename = "KEY_F9")]
    KeyF9 = 120,
    #[serde(rename = "KEY_F10")]
    KeyF10 = 121,
    #[serde(rename = "KEY_F11")]
    KeyF11 = 122,
    #[serde(rename = "KEY_F12")]
    KeyF12 = 123,
}
