//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkDriverInfo`
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

/// `hkbFootIkDriverInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc6a09dbf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkDriverInfo {
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
    /// -   name:`"legs"`
    /// -   type: `hkArray<struct hkbFootIkDriverInfoLeg>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub legs: HkArrayClass<HkbFootIkDriverInfoLeg>,
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub raycast_distance_up: f32,
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub raycast_distance_down: f32,
    /// # C++ Class Fields Info
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub original_ground_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub vertical_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub forward_align_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub sideways_align_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub sideways_sample_width: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub lock_feet_when_planted: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    pub use_character_up_vector: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isQuadrupedNarrow"`
    /// -   type: `hkBool`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    pub is_quadruped_narrow: bool,
}

impl Serialize for HkbFootIkDriverInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkDriverInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkDriverInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkDriverInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbFootIkDriverInfoVisitor>> for HkbFootIkDriverInfo {
    fn from(_values: Vec<HkbFootIkDriverInfoVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut legs = None;
            let mut raycast_distance_up = None;
            let mut raycast_distance_down = None;
            let mut original_ground_height_ms = None;
            let mut vertical_offset = None;
            let mut collision_filter_info = None;
            let mut forward_align_fraction = None;
            let mut sideways_align_fraction = None;
            let mut sideways_sample_width = None;
            let mut lock_feet_when_planted = None;
            let mut use_character_up_vector = None;
            let mut is_quadruped_narrow = None;


        for _value in _values {
            match _value {
                HkbFootIkDriverInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbFootIkDriverInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbFootIkDriverInfoVisitor::Legs(m) => legs = Some(m),
                HkbFootIkDriverInfoVisitor::RaycastDistanceUp(m) => raycast_distance_up = Some(m),
                HkbFootIkDriverInfoVisitor::RaycastDistanceDown(m) => raycast_distance_down = Some(m),
                HkbFootIkDriverInfoVisitor::OriginalGroundHeightMs(m) => original_ground_height_ms = Some(m),
                HkbFootIkDriverInfoVisitor::VerticalOffset(m) => vertical_offset = Some(m),
                HkbFootIkDriverInfoVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbFootIkDriverInfoVisitor::ForwardAlignFraction(m) => forward_align_fraction = Some(m),
                HkbFootIkDriverInfoVisitor::SidewaysAlignFraction(m) => sideways_align_fraction = Some(m),
                HkbFootIkDriverInfoVisitor::SidewaysSampleWidth(m) => sideways_sample_width = Some(m),
                HkbFootIkDriverInfoVisitor::LockFeetWhenPlanted(m) => lock_feet_when_planted = Some(m),
                HkbFootIkDriverInfoVisitor::UseCharacterUpVector(m) => use_character_up_vector = Some(m),
                HkbFootIkDriverInfoVisitor::IsQuadrupedNarrow(m) => is_quadruped_narrow = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            legs: legs.unwrap_or_default(),
            raycast_distance_up: raycast_distance_up.unwrap_or_default().into_inner(),
            raycast_distance_down: raycast_distance_down.unwrap_or_default().into_inner(),
            original_ground_height_ms: original_ground_height_ms.unwrap_or_default().into_inner(),
            vertical_offset: vertical_offset.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            forward_align_fraction: forward_align_fraction.unwrap_or_default().into_inner(),
            sideways_align_fraction: sideways_align_fraction.unwrap_or_default().into_inner(),
            sideways_sample_width: sideways_sample_width.unwrap_or_default().into_inner(),
            lock_feet_when_planted: lock_feet_when_planted.unwrap_or_default().into_inner(),
            use_character_up_vector: use_character_up_vector.unwrap_or_default().into_inner(),
            is_quadruped_narrow: is_quadruped_narrow.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbFootIkDriverInfo> for Vec<HkbFootIkDriverInfoVisitor> {
    fn from(data: &HkbFootIkDriverInfo) -> Self {
        vec![
            HkbFootIkDriverInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbFootIkDriverInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbFootIkDriverInfoVisitor::Legs(data.legs.clone()),
            HkbFootIkDriverInfoVisitor::RaycastDistanceUp(data.raycast_distance_up.into()),
            HkbFootIkDriverInfoVisitor::RaycastDistanceDown(data.raycast_distance_down.into()),
            HkbFootIkDriverInfoVisitor::OriginalGroundHeightMs(data.original_ground_height_ms.into()),
            HkbFootIkDriverInfoVisitor::VerticalOffset(data.vertical_offset.into()),
            HkbFootIkDriverInfoVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbFootIkDriverInfoVisitor::ForwardAlignFraction(data.forward_align_fraction.into()),
            HkbFootIkDriverInfoVisitor::SidewaysAlignFraction(data.sideways_align_fraction.into()),
            HkbFootIkDriverInfoVisitor::SidewaysSampleWidth(data.sideways_sample_width.into()),
            HkbFootIkDriverInfoVisitor::LockFeetWhenPlanted(data.lock_feet_when_planted.into()),
            HkbFootIkDriverInfoVisitor::UseCharacterUpVector(data.use_character_up_vector.into()),
            HkbFootIkDriverInfoVisitor::IsQuadrupedNarrow(data.is_quadruped_narrow.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkDriverInfo {
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
enum HkbFootIkDriverInfoVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "legs")]
    Legs(HkArrayClass<HkbFootIkDriverInfoLeg>),
    /// Visitor fields
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isQuadrupedNarrow")]
    IsQuadrupedNarrow(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkDriverInfoVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("legs" => Legs(HkArrayClass<HkbFootIkDriverInfoLeg>)),
    ("raycastDistanceUp" => RaycastDistanceUp(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(Primitive<f32>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("forwardAlignFraction" => ForwardAlignFraction(Primitive<f32>)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(Primitive<f32>)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(Primitive<f32>)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(Primitive<bool>)),
    ("useCharacterUpVector" => UseCharacterUpVector(Primitive<bool>)),
    ("isQuadrupedNarrow" => IsQuadrupedNarrow(Primitive<bool>)),
}
