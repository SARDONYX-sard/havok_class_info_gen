//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSLookAtModifier`
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

/// `BSLookAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xd756fc25`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsLookAtModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_modifier: CStyleArray<[bool; 3]>,

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: i16,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub clone_state: (),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_node: CStyleArray<[bool; 1]>,

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variable_binding_set: Cow<'a, str>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub cached_bindables: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub are_bindables_cached: bool,

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
    /// -   name:`"lookAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub look_at_target: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `hkArray<struct BSLookAtModifierBoneData>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub bones: HkArrayClass<BsLookAtModifierBoneData>,
    /// # C++ Class Fields Info
    /// -   name:`"eyeBones"`
    /// -   type: `hkArray<struct BSLookAtModifierBoneData>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub eye_bones: HkArrayClass<BsLookAtModifierBoneData>,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleThresholdDegrees"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_threshold_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"continueLookOutsideOfLimit"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub continue_look_outside_of_limit: bool,
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub on_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub off_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"useBoneGains"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub use_bone_gains: bool,
    /// # C++ Class Fields Info
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub target_location: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetOutsideLimits"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub target_outside_limits: bool,
    /// # C++ Class Fields Info
    /// -   name:`"targetOutOfLimitEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub target_out_of_limit_event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub look_at_camera: bool,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub look_at_camera_x: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub look_at_camera_y: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub look_at_camera_z: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"ballBonesValid"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub ball_bones_valid: bool,
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_skeleton_memory: Cow<'a, str>,
}

impl Serialize for BsLookAtModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsLookAtModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsLookAtModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsLookAtModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsLookAtModifierVisitor<'a>>> for BsLookAtModifier<'a> {
    fn from(_values: Vec<BsLookAtModifierVisitor<'a>>) -> Self {
            let mut enable = None;
            let mut pad_modifier = None;
            let mut user_data = None;
            let mut name = None;
            let mut id = None;
            let mut clone_state = None;
            let mut pad_node = None;
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut look_at_target = None;
            let mut bones = None;
            let mut eye_bones = None;
            let mut limit_angle_degrees = None;
            let mut limit_angle_threshold_degrees = None;
            let mut continue_look_outside_of_limit = None;
            let mut on_gain = None;
            let mut off_gain = None;
            let mut use_bone_gains = None;
            let mut target_location = None;
            let mut target_outside_limits = None;
            let mut target_out_of_limit_event = None;
            let mut look_at_camera = None;
            let mut look_at_camera_x = None;
            let mut look_at_camera_y = None;
            let mut look_at_camera_z = None;
            let mut time_step = None;
            let mut ball_bones_valid = None;
            let mut p_skeleton_memory = None;


        for _value in _values {
            match _value {
                BsLookAtModifierVisitor::Enable(m) => enable = Some(m),
                BsLookAtModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsLookAtModifierVisitor::UserData(m) => user_data = Some(m),
                BsLookAtModifierVisitor::Name(m) => name = Some(m),
                BsLookAtModifierVisitor::Id(m) => id = Some(m),
                BsLookAtModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsLookAtModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsLookAtModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsLookAtModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsLookAtModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsLookAtModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsLookAtModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsLookAtModifierVisitor::LookAtTarget(m) => look_at_target = Some(m),
                BsLookAtModifierVisitor::Bones(m) => bones = Some(m),
                BsLookAtModifierVisitor::EyeBones(m) => eye_bones = Some(m),
                BsLookAtModifierVisitor::LimitAngleDegrees(m) => limit_angle_degrees = Some(m),
                BsLookAtModifierVisitor::LimitAngleThresholdDegrees(m) => limit_angle_threshold_degrees = Some(m),
                BsLookAtModifierVisitor::ContinueLookOutsideOfLimit(m) => continue_look_outside_of_limit = Some(m),
                BsLookAtModifierVisitor::OnGain(m) => on_gain = Some(m),
                BsLookAtModifierVisitor::OffGain(m) => off_gain = Some(m),
                BsLookAtModifierVisitor::UseBoneGains(m) => use_bone_gains = Some(m),
                BsLookAtModifierVisitor::TargetLocation(m) => target_location = Some(m),
                BsLookAtModifierVisitor::TargetOutsideLimits(m) => target_outside_limits = Some(m),
                BsLookAtModifierVisitor::TargetOutOfLimitEvent(m) => target_out_of_limit_event = Some(m),
                BsLookAtModifierVisitor::LookAtCamera(m) => look_at_camera = Some(m),
                BsLookAtModifierVisitor::LookAtCameraX(m) => look_at_camera_x = Some(m),
                BsLookAtModifierVisitor::LookAtCameraY(m) => look_at_camera_y = Some(m),
                BsLookAtModifierVisitor::LookAtCameraZ(m) => look_at_camera_z = Some(m),
                BsLookAtModifierVisitor::TimeStep(m) => time_step = Some(m),
                BsLookAtModifierVisitor::BallBonesValid(m) => ball_bones_valid = Some(m),
                BsLookAtModifierVisitor::PSkeletonMemory(m) => p_skeleton_memory = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enable: enable.unwrap_or_default().into_inner(),
            pad_modifier: pad_modifier.unwrap_or_default(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),
            clone_state: clone_state.unwrap_or_default().into_inner(),
            pad_node: pad_node.unwrap_or_default(),
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            look_at_target: look_at_target.unwrap_or_default().into_inner(),
            bones: bones.unwrap_or_default(),
            eye_bones: eye_bones.unwrap_or_default(),
            limit_angle_degrees: limit_angle_degrees.unwrap_or_default().into_inner(),
            limit_angle_threshold_degrees: limit_angle_threshold_degrees.unwrap_or_default().into_inner(),
            continue_look_outside_of_limit: continue_look_outside_of_limit.unwrap_or_default().into_inner(),
            on_gain: on_gain.unwrap_or_default().into_inner(),
            off_gain: off_gain.unwrap_or_default().into_inner(),
            use_bone_gains: use_bone_gains.unwrap_or_default().into_inner(),
            target_location: target_location.unwrap_or_default().into_inner(),
            target_outside_limits: target_outside_limits.unwrap_or_default().into_inner(),
            target_out_of_limit_event: target_out_of_limit_event.unwrap_or_default(),
            look_at_camera: look_at_camera.unwrap_or_default().into_inner(),
            look_at_camera_x: look_at_camera_x.unwrap_or_default().into_inner(),
            look_at_camera_y: look_at_camera_y.unwrap_or_default().into_inner(),
            look_at_camera_z: look_at_camera_z.unwrap_or_default().into_inner(),
            time_step: time_step.unwrap_or_default().into_inner(),
            ball_bones_valid: ball_bones_valid.unwrap_or_default().into_inner(),
            p_skeleton_memory: p_skeleton_memory.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsLookAtModifier<'a>> for Vec<BsLookAtModifierVisitor<'a>> {
    fn from(data: &BsLookAtModifier<'a>) -> Self {
        vec![
            BsLookAtModifierVisitor::Enable(data.enable.into()),
            BsLookAtModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsLookAtModifierVisitor::UserData(data.user_data.into()),
            BsLookAtModifierVisitor::Name(data.name.clone().into()),
            BsLookAtModifierVisitor::Id(data.id.into()),
            BsLookAtModifierVisitor::CloneState(data.clone_state.into()),
            BsLookAtModifierVisitor::PadNode(data.pad_node.clone()),
            BsLookAtModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsLookAtModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsLookAtModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsLookAtModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsLookAtModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsLookAtModifierVisitor::LookAtTarget(data.look_at_target.into()),
            BsLookAtModifierVisitor::Bones(data.bones.clone()),
            BsLookAtModifierVisitor::EyeBones(data.eye_bones.clone()),
            BsLookAtModifierVisitor::LimitAngleDegrees(data.limit_angle_degrees.into()),
            BsLookAtModifierVisitor::LimitAngleThresholdDegrees(data.limit_angle_threshold_degrees.into()),
            BsLookAtModifierVisitor::ContinueLookOutsideOfLimit(data.continue_look_outside_of_limit.into()),
            BsLookAtModifierVisitor::OnGain(data.on_gain.into()),
            BsLookAtModifierVisitor::OffGain(data.off_gain.into()),
            BsLookAtModifierVisitor::UseBoneGains(data.use_bone_gains.into()),
            BsLookAtModifierVisitor::TargetLocation(data.target_location.into()),
            BsLookAtModifierVisitor::TargetOutsideLimits(data.target_outside_limits.into()),
            BsLookAtModifierVisitor::TargetOutOfLimitEvent(data.target_out_of_limit_event.clone()),
            BsLookAtModifierVisitor::LookAtCamera(data.look_at_camera.into()),
            BsLookAtModifierVisitor::LookAtCameraX(data.look_at_camera_x.into()),
            BsLookAtModifierVisitor::LookAtCameraY(data.look_at_camera_y.into()),
            BsLookAtModifierVisitor::LookAtCameraZ(data.look_at_camera_z.into()),
            BsLookAtModifierVisitor::TimeStep(data.time_step.into()),
            BsLookAtModifierVisitor::BallBonesValid(data.ball_bones_valid.into()),
            BsLookAtModifierVisitor::PSkeletonMemory(data.p_skeleton_memory.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsLookAtModifier<'de> {
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
enum BsLookAtModifierVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// Visitor fields
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "lookAtTarget")]
    LookAtTarget(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bones")]
    Bones(HkArrayClass<BsLookAtModifierBoneData>),
    /// Visitor fields
    #[serde(rename = "eyeBones")]
    EyeBones(HkArrayClass<BsLookAtModifierBoneData>),
    /// Visitor fields
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleThresholdDegrees")]
    LimitAngleThresholdDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "continueLookOutsideOfLimit")]
    ContinueLookOutsideOfLimit(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "useBoneGains")]
    UseBoneGains(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "targetLocation")]
    TargetLocation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "targetOutsideLimits")]
    TargetOutsideLimits(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "targetOutOfLimitEvent")]
    TargetOutOfLimitEvent(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "lookAtCamera")]
    LookAtCamera(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "lookAtCameraX")]
    LookAtCameraX(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lookAtCameraY")]
    LookAtCameraY(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lookAtCameraZ")]
    LookAtCameraZ(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "ballBonesValid", skip_serializing)]
    BallBonesValid(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifierVisitor<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("lookAtTarget" => LookAtTarget(Primitive<bool>)),
    ("bones" => Bones(HkArrayClass<BsLookAtModifierBoneData>)),
    ("eyeBones" => EyeBones(HkArrayClass<BsLookAtModifierBoneData>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("limitAngleThresholdDegrees" => LimitAngleThresholdDegrees(Primitive<f32>)),
    ("continueLookOutsideOfLimit" => ContinueLookOutsideOfLimit(Primitive<bool>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("useBoneGains" => UseBoneGains(Primitive<bool>)),
    ("targetLocation" => TargetLocation(Primitive<Vector4<f32>>)),
    ("targetOutsideLimits" => TargetOutsideLimits(Primitive<bool>)),
    ("targetOutOfLimitEvent" => TargetOutOfLimitEvent(SingleClass<HkbEventProperty<'de>>)),
    ("lookAtCamera" => LookAtCamera(Primitive<bool>)),
    ("lookAtCameraX" => LookAtCameraX(Primitive<f32>)),
    ("lookAtCameraY" => LookAtCameraY(Primitive<f32>)),
    ("lookAtCameraZ" => LookAtCameraZ(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("ballBonesValid" => BallBonesValid(Primitive<bool>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
}
