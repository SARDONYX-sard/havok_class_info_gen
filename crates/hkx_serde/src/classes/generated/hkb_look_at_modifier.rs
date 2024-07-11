//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbLookAtModifier`
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

/// `hkbLookAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x3d28e066`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbLookAtModifier<'a> {
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
    /// -   name:`"targetWS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub target_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"headForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub head_forward_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"neckForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub neck_forward_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"neckRightLS"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub neck_right_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"eyePositionHS"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub eye_position_hs: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"newTargetGain"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub new_target_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub on_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub off_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleLeft"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_left: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleRight"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_right: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleUp"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_up: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDown"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_down: f32,
    /// # C++ Class Fields Info
    /// -   name:`"headIndex"`
    /// -   type: `hkInt16`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub head_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"neckIndex"`
    /// -   type: `hkInt16`
    /// - offset: 162
    /// -  flags: `FLAGS_NONE`
    pub neck_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"isOn"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub is_on: bool,
    /// # C++ Class Fields Info
    /// -   name:`"individualLimitsOn"`
    /// -   type: `hkBool`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    pub individual_limits_on: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isTargetInsideLimitCone"`
    /// -   type: `hkBool`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    pub is_target_inside_limit_cone: bool,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtLastTargetWS"`
    /// -   type: `hkVector4`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub look_at_last_target_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"lookAtWeight"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub look_at_weight: f32,
}

impl Serialize for HkbLookAtModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbLookAtModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbLookAtModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbLookAtModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbLookAtModifierVisitor<'a>>> for HkbLookAtModifier<'a> {
    fn from(_values: Vec<HkbLookAtModifierVisitor<'a>>) -> Self {
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
            let mut target_ws = None;
            let mut head_forward_ls = None;
            let mut neck_forward_ls = None;
            let mut neck_right_ls = None;
            let mut eye_position_hs = None;
            let mut new_target_gain = None;
            let mut on_gain = None;
            let mut off_gain = None;
            let mut limit_angle_degrees = None;
            let mut limit_angle_left = None;
            let mut limit_angle_right = None;
            let mut limit_angle_up = None;
            let mut limit_angle_down = None;
            let mut head_index = None;
            let mut neck_index = None;
            let mut is_on = None;
            let mut individual_limits_on = None;
            let mut is_target_inside_limit_cone = None;
            let mut look_at_last_target_ws = None;
            let mut look_at_weight = None;


        for _value in _values {
            match _value {
                HkbLookAtModifierVisitor::Enable(m) => enable = Some(m),
                HkbLookAtModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbLookAtModifierVisitor::UserData(m) => user_data = Some(m),
                HkbLookAtModifierVisitor::Name(m) => name = Some(m),
                HkbLookAtModifierVisitor::Id(m) => id = Some(m),
                HkbLookAtModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbLookAtModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbLookAtModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbLookAtModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbLookAtModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbLookAtModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbLookAtModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbLookAtModifierVisitor::TargetWs(m) => target_ws = Some(m),
                HkbLookAtModifierVisitor::HeadForwardLs(m) => head_forward_ls = Some(m),
                HkbLookAtModifierVisitor::NeckForwardLs(m) => neck_forward_ls = Some(m),
                HkbLookAtModifierVisitor::NeckRightLs(m) => neck_right_ls = Some(m),
                HkbLookAtModifierVisitor::EyePositionHs(m) => eye_position_hs = Some(m),
                HkbLookAtModifierVisitor::NewTargetGain(m) => new_target_gain = Some(m),
                HkbLookAtModifierVisitor::OnGain(m) => on_gain = Some(m),
                HkbLookAtModifierVisitor::OffGain(m) => off_gain = Some(m),
                HkbLookAtModifierVisitor::LimitAngleDegrees(m) => limit_angle_degrees = Some(m),
                HkbLookAtModifierVisitor::LimitAngleLeft(m) => limit_angle_left = Some(m),
                HkbLookAtModifierVisitor::LimitAngleRight(m) => limit_angle_right = Some(m),
                HkbLookAtModifierVisitor::LimitAngleUp(m) => limit_angle_up = Some(m),
                HkbLookAtModifierVisitor::LimitAngleDown(m) => limit_angle_down = Some(m),
                HkbLookAtModifierVisitor::HeadIndex(m) => head_index = Some(m),
                HkbLookAtModifierVisitor::NeckIndex(m) => neck_index = Some(m),
                HkbLookAtModifierVisitor::IsOn(m) => is_on = Some(m),
                HkbLookAtModifierVisitor::IndividualLimitsOn(m) => individual_limits_on = Some(m),
                HkbLookAtModifierVisitor::IsTargetInsideLimitCone(m) => is_target_inside_limit_cone = Some(m),
                HkbLookAtModifierVisitor::LookAtLastTargetWs(m) => look_at_last_target_ws = Some(m),
                HkbLookAtModifierVisitor::LookAtWeight(m) => look_at_weight = Some(m),

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
            target_ws: target_ws.unwrap_or_default().into_inner(),
            head_forward_ls: head_forward_ls.unwrap_or_default().into_inner(),
            neck_forward_ls: neck_forward_ls.unwrap_or_default().into_inner(),
            neck_right_ls: neck_right_ls.unwrap_or_default().into_inner(),
            eye_position_hs: eye_position_hs.unwrap_or_default().into_inner(),
            new_target_gain: new_target_gain.unwrap_or_default().into_inner(),
            on_gain: on_gain.unwrap_or_default().into_inner(),
            off_gain: off_gain.unwrap_or_default().into_inner(),
            limit_angle_degrees: limit_angle_degrees.unwrap_or_default().into_inner(),
            limit_angle_left: limit_angle_left.unwrap_or_default().into_inner(),
            limit_angle_right: limit_angle_right.unwrap_or_default().into_inner(),
            limit_angle_up: limit_angle_up.unwrap_or_default().into_inner(),
            limit_angle_down: limit_angle_down.unwrap_or_default().into_inner(),
            head_index: head_index.unwrap_or_default().into_inner(),
            neck_index: neck_index.unwrap_or_default().into_inner(),
            is_on: is_on.unwrap_or_default().into_inner(),
            individual_limits_on: individual_limits_on.unwrap_or_default().into_inner(),
            is_target_inside_limit_cone: is_target_inside_limit_cone.unwrap_or_default().into_inner(),
            look_at_last_target_ws: look_at_last_target_ws.unwrap_or_default().into_inner(),
            look_at_weight: look_at_weight.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbLookAtModifier<'a>> for Vec<HkbLookAtModifierVisitor<'a>> {
    fn from(data: &HkbLookAtModifier<'a>) -> Self {
        vec![
            HkbLookAtModifierVisitor::Enable(data.enable.into()),
            HkbLookAtModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbLookAtModifierVisitor::UserData(data.user_data.into()),
            HkbLookAtModifierVisitor::Name(data.name.clone().into()),
            HkbLookAtModifierVisitor::Id(data.id.into()),
            HkbLookAtModifierVisitor::CloneState(data.clone_state.into()),
            HkbLookAtModifierVisitor::PadNode(data.pad_node.clone()),
            HkbLookAtModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbLookAtModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbLookAtModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbLookAtModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbLookAtModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbLookAtModifierVisitor::TargetWs(data.target_ws.into()),
            HkbLookAtModifierVisitor::HeadForwardLs(data.head_forward_ls.into()),
            HkbLookAtModifierVisitor::NeckForwardLs(data.neck_forward_ls.into()),
            HkbLookAtModifierVisitor::NeckRightLs(data.neck_right_ls.into()),
            HkbLookAtModifierVisitor::EyePositionHs(data.eye_position_hs.into()),
            HkbLookAtModifierVisitor::NewTargetGain(data.new_target_gain.into()),
            HkbLookAtModifierVisitor::OnGain(data.on_gain.into()),
            HkbLookAtModifierVisitor::OffGain(data.off_gain.into()),
            HkbLookAtModifierVisitor::LimitAngleDegrees(data.limit_angle_degrees.into()),
            HkbLookAtModifierVisitor::LimitAngleLeft(data.limit_angle_left.into()),
            HkbLookAtModifierVisitor::LimitAngleRight(data.limit_angle_right.into()),
            HkbLookAtModifierVisitor::LimitAngleUp(data.limit_angle_up.into()),
            HkbLookAtModifierVisitor::LimitAngleDown(data.limit_angle_down.into()),
            HkbLookAtModifierVisitor::HeadIndex(data.head_index.into()),
            HkbLookAtModifierVisitor::NeckIndex(data.neck_index.into()),
            HkbLookAtModifierVisitor::IsOn(data.is_on.into()),
            HkbLookAtModifierVisitor::IndividualLimitsOn(data.individual_limits_on.into()),
            HkbLookAtModifierVisitor::IsTargetInsideLimitCone(data.is_target_inside_limit_cone.into()),
            HkbLookAtModifierVisitor::LookAtLastTargetWs(data.look_at_last_target_ws.into()),
            HkbLookAtModifierVisitor::LookAtWeight(data.look_at_weight.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbLookAtModifier<'de> {
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
enum HkbLookAtModifierVisitor<'a> {
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
    #[serde(rename = "targetWS")]
    TargetWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "headForwardLS")]
    HeadForwardLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "neckForwardLS")]
    NeckForwardLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "neckRightLS")]
    NeckRightLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "eyePositionHS")]
    EyePositionHs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "newTargetGain")]
    NewTargetGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleLeft")]
    LimitAngleLeft(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleRight")]
    LimitAngleRight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleUp")]
    LimitAngleUp(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitAngleDown")]
    LimitAngleDown(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "headIndex")]
    HeadIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "neckIndex")]
    NeckIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "isOn")]
    IsOn(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "individualLimitsOn")]
    IndividualLimitsOn(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isTargetInsideLimitCone")]
    IsTargetInsideLimitCone(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "lookAtLastTargetWS", skip_serializing)]
    LookAtLastTargetWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "lookAtWeight", skip_serializing)]
    LookAtWeight(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbLookAtModifierVisitor<'de>, "@name",
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
    ("targetWS" => TargetWs(Primitive<Vector4<f32>>)),
    ("headForwardLS" => HeadForwardLs(Primitive<Vector4<f32>>)),
    ("neckForwardLS" => NeckForwardLs(Primitive<Vector4<f32>>)),
    ("neckRightLS" => NeckRightLs(Primitive<Vector4<f32>>)),
    ("eyePositionHS" => EyePositionHs(Primitive<Vector4<f32>>)),
    ("newTargetGain" => NewTargetGain(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("limitAngleLeft" => LimitAngleLeft(Primitive<f32>)),
    ("limitAngleRight" => LimitAngleRight(Primitive<f32>)),
    ("limitAngleUp" => LimitAngleUp(Primitive<f32>)),
    ("limitAngleDown" => LimitAngleDown(Primitive<f32>)),
    ("headIndex" => HeadIndex(Primitive<i16>)),
    ("neckIndex" => NeckIndex(Primitive<i16>)),
    ("isOn" => IsOn(Primitive<bool>)),
    ("individualLimitsOn" => IndividualLimitsOn(Primitive<bool>)),
    ("isTargetInsideLimitCone" => IsTargetInsideLimitCone(Primitive<bool>)),
    ("lookAtLastTargetWS" => LookAtLastTargetWs(Primitive<Vector4<f32>>)),
    ("lookAtWeight" => LookAtWeight(Primitive<f32>)),
}
