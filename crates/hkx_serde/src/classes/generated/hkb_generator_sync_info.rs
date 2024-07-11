//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorSyncInfo`
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

/// `hkbGeneratorSyncInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xa3c341f8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbGeneratorSyncInfo {
    /// # C++ Class Fields Info
    /// -   name:`"syncPoints"`
    /// -   type: `struct hkbGeneratorSyncInfoSyncPoint[8]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub sync_points: CStyleArrayClass<HkbGeneratorSyncInfoSyncPoint, 8>,
    /// # C++ Class Fields Info
    /// -   name:`"baseFrequency"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub base_frequency: f32,
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub playback_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numSyncPoints"`
    /// -   type: `hkInt8`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub num_sync_points: i8,
    /// # C++ Class Fields Info
    /// -   name:`"isCyclic"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    pub is_cyclic: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isMirrored"`
    /// -   type: `hkBool`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    pub is_mirrored: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 79
    /// -  flags: `FLAGS_NONE`
    pub is_additive: bool,
}

impl Serialize for HkbGeneratorSyncInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbGeneratorSyncInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbGeneratorSyncInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbGeneratorSyncInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbGeneratorSyncInfoVisitor>> for HkbGeneratorSyncInfo {
    fn from(_values: Vec<HkbGeneratorSyncInfoVisitor>) -> Self {
            let mut sync_points = None;
            let mut base_frequency = None;
            let mut local_time = None;
            let mut playback_speed = None;
            let mut num_sync_points = None;
            let mut is_cyclic = None;
            let mut is_mirrored = None;
            let mut is_additive = None;


        for _value in _values {
            match _value {
                HkbGeneratorSyncInfoVisitor::SyncPoints(m) => sync_points = Some(m),
                HkbGeneratorSyncInfoVisitor::BaseFrequency(m) => base_frequency = Some(m),
                HkbGeneratorSyncInfoVisitor::LocalTime(m) => local_time = Some(m),
                HkbGeneratorSyncInfoVisitor::PlaybackSpeed(m) => playback_speed = Some(m),
                HkbGeneratorSyncInfoVisitor::NumSyncPoints(m) => num_sync_points = Some(m),
                HkbGeneratorSyncInfoVisitor::IsCyclic(m) => is_cyclic = Some(m),
                HkbGeneratorSyncInfoVisitor::IsMirrored(m) => is_mirrored = Some(m),
                HkbGeneratorSyncInfoVisitor::IsAdditive(m) => is_additive = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            sync_points: sync_points.unwrap_or_default(),
            base_frequency: base_frequency.unwrap_or_default().into_inner(),
            local_time: local_time.unwrap_or_default().into_inner(),
            playback_speed: playback_speed.unwrap_or_default().into_inner(),
            num_sync_points: num_sync_points.unwrap_or_default().into_inner(),
            is_cyclic: is_cyclic.unwrap_or_default().into_inner(),
            is_mirrored: is_mirrored.unwrap_or_default().into_inner(),
            is_additive: is_additive.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbGeneratorSyncInfo> for Vec<HkbGeneratorSyncInfoVisitor> {
    fn from(data: &HkbGeneratorSyncInfo) -> Self {
        vec![
            HkbGeneratorSyncInfoVisitor::SyncPoints(data.sync_points.clone()),
            HkbGeneratorSyncInfoVisitor::BaseFrequency(data.base_frequency.into()),
            HkbGeneratorSyncInfoVisitor::LocalTime(data.local_time.into()),
            HkbGeneratorSyncInfoVisitor::PlaybackSpeed(data.playback_speed.into()),
            HkbGeneratorSyncInfoVisitor::NumSyncPoints(data.num_sync_points.into()),
            HkbGeneratorSyncInfoVisitor::IsCyclic(data.is_cyclic.into()),
            HkbGeneratorSyncInfoVisitor::IsMirrored(data.is_mirrored.into()),
            HkbGeneratorSyncInfoVisitor::IsAdditive(data.is_additive.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbGeneratorSyncInfo {
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
enum HkbGeneratorSyncInfoVisitor {
    /// Visitor fields
    #[serde(rename = "syncPoints")]
    SyncPoints(CStyleArrayClass<HkbGeneratorSyncInfoSyncPoint, 8>),
    /// Visitor fields
    #[serde(rename = "baseFrequency")]
    BaseFrequency(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numSyncPoints")]
    NumSyncPoints(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "isCyclic")]
    IsCyclic(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isMirrored")]
    IsMirrored(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isAdditive")]
    IsAdditive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorSyncInfoVisitor, "@name",
    ("syncPoints" => SyncPoints(CStyleArrayClass<HkbGeneratorSyncInfoSyncPoint, 8>)),
    ("baseFrequency" => BaseFrequency(Primitive<f32>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("playbackSpeed" => PlaybackSpeed(Primitive<f32>)),
    ("numSyncPoints" => NumSyncPoints(Primitive<i8>)),
    ("isCyclic" => IsCyclic(Primitive<bool>)),
    ("isMirrored" => IsMirrored(Primitive<bool>)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
}
