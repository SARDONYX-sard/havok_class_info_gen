//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGravityGun`
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

/// `hkpGravityGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0x5e2754cd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpGravityGun<'a> {
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
    /// -   name:`"grabbedBodies"`
    /// -   type: `hkArray<void*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub grabbed_bodies: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"maxNumObjectsPicked"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub max_num_objects_picked: i32,
    /// # C++ Class Fields Info
    /// -   name:`"maxMassOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub max_mass_of_object_picked: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxDistOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub max_dist_of_object_picked: f32,
    /// # C++ Class Fields Info
    /// -   name:`"impulseAppliedWhenObjectNotPicked"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub impulse_applied_when_object_not_picked: f32,
    /// # C++ Class Fields Info
    /// -   name:`"throwVelocity"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub throw_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"capturedObjectPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub captured_object_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"capturedObjectsOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub captured_objects_offset: Vector4<f32>,
}

impl Serialize for HkpGravityGun<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpGravityGunVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpGravityGun<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpGravityGunVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpGravityGunVisitor<'a>>> for HkpGravityGun<'a> {
    fn from(_values: Vec<HkpGravityGunVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut name = None;
            let mut keyboard_key = None;
            let mut listeners = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut grabbed_bodies = None;
            let mut max_num_objects_picked = None;
            let mut max_mass_of_object_picked = None;
            let mut max_dist_of_object_picked = None;
            let mut impulse_applied_when_object_not_picked = None;
            let mut throw_velocity = None;
            let mut captured_object_position = None;
            let mut captured_objects_offset = None;


        for _value in _values {
            match _value {
                HkpGravityGunVisitor::Type(m) => _type = Some(m),
                HkpGravityGunVisitor::Name(m) => name = Some(m),
                HkpGravityGunVisitor::KeyboardKey(m) => keyboard_key = Some(m),
                HkpGravityGunVisitor::Listeners(m) => listeners = Some(m),
                HkpGravityGunVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpGravityGunVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpGravityGunVisitor::GrabbedBodies(m) => grabbed_bodies = Some(m),
                HkpGravityGunVisitor::MaxNumObjectsPicked(m) => max_num_objects_picked = Some(m),
                HkpGravityGunVisitor::MaxMassOfObjectPicked(m) => max_mass_of_object_picked = Some(m),
                HkpGravityGunVisitor::MaxDistOfObjectPicked(m) => max_dist_of_object_picked = Some(m),
                HkpGravityGunVisitor::ImpulseAppliedWhenObjectNotPicked(m) => impulse_applied_when_object_not_picked = Some(m),
                HkpGravityGunVisitor::ThrowVelocity(m) => throw_velocity = Some(m),
                HkpGravityGunVisitor::CapturedObjectPosition(m) => captured_object_position = Some(m),
                HkpGravityGunVisitor::CapturedObjectsOffset(m) => captured_objects_offset = Some(m),

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
            grabbed_bodies: grabbed_bodies.unwrap_or_default(),
            max_num_objects_picked: max_num_objects_picked.unwrap_or_default().into_inner(),
            max_mass_of_object_picked: max_mass_of_object_picked.unwrap_or_default().into_inner(),
            max_dist_of_object_picked: max_dist_of_object_picked.unwrap_or_default().into_inner(),
            impulse_applied_when_object_not_picked: impulse_applied_when_object_not_picked.unwrap_or_default().into_inner(),
            throw_velocity: throw_velocity.unwrap_or_default().into_inner(),
            captured_object_position: captured_object_position.unwrap_or_default().into_inner(),
            captured_objects_offset: captured_objects_offset.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpGravityGun<'a>> for Vec<HkpGravityGunVisitor<'a>> {
    fn from(data: &HkpGravityGun<'a>) -> Self {
        vec![
            HkpGravityGunVisitor::Type(data._type.into()),
            HkpGravityGunVisitor::Name(data.name.clone().into()),
            HkpGravityGunVisitor::KeyboardKey(data.keyboard_key.clone().into()),
            HkpGravityGunVisitor::Listeners(data.listeners.clone()),
            HkpGravityGunVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpGravityGunVisitor::ReferenceCount(data.reference_count.into()),
            HkpGravityGunVisitor::GrabbedBodies(data.grabbed_bodies.clone()),
            HkpGravityGunVisitor::MaxNumObjectsPicked(data.max_num_objects_picked.into()),
            HkpGravityGunVisitor::MaxMassOfObjectPicked(data.max_mass_of_object_picked.into()),
            HkpGravityGunVisitor::MaxDistOfObjectPicked(data.max_dist_of_object_picked.into()),
            HkpGravityGunVisitor::ImpulseAppliedWhenObjectNotPicked(data.impulse_applied_when_object_not_picked.into()),
            HkpGravityGunVisitor::ThrowVelocity(data.throw_velocity.into()),
            HkpGravityGunVisitor::CapturedObjectPosition(data.captured_object_position.into()),
            HkpGravityGunVisitor::CapturedObjectsOffset(data.captured_objects_offset.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpGravityGun<'de> {
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
enum HkpGravityGunVisitor<'a> {
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
    #[serde(rename = "grabbedBodies", skip_serializing)]
    GrabbedBodies(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "maxNumObjectsPicked")]
    MaxNumObjectsPicked(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "maxMassOfObjectPicked")]
    MaxMassOfObjectPicked(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxDistOfObjectPicked")]
    MaxDistOfObjectPicked(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "impulseAppliedWhenObjectNotPicked")]
    ImpulseAppliedWhenObjectNotPicked(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "throwVelocity")]
    ThrowVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "capturedObjectPosition")]
    CapturedObjectPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "capturedObjectsOffset")]
    CapturedObjectsOffset(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGravityGunVisitor<'de>, "@name",
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("grabbedBodies" => GrabbedBodies(HkArrayRef<Cow<'de, str>>)),
    ("maxNumObjectsPicked" => MaxNumObjectsPicked(Primitive<i32>)),
    ("maxMassOfObjectPicked" => MaxMassOfObjectPicked(Primitive<f32>)),
    ("maxDistOfObjectPicked" => MaxDistOfObjectPicked(Primitive<f32>)),
    ("impulseAppliedWhenObjectNotPicked" => ImpulseAppliedWhenObjectNotPicked(Primitive<f32>)),
    ("throwVelocity" => ThrowVelocity(Primitive<f32>)),
    ("capturedObjectPosition" => CapturedObjectPosition(Primitive<Vector4<f32>>)),
    ("capturedObjectsOffset" => CapturedObjectsOffset(Primitive<Vector4<f32>>)),
}
