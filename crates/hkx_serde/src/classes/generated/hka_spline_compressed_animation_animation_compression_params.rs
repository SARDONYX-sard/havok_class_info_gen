//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimationAnimationCompressionParams`
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

/// `hkaSplineCompressedAnimationAnimationCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0xde830789`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSplineCompressedAnimationAnimationCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"maxFramesPerBlock"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub max_frames_per_block: u16,
    /// # C++ Class Fields Info
    /// -   name:`"enableSampleSingleTracks"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub enable_sample_single_tracks: bool,
}

impl Serialize for HkaSplineCompressedAnimationAnimationCompressionParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSplineCompressedAnimationAnimationCompressionParamsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSplineCompressedAnimationAnimationCompressionParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSplineCompressedAnimationAnimationCompressionParamsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaSplineCompressedAnimationAnimationCompressionParamsVisitor>> for HkaSplineCompressedAnimationAnimationCompressionParams {
    fn from(_values: Vec<HkaSplineCompressedAnimationAnimationCompressionParamsVisitor>) -> Self {
            let mut max_frames_per_block = None;
            let mut enable_sample_single_tracks = None;


        for _value in _values {
            match _value {
                HkaSplineCompressedAnimationAnimationCompressionParamsVisitor::MaxFramesPerBlock(m) => max_frames_per_block = Some(m),
                HkaSplineCompressedAnimationAnimationCompressionParamsVisitor::EnableSampleSingleTracks(m) => enable_sample_single_tracks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            max_frames_per_block: max_frames_per_block.unwrap_or_default().into_inner(),
            enable_sample_single_tracks: enable_sample_single_tracks.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaSplineCompressedAnimationAnimationCompressionParams> for Vec<HkaSplineCompressedAnimationAnimationCompressionParamsVisitor> {
    fn from(data: &HkaSplineCompressedAnimationAnimationCompressionParams) -> Self {
        vec![
            HkaSplineCompressedAnimationAnimationCompressionParamsVisitor::MaxFramesPerBlock(data.max_frames_per_block.into()),
            HkaSplineCompressedAnimationAnimationCompressionParamsVisitor::EnableSampleSingleTracks(data.enable_sample_single_tracks.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSplineCompressedAnimationAnimationCompressionParams {
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
enum HkaSplineCompressedAnimationAnimationCompressionParamsVisitor {
    /// Visitor fields
    #[serde(rename = "maxFramesPerBlock")]
    MaxFramesPerBlock(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "enableSampleSingleTracks")]
    EnableSampleSingleTracks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationAnimationCompressionParamsVisitor, "@name",
    ("maxFramesPerBlock" => MaxFramesPerBlock(Primitive<u16>)),
    ("enableSampleSingleTracks" => EnableSampleSingleTracks(Primitive<bool>)),
}
