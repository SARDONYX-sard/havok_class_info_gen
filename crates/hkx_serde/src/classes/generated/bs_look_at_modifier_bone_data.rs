//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSLookAtModifierBoneData`
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

/// `BSLookAtModifierBoneData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x29efee59`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsLookAtModifierBoneData {
    /// # C++ Class Fields Info
    /// -   name:`"index"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"fwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub fwd_axis_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub on_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub off_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enabled"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub enabled: bool,
    /// # C++ Class Fields Info
    /// -   name:`"currentFwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub current_fwd_axis_ls: Vector4<f32>,
}

impl Serialize for BsLookAtModifierBoneData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsLookAtModifierBoneDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsLookAtModifierBoneData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsLookAtModifierBoneDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<BsLookAtModifierBoneDataVisitor>> for BsLookAtModifierBoneData {
    fn from(_values: Vec<BsLookAtModifierBoneDataVisitor>) -> Self {
            let mut index = None;
            let mut fwd_axis_ls = None;
            let mut limit_angle_degrees = None;
            let mut on_gain = None;
            let mut off_gain = None;
            let mut enabled = None;
            let mut current_fwd_axis_ls = None;


        for _value in _values {
            match _value {
                BsLookAtModifierBoneDataVisitor::Index(m) => index = Some(m),
                BsLookAtModifierBoneDataVisitor::FwdAxisLs(m) => fwd_axis_ls = Some(m),
                BsLookAtModifierBoneDataVisitor::LimitAngleDegrees(m) => limit_angle_degrees = Some(m),
                BsLookAtModifierBoneDataVisitor::OnGain(m) => on_gain = Some(m),
                BsLookAtModifierBoneDataVisitor::OffGain(m) => off_gain = Some(m),
                BsLookAtModifierBoneDataVisitor::Enabled(m) => enabled = Some(m),
                BsLookAtModifierBoneDataVisitor::CurrentFwdAxisLs(m) => current_fwd_axis_ls = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            index: index.unwrap_or_default().into_inner(),
            fwd_axis_ls: fwd_axis_ls.unwrap_or_default().into_inner(),
            limit_angle_degrees: limit_angle_degrees.unwrap_or_default().into_inner(),
            on_gain: on_gain.unwrap_or_default().into_inner(),
            off_gain: off_gain.unwrap_or_default().into_inner(),
            enabled: enabled.unwrap_or_default().into_inner(),
            current_fwd_axis_ls: current_fwd_axis_ls.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&BsLookAtModifierBoneData> for Vec<BsLookAtModifierBoneDataVisitor> {
    fn from(data: &BsLookAtModifierBoneData) -> Self {
        vec![
            BsLookAtModifierBoneDataVisitor::Index(data.index.into()),
            BsLookAtModifierBoneDataVisitor::FwdAxisLs(data.fwd_axis_ls.into()),
            BsLookAtModifierBoneDataVisitor::LimitAngleDegrees(data.limit_angle_degrees.into()),
            BsLookAtModifierBoneDataVisitor::OnGain(data.on_gain.into()),
            BsLookAtModifierBoneDataVisitor::OffGain(data.off_gain.into()),
            BsLookAtModifierBoneDataVisitor::Enabled(data.enabled.into()),
            BsLookAtModifierBoneDataVisitor::CurrentFwdAxisLs(data.current_fwd_axis_ls.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsLookAtModifierBoneData {
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
enum BsLookAtModifierBoneDataVisitor {
    /// Visitor fields
    #[serde(rename = "index")]
    Index(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "fwdAxisLS")]
    FwdAxisLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "enabled")]
    Enabled(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "currentFwdAxisLS", skip_serializing)]
    CurrentFwdAxisLs(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifierBoneDataVisitor, "@name",
    ("index" => Index(Primitive<i16>)),
    ("fwdAxisLS" => FwdAxisLs(Primitive<Vector4<f32>>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("enabled" => Enabled(Primitive<bool>)),
    ("currentFwdAxisLS" => CurrentFwdAxisLs(Primitive<Vector4<f32>>)),
}
