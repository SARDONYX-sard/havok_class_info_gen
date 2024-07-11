//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkControlData`
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

/// `hkbHandIkControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xd72b8d17`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbHandIkControlData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub target_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub target_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetNormal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub target_normal: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub target_handle: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transformOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub transform_on_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"normalOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub normal_on_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fadeInDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub fade_in_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fadeOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub fade_out_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub extrapolation_time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub handle_change_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub handle_change_mode: HandleChangeMode,
    /// # C++ Class Fields Info
    /// -   name:`"fixUp"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    pub fix_up: bool,
}

impl Serialize for HkbHandIkControlData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbHandIkControlDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbHandIkControlData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbHandIkControlDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbHandIkControlDataVisitor<'a>>> for HkbHandIkControlData<'a> {
    fn from(_values: Vec<HkbHandIkControlDataVisitor<'a>>) -> Self {
            let mut target_position = None;
            let mut target_rotation = None;
            let mut target_normal = None;
            let mut target_handle = None;
            let mut transform_on_fraction = None;
            let mut normal_on_fraction = None;
            let mut fade_in_duration = None;
            let mut fade_out_duration = None;
            let mut extrapolation_time_step = None;
            let mut handle_change_speed = None;
            let mut handle_change_mode = None;
            let mut fix_up = None;


        for _value in _values {
            match _value {
                HkbHandIkControlDataVisitor::TargetPosition(m) => target_position = Some(m),
                HkbHandIkControlDataVisitor::TargetRotation(m) => target_rotation = Some(m),
                HkbHandIkControlDataVisitor::TargetNormal(m) => target_normal = Some(m),
                HkbHandIkControlDataVisitor::TargetHandle(m) => target_handle = Some(m),
                HkbHandIkControlDataVisitor::TransformOnFraction(m) => transform_on_fraction = Some(m),
                HkbHandIkControlDataVisitor::NormalOnFraction(m) => normal_on_fraction = Some(m),
                HkbHandIkControlDataVisitor::FadeInDuration(m) => fade_in_duration = Some(m),
                HkbHandIkControlDataVisitor::FadeOutDuration(m) => fade_out_duration = Some(m),
                HkbHandIkControlDataVisitor::ExtrapolationTimeStep(m) => extrapolation_time_step = Some(m),
                HkbHandIkControlDataVisitor::HandleChangeSpeed(m) => handle_change_speed = Some(m),
                HkbHandIkControlDataVisitor::HandleChangeMode(m) => handle_change_mode = Some(m),
                HkbHandIkControlDataVisitor::FixUp(m) => fix_up = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            target_position: target_position.unwrap_or_default().into_inner(),
            target_rotation: target_rotation.unwrap_or_default().into_inner(),
            target_normal: target_normal.unwrap_or_default().into_inner(),
            target_handle: target_handle.unwrap_or_default().into_inner(),
            transform_on_fraction: transform_on_fraction.unwrap_or_default().into_inner(),
            normal_on_fraction: normal_on_fraction.unwrap_or_default().into_inner(),
            fade_in_duration: fade_in_duration.unwrap_or_default().into_inner(),
            fade_out_duration: fade_out_duration.unwrap_or_default().into_inner(),
            extrapolation_time_step: extrapolation_time_step.unwrap_or_default().into_inner(),
            handle_change_speed: handle_change_speed.unwrap_or_default().into_inner(),
            handle_change_mode: handle_change_mode.unwrap_or_default().into_inner(),
            fix_up: fix_up.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbHandIkControlData<'a>> for Vec<HkbHandIkControlDataVisitor<'a>> {
    fn from(data: &HkbHandIkControlData<'a>) -> Self {
        vec![
            HkbHandIkControlDataVisitor::TargetPosition(data.target_position.into()),
            HkbHandIkControlDataVisitor::TargetRotation(data.target_rotation.clone().into()),
            HkbHandIkControlDataVisitor::TargetNormal(data.target_normal.into()),
            HkbHandIkControlDataVisitor::TargetHandle(data.target_handle.clone().into()),
            HkbHandIkControlDataVisitor::TransformOnFraction(data.transform_on_fraction.into()),
            HkbHandIkControlDataVisitor::NormalOnFraction(data.normal_on_fraction.into()),
            HkbHandIkControlDataVisitor::FadeInDuration(data.fade_in_duration.into()),
            HkbHandIkControlDataVisitor::FadeOutDuration(data.fade_out_duration.into()),
            HkbHandIkControlDataVisitor::ExtrapolationTimeStep(data.extrapolation_time_step.into()),
            HkbHandIkControlDataVisitor::HandleChangeSpeed(data.handle_change_speed.into()),
            HkbHandIkControlDataVisitor::HandleChangeMode(data.handle_change_mode.clone().into()),
            HkbHandIkControlDataVisitor::FixUp(data.fix_up.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbHandIkControlData<'de> {
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
enum HkbHandIkControlDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "targetPosition")]
    TargetPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "targetRotation")]
    TargetRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "targetNormal")]
    TargetNormal(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "targetHandle")]
    TargetHandle(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transformOnFraction")]
    TransformOnFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "normalOnFraction")]
    NormalOnFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fadeInDuration")]
    FadeInDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fadeOutDuration")]
    FadeOutDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(Primitive<HandleChangeMode>),
    /// Visitor fields
    #[serde(rename = "fixUp")]
    FixUp(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkControlDataVisitor<'de>, "@name",
    ("targetPosition" => TargetPosition(Primitive<Vector4<f32>>)),
    ("targetRotation" => TargetRotation(Primitive<Quaternion<f32>>)),
    ("targetNormal" => TargetNormal(Primitive<Vector4<f32>>)),
    ("targetHandle" => TargetHandle(Primitive<Cow<'de, str>>)),
    ("transformOnFraction" => TransformOnFraction(Primitive<f32>)),
    ("normalOnFraction" => NormalOnFraction(Primitive<f32>)),
    ("fadeInDuration" => FadeInDuration(Primitive<f32>)),
    ("fadeOutDuration" => FadeOutDuration(Primitive<f32>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("handleChangeSpeed" => HandleChangeSpeed(Primitive<f32>)),
    ("handleChangeMode" => HandleChangeMode(Primitive<HandleChangeMode>)),
    ("fixUp" => FixUp(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    #[default]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
