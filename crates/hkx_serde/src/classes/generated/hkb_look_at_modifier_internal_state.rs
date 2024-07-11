//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbLookAtModifierInternalState`
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

/// `hkbLookAtModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa14caba6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbLookAtModifierInternalState {
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
    /// -   name:`"lookAtLastTargetWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub look_at_last_target_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtWeight"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub look_at_weight: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isTargetInsideLimitCone"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub is_target_inside_limit_cone: bool,
}

impl Serialize for HkbLookAtModifierInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbLookAtModifierInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbLookAtModifierInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbLookAtModifierInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbLookAtModifierInternalStateVisitor>> for HkbLookAtModifierInternalState {
    fn from(_values: Vec<HkbLookAtModifierInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut look_at_last_target_ws = None;
            let mut look_at_weight = None;
            let mut is_target_inside_limit_cone = None;


        for _value in _values {
            match _value {
                HkbLookAtModifierInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbLookAtModifierInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbLookAtModifierInternalStateVisitor::LookAtLastTargetWs(m) => look_at_last_target_ws = Some(m),
                HkbLookAtModifierInternalStateVisitor::LookAtWeight(m) => look_at_weight = Some(m),
                HkbLookAtModifierInternalStateVisitor::IsTargetInsideLimitCone(m) => is_target_inside_limit_cone = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            look_at_last_target_ws: look_at_last_target_ws.unwrap_or_default().into_inner(),
            look_at_weight: look_at_weight.unwrap_or_default().into_inner(),
            is_target_inside_limit_cone: is_target_inside_limit_cone.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbLookAtModifierInternalState> for Vec<HkbLookAtModifierInternalStateVisitor> {
    fn from(data: &HkbLookAtModifierInternalState) -> Self {
        vec![
            HkbLookAtModifierInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbLookAtModifierInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbLookAtModifierInternalStateVisitor::LookAtLastTargetWs(data.look_at_last_target_ws.into()),
            HkbLookAtModifierInternalStateVisitor::LookAtWeight(data.look_at_weight.into()),
            HkbLookAtModifierInternalStateVisitor::IsTargetInsideLimitCone(data.is_target_inside_limit_cone.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbLookAtModifierInternalState {
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
enum HkbLookAtModifierInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "lookAtLastTargetWS")]
    LookAtLastTargetWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "lookAtWeight")]
    LookAtWeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isTargetInsideLimitCone")]
    IsTargetInsideLimitCone(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbLookAtModifierInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("lookAtLastTargetWS" => LookAtLastTargetWs(Primitive<Vector4<f32>>)),
    ("lookAtWeight" => LookAtWeight(Primitive<f32>)),
    ("isTargetInsideLimitCone" => IsTargetInsideLimitCone(Primitive<bool>)),
}
