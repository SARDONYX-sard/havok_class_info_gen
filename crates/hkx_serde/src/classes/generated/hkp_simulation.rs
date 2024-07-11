//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimulation`
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

/// `hkpSimulation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x97aba922`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimulation<'a> {
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
    /// -   name:`"determinismCheckFrameCounter"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub determinism_check_frame_counter: u32,
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `struct hkpWorld*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"lastProcessingStep"`
    /// -   type: `enum LastProcessingStep`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub last_processing_step: LastProcessingStep,
    /// # C++ Class Fields Info
    /// -   name:`"currentTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub current_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"currentPsiTime"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub current_psi_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"physicsDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub physics_delta_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"simulateUntilTime"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub simulate_until_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub frame_marker_psi_snap: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousStepResult"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub previous_step_result: u32,
}

impl Serialize for HkpSimulation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimulationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimulation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimulationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSimulationVisitor<'a>>> for HkpSimulation<'a> {
    fn from(_values: Vec<HkpSimulationVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut determinism_check_frame_counter = None;
            let mut world = None;
            let mut last_processing_step = None;
            let mut current_time = None;
            let mut current_psi_time = None;
            let mut physics_delta_time = None;
            let mut simulate_until_time = None;
            let mut frame_marker_psi_snap = None;
            let mut previous_step_result = None;


        for _value in _values {
            match _value {
                HkpSimulationVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpSimulationVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpSimulationVisitor::DeterminismCheckFrameCounter(m) => determinism_check_frame_counter = Some(m),
                HkpSimulationVisitor::World(m) => world = Some(m),
                HkpSimulationVisitor::LastProcessingStep(m) => last_processing_step = Some(m),
                HkpSimulationVisitor::CurrentTime(m) => current_time = Some(m),
                HkpSimulationVisitor::CurrentPsiTime(m) => current_psi_time = Some(m),
                HkpSimulationVisitor::PhysicsDeltaTime(m) => physics_delta_time = Some(m),
                HkpSimulationVisitor::SimulateUntilTime(m) => simulate_until_time = Some(m),
                HkpSimulationVisitor::FrameMarkerPsiSnap(m) => frame_marker_psi_snap = Some(m),
                HkpSimulationVisitor::PreviousStepResult(m) => previous_step_result = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            determinism_check_frame_counter: determinism_check_frame_counter.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            last_processing_step: last_processing_step.unwrap_or_default().into_inner(),
            current_time: current_time.unwrap_or_default().into_inner(),
            current_psi_time: current_psi_time.unwrap_or_default().into_inner(),
            physics_delta_time: physics_delta_time.unwrap_or_default().into_inner(),
            simulate_until_time: simulate_until_time.unwrap_or_default().into_inner(),
            frame_marker_psi_snap: frame_marker_psi_snap.unwrap_or_default().into_inner(),
            previous_step_result: previous_step_result.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSimulation<'a>> for Vec<HkpSimulationVisitor<'a>> {
    fn from(data: &HkpSimulation<'a>) -> Self {
        vec![
            HkpSimulationVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpSimulationVisitor::ReferenceCount(data.reference_count.into()),
            HkpSimulationVisitor::DeterminismCheckFrameCounter(data.determinism_check_frame_counter.into()),
            HkpSimulationVisitor::World(data.world.clone().into()),
            HkpSimulationVisitor::LastProcessingStep(data.last_processing_step.clone().into()),
            HkpSimulationVisitor::CurrentTime(data.current_time.into()),
            HkpSimulationVisitor::CurrentPsiTime(data.current_psi_time.into()),
            HkpSimulationVisitor::PhysicsDeltaTime(data.physics_delta_time.into()),
            HkpSimulationVisitor::SimulateUntilTime(data.simulate_until_time.into()),
            HkpSimulationVisitor::FrameMarkerPsiSnap(data.frame_marker_psi_snap.into()),
            HkpSimulationVisitor::PreviousStepResult(data.previous_step_result.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimulation<'de> {
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
enum HkpSimulationVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "determinismCheckFrameCounter")]
    DeterminismCheckFrameCounter(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "world")]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "lastProcessingStep")]
    LastProcessingStep(Primitive<LastProcessingStep>),
    /// Visitor fields
    #[serde(rename = "currentTime")]
    CurrentTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "currentPsiTime")]
    CurrentPsiTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "physicsDeltaTime")]
    PhysicsDeltaTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "simulateUntilTime")]
    SimulateUntilTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousStepResult")]
    PreviousStepResult(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimulationVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("determinismCheckFrameCounter" => DeterminismCheckFrameCounter(Primitive<u32>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("lastProcessingStep" => LastProcessingStep(Primitive<LastProcessingStep>)),
    ("currentTime" => CurrentTime(Primitive<f32>)),
    ("currentPsiTime" => CurrentPsiTime(Primitive<f32>)),
    ("physicsDeltaTime" => PhysicsDeltaTime(Primitive<f32>)),
    ("simulateUntilTime" => SimulateUntilTime(Primitive<f32>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("previousStepResult" => PreviousStepResult(Primitive<u32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum FindContacts {
    #[serde(rename = "FIND_CONTACTS_DEFAULT")]
    #[default]
    FindContactsDefault = 0,
    #[serde(rename = "FIND_CONTACTS_EXTRA")]
    FindContactsExtra = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ResetCollisionInformation {
    #[serde(rename = "RESET_TOI")]
    #[default]
    ResetToi = 1,
    #[serde(rename = "RESET_TIM")]
    ResetTim = 2,
    #[serde(rename = "RESET_AABB")]
    ResetAabb = 4,
    #[serde(rename = "RESET_ALL")]
    ResetAll = 7,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum LastProcessingStep {
    #[serde(rename = "INTEGRATE")]
    #[default]
    Integrate = 0,
    #[serde(rename = "COLLIDE")]
    Collide = 1,
}
