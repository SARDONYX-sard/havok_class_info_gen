//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpFixedRigidMotion`
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

/// `hkpFixedRigidMotion`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 288
/// -    vtable: true
/// -    parent: `hkpKeyframedRigidMotion`/`0xbafa2bb7`
/// - signature: `0x64abf85c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpFixedRigidMotion<'a> {
    // C++ Parent class(`hkpKeyframedRigidMotion` => parent: `hkpMotion`) has no fields
    //
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum MotionType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: MotionType,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    pub deactivation_integrate_counter: u8,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationNumInactiveFrames"`
    /// -   type: `hkUint16[2]`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub deactivation_num_inactive_frames: CStyleArray<[u16; 2]>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub motion_state: SingleClass<HkMotionState>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"inertiaAndMassInv"`
    /// -   type: `hkVector4`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub inertia_and_mass_inv: Vector4<f32>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub linear_velocity: Vector4<f32>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"angularVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub angular_velocity: Vector4<f32>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationRefPosition"`
    /// -   type: `hkVector4[2]`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    pub deactivation_ref_position: CStyleArrayVector<Vector4<f32>, 2>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationRefOrientation"`
    /// -   type: `hkUint32[2]`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    pub deactivation_ref_orientation: CStyleArray<[u32; 2]>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"savedMotion"`
    /// -   type: `struct hkpMaxSizeMotion*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    pub saved_motion: Cow<'a, str>,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"savedQualityTypeIndex"`
    /// -   type: `hkUint16`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    pub saved_quality_type_index: u16,
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"gravityFactor"`
    /// -   type: `hkHalf`
    /// - offset: 286
    /// -  flags: `FLAGS_NONE`
    pub gravity_factor: f32,

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
}

impl Serialize for HkpFixedRigidMotion<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpFixedRigidMotionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpFixedRigidMotion<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpFixedRigidMotionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpFixedRigidMotionVisitor<'a>>> for HkpFixedRigidMotion<'a> {
    fn from(_values: Vec<HkpFixedRigidMotionVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut deactivation_integrate_counter = None;
            let mut deactivation_num_inactive_frames = None;
            let mut motion_state = None;
            let mut inertia_and_mass_inv = None;
            let mut linear_velocity = None;
            let mut angular_velocity = None;
            let mut deactivation_ref_position = None;
            let mut deactivation_ref_orientation = None;
            let mut saved_motion = None;
            let mut saved_quality_type_index = None;
            let mut gravity_factor = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;


        for _value in _values {
            match _value {
                HkpFixedRigidMotionVisitor::Type(m) => _type = Some(m),
                HkpFixedRigidMotionVisitor::DeactivationIntegrateCounter(m) => deactivation_integrate_counter = Some(m),
                HkpFixedRigidMotionVisitor::DeactivationNumInactiveFrames(m) => deactivation_num_inactive_frames = Some(m),
                HkpFixedRigidMotionVisitor::MotionState(m) => motion_state = Some(m),
                HkpFixedRigidMotionVisitor::InertiaAndMassInv(m) => inertia_and_mass_inv = Some(m),
                HkpFixedRigidMotionVisitor::LinearVelocity(m) => linear_velocity = Some(m),
                HkpFixedRigidMotionVisitor::AngularVelocity(m) => angular_velocity = Some(m),
                HkpFixedRigidMotionVisitor::DeactivationRefPosition(m) => deactivation_ref_position = Some(m),
                HkpFixedRigidMotionVisitor::DeactivationRefOrientation(m) => deactivation_ref_orientation = Some(m),
                HkpFixedRigidMotionVisitor::SavedMotion(m) => saved_motion = Some(m),
                HkpFixedRigidMotionVisitor::SavedQualityTypeIndex(m) => saved_quality_type_index = Some(m),
                HkpFixedRigidMotionVisitor::GravityFactor(m) => gravity_factor = Some(m),
                HkpFixedRigidMotionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpFixedRigidMotionVisitor::ReferenceCount(m) => reference_count = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            deactivation_integrate_counter: deactivation_integrate_counter.unwrap_or_default().into_inner(),
            deactivation_num_inactive_frames: deactivation_num_inactive_frames.unwrap_or_default(),
            motion_state: motion_state.unwrap_or_default(),
            inertia_and_mass_inv: inertia_and_mass_inv.unwrap_or_default().into_inner(),
            linear_velocity: linear_velocity.unwrap_or_default().into_inner(),
            angular_velocity: angular_velocity.unwrap_or_default().into_inner(),
            deactivation_ref_position: deactivation_ref_position.unwrap_or_default(),
            deactivation_ref_orientation: deactivation_ref_orientation.unwrap_or_default(),
            saved_motion: saved_motion.unwrap_or_default().into_inner(),
            saved_quality_type_index: saved_quality_type_index.unwrap_or_default().into_inner(),
            gravity_factor: gravity_factor.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpFixedRigidMotion<'a>> for Vec<HkpFixedRigidMotionVisitor<'a>> {
    fn from(data: &HkpFixedRigidMotion<'a>) -> Self {
        vec![
            HkpFixedRigidMotionVisitor::Type(data._type.clone().into()),
            HkpFixedRigidMotionVisitor::DeactivationIntegrateCounter(data.deactivation_integrate_counter.into()),
            HkpFixedRigidMotionVisitor::DeactivationNumInactiveFrames(data.deactivation_num_inactive_frames.clone()),
            HkpFixedRigidMotionVisitor::MotionState(data.motion_state.clone()),
            HkpFixedRigidMotionVisitor::InertiaAndMassInv(data.inertia_and_mass_inv.into()),
            HkpFixedRigidMotionVisitor::LinearVelocity(data.linear_velocity.into()),
            HkpFixedRigidMotionVisitor::AngularVelocity(data.angular_velocity.into()),
            HkpFixedRigidMotionVisitor::DeactivationRefPosition(data.deactivation_ref_position.clone()),
            HkpFixedRigidMotionVisitor::DeactivationRefOrientation(data.deactivation_ref_orientation.clone()),
            HkpFixedRigidMotionVisitor::SavedMotion(data.saved_motion.clone().into()),
            HkpFixedRigidMotionVisitor::SavedQualityTypeIndex(data.saved_quality_type_index.into()),
            HkpFixedRigidMotionVisitor::GravityFactor(data.gravity_factor.into()),
            HkpFixedRigidMotionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpFixedRigidMotionVisitor::ReferenceCount(data.reference_count.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpFixedRigidMotion<'de> {
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
enum HkpFixedRigidMotionVisitor<'a> {
    // C++ Parent class(`hkpKeyframedRigidMotion` => parent: `hkpMotion`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<MotionType>),
    /// Visitor fields
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "deactivationNumInactiveFrames")]
    DeactivationNumInactiveFrames(CStyleArray<[u16; 2]>),
    /// Visitor fields
    #[serde(rename = "motionState")]
    MotionState(SingleClass<HkMotionState>),
    /// Visitor fields
    #[serde(rename = "inertiaAndMassInv")]
    InertiaAndMassInv(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "angularVelocity")]
    AngularVelocity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "deactivationRefPosition")]
    DeactivationRefPosition(CStyleArrayVector<Vector4<f32>, 2>),
    /// Visitor fields
    #[serde(rename = "deactivationRefOrientation")]
    DeactivationRefOrientation(CStyleArray<[u32; 2]>),
    /// Visitor fields
    #[serde(rename = "savedMotion")]
    SavedMotion(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "savedQualityTypeIndex")]
    SavedQualityTypeIndex(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "gravityFactor")]
    GravityFactor(Primitive<f32>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpFixedRigidMotionVisitor<'de>, "@name",
    ("type" => Type(Primitive<MotionType>)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("deactivationNumInactiveFrames" => DeactivationNumInactiveFrames(CStyleArray<[u16; 2]>)),
    ("motionState" => MotionState(SingleClass<HkMotionState>)),
    ("inertiaAndMassInv" => InertiaAndMassInv(Primitive<Vector4<f32>>)),
    ("linearVelocity" => LinearVelocity(Primitive<Vector4<f32>>)),
    ("angularVelocity" => AngularVelocity(Primitive<Vector4<f32>>)),
    ("deactivationRefPosition" => DeactivationRefPosition(CStyleArrayVector<Vector4<f32>, 2>)),
    ("deactivationRefOrientation" => DeactivationRefOrientation(CStyleArray<[u32; 2]>)),
    ("savedMotion" => SavedMotion(Primitive<Cow<'de, str>>)),
    ("savedQualityTypeIndex" => SavedQualityTypeIndex(Primitive<u16>)),
    ("gravityFactor" => GravityFactor(Primitive<f32>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
