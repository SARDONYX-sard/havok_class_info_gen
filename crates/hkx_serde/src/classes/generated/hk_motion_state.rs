//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMotionState`
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

/// `hkMotionState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: false
/// - signature: `0x5797386e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMotionState {
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"sweptTransform"`
    /// -   type: `struct hkSweptTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub swept_transform: SingleClass<HkSweptTransform>,
    /// # C++ Class Fields Info
    /// -   name:`"deltaAngle"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub delta_angle: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"objectRadius"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub object_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"linearDamping"`
    /// -   type: `hkHalf`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub linear_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"angularDamping"`
    /// -   type: `hkHalf`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    pub angular_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeFactor"`
    /// -   type: `hkHalf`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub time_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    pub max_linear_velocity: u8,
    /// # C++ Class Fields Info
    /// -   name:`"maxAngularVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 171
    /// -  flags: `FLAGS_NONE`
    pub max_angular_velocity: u8,
    /// # C++ Class Fields Info
    /// -   name:`"deactivationClass"`
    /// -   type: `hkUint8`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub deactivation_class: u8,
}

impl Serialize for HkMotionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMotionStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMotionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMotionStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkMotionStateVisitor>> for HkMotionState {
    fn from(_values: Vec<HkMotionStateVisitor>) -> Self {
            let mut transform = None;
            let mut swept_transform = None;
            let mut delta_angle = None;
            let mut object_radius = None;
            let mut linear_damping = None;
            let mut angular_damping = None;
            let mut time_factor = None;
            let mut max_linear_velocity = None;
            let mut max_angular_velocity = None;
            let mut deactivation_class = None;


        for _value in _values {
            match _value {
                HkMotionStateVisitor::Transform(m) => transform = Some(m),
                HkMotionStateVisitor::SweptTransform(m) => swept_transform = Some(m),
                HkMotionStateVisitor::DeltaAngle(m) => delta_angle = Some(m),
                HkMotionStateVisitor::ObjectRadius(m) => object_radius = Some(m),
                HkMotionStateVisitor::LinearDamping(m) => linear_damping = Some(m),
                HkMotionStateVisitor::AngularDamping(m) => angular_damping = Some(m),
                HkMotionStateVisitor::TimeFactor(m) => time_factor = Some(m),
                HkMotionStateVisitor::MaxLinearVelocity(m) => max_linear_velocity = Some(m),
                HkMotionStateVisitor::MaxAngularVelocity(m) => max_angular_velocity = Some(m),
                HkMotionStateVisitor::DeactivationClass(m) => deactivation_class = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transform: transform.unwrap_or_default().into_inner(),
            swept_transform: swept_transform.unwrap_or_default(),
            delta_angle: delta_angle.unwrap_or_default().into_inner(),
            object_radius: object_radius.unwrap_or_default().into_inner(),
            linear_damping: linear_damping.unwrap_or_default().into_inner(),
            angular_damping: angular_damping.unwrap_or_default().into_inner(),
            time_factor: time_factor.unwrap_or_default().into_inner(),
            max_linear_velocity: max_linear_velocity.unwrap_or_default().into_inner(),
            max_angular_velocity: max_angular_velocity.unwrap_or_default().into_inner(),
            deactivation_class: deactivation_class.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkMotionState> for Vec<HkMotionStateVisitor> {
    fn from(data: &HkMotionState) -> Self {
        vec![
            HkMotionStateVisitor::Transform(data.transform.clone().into()),
            HkMotionStateVisitor::SweptTransform(data.swept_transform.clone()),
            HkMotionStateVisitor::DeltaAngle(data.delta_angle.into()),
            HkMotionStateVisitor::ObjectRadius(data.object_radius.into()),
            HkMotionStateVisitor::LinearDamping(data.linear_damping.into()),
            HkMotionStateVisitor::AngularDamping(data.angular_damping.into()),
            HkMotionStateVisitor::TimeFactor(data.time_factor.into()),
            HkMotionStateVisitor::MaxLinearVelocity(data.max_linear_velocity.into()),
            HkMotionStateVisitor::MaxAngularVelocity(data.max_angular_velocity.into()),
            HkMotionStateVisitor::DeactivationClass(data.deactivation_class.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMotionState {
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
enum HkMotionStateVisitor {
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
    /// Visitor fields
    #[serde(rename = "sweptTransform")]
    SweptTransform(SingleClass<HkSweptTransform>),
    /// Visitor fields
    #[serde(rename = "deltaAngle")]
    DeltaAngle(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "objectRadius")]
    ObjectRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "linearDamping")]
    LinearDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "angularDamping")]
    AngularDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeFactor")]
    TimeFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "maxAngularVelocity")]
    MaxAngularVelocity(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "deactivationClass")]
    DeactivationClass(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMotionStateVisitor, "@name",
    ("transform" => Transform(Primitive<Transform<f32>>)),
    ("sweptTransform" => SweptTransform(SingleClass<HkSweptTransform>)),
    ("deltaAngle" => DeltaAngle(Primitive<Vector4<f32>>)),
    ("objectRadius" => ObjectRadius(Primitive<f32>)),
    ("linearDamping" => LinearDamping(Primitive<f32>)),
    ("angularDamping" => AngularDamping(Primitive<f32>)),
    ("timeFactor" => TimeFactor(Primitive<f32>)),
    ("maxLinearVelocity" => MaxLinearVelocity(Primitive<u8>)),
    ("maxAngularVelocity" => MaxAngularVelocity(Primitive<u8>)),
    ("deactivationClass" => DeactivationClass(Primitive<u8>)),
}
