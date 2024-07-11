//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterData`
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

/// `hkbCharacterData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x300d6808`
/// -   version: 7
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterData<'a> {
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
    /// -   name:`"characterControllerInfo"`
    /// -   type: `struct hkbCharacterDataCharacterControllerInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub character_controller_info: SingleClass<HkbCharacterDataCharacterControllerInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"modelUpMS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub model_up_ms: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"modelForwardMS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub model_forward_ms: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"modelRightMS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub model_right_ms: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray<struct hkbVariableInfo>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub character_property_infos: HkArrayClass<HkbVariableInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"numBonesPerLod"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub num_bones_per_lod: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub character_property_values: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"footIkDriverInfo"`
    /// -   type: `struct hkbFootIkDriverInfo*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub foot_ik_driver_info: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"handIkDriverInfo"`
    /// -   type: `struct hkbHandIkDriverInfo*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub hand_ik_driver_info: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbCharacterStringData*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub string_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSkeletonInfo"`
    /// -   type: `struct hkbMirroredSkeletonInfo*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub mirrored_skeleton_info: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub scale: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numHands"`
    /// -   type: `hkInt16`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_hands: i16,
    /// # C++ Class Fields Info
    /// -   name:`"numFloatSlots"`
    /// -   type: `hkInt16`
    /// - offset: 130
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_float_slots: i16,
}

impl Serialize for HkbCharacterData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterDataVisitor<'a>>> for HkbCharacterData<'a> {
    fn from(_values: Vec<HkbCharacterDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_controller_info = None;
            let mut model_up_ms = None;
            let mut model_forward_ms = None;
            let mut model_right_ms = None;
            let mut character_property_infos = None;
            let mut num_bones_per_lod = None;
            let mut character_property_values = None;
            let mut foot_ik_driver_info = None;
            let mut hand_ik_driver_info = None;
            let mut string_data = None;
            let mut mirrored_skeleton_info = None;
            let mut scale = None;
            let mut num_hands = None;
            let mut num_float_slots = None;


        for _value in _values {
            match _value {
                HkbCharacterDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterDataVisitor::CharacterControllerInfo(m) => character_controller_info = Some(m),
                HkbCharacterDataVisitor::ModelUpMs(m) => model_up_ms = Some(m),
                HkbCharacterDataVisitor::ModelForwardMs(m) => model_forward_ms = Some(m),
                HkbCharacterDataVisitor::ModelRightMs(m) => model_right_ms = Some(m),
                HkbCharacterDataVisitor::CharacterPropertyInfos(m) => character_property_infos = Some(m),
                HkbCharacterDataVisitor::NumBonesPerLod(m) => num_bones_per_lod = Some(m),
                HkbCharacterDataVisitor::CharacterPropertyValues(m) => character_property_values = Some(m),
                HkbCharacterDataVisitor::FootIkDriverInfo(m) => foot_ik_driver_info = Some(m),
                HkbCharacterDataVisitor::HandIkDriverInfo(m) => hand_ik_driver_info = Some(m),
                HkbCharacterDataVisitor::StringData(m) => string_data = Some(m),
                HkbCharacterDataVisitor::MirroredSkeletonInfo(m) => mirrored_skeleton_info = Some(m),
                HkbCharacterDataVisitor::Scale(m) => scale = Some(m),
                HkbCharacterDataVisitor::NumHands(m) => num_hands = Some(m),
                HkbCharacterDataVisitor::NumFloatSlots(m) => num_float_slots = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_controller_info: character_controller_info.unwrap_or_default(),
            model_up_ms: model_up_ms.unwrap_or_default().into_inner(),
            model_forward_ms: model_forward_ms.unwrap_or_default().into_inner(),
            model_right_ms: model_right_ms.unwrap_or_default().into_inner(),
            character_property_infos: character_property_infos.unwrap_or_default(),
            num_bones_per_lod: num_bones_per_lod.unwrap_or_default(),
            character_property_values: character_property_values.unwrap_or_default().into_inner(),
            foot_ik_driver_info: foot_ik_driver_info.unwrap_or_default().into_inner(),
            hand_ik_driver_info: hand_ik_driver_info.unwrap_or_default().into_inner(),
            string_data: string_data.unwrap_or_default().into_inner(),
            mirrored_skeleton_info: mirrored_skeleton_info.unwrap_or_default().into_inner(),
            scale: scale.unwrap_or_default().into_inner(),
            num_hands: num_hands.unwrap_or_default().into_inner(),
            num_float_slots: num_float_slots.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterData<'a>> for Vec<HkbCharacterDataVisitor<'a>> {
    fn from(data: &HkbCharacterData<'a>) -> Self {
        vec![
            HkbCharacterDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterDataVisitor::CharacterControllerInfo(data.character_controller_info.clone()),
            HkbCharacterDataVisitor::ModelUpMs(data.model_up_ms.into()),
            HkbCharacterDataVisitor::ModelForwardMs(data.model_forward_ms.into()),
            HkbCharacterDataVisitor::ModelRightMs(data.model_right_ms.into()),
            HkbCharacterDataVisitor::CharacterPropertyInfos(data.character_property_infos.clone()),
            HkbCharacterDataVisitor::NumBonesPerLod(data.num_bones_per_lod.clone()),
            HkbCharacterDataVisitor::CharacterPropertyValues(data.character_property_values.clone().into()),
            HkbCharacterDataVisitor::FootIkDriverInfo(data.foot_ik_driver_info.clone().into()),
            HkbCharacterDataVisitor::HandIkDriverInfo(data.hand_ik_driver_info.clone().into()),
            HkbCharacterDataVisitor::StringData(data.string_data.clone().into()),
            HkbCharacterDataVisitor::MirroredSkeletonInfo(data.mirrored_skeleton_info.clone().into()),
            HkbCharacterDataVisitor::Scale(data.scale.into()),
            HkbCharacterDataVisitor::NumHands(data.num_hands.into()),
            HkbCharacterDataVisitor::NumFloatSlots(data.num_float_slots.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterData<'de> {
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
enum HkbCharacterDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "characterControllerInfo")]
    CharacterControllerInfo(SingleClass<HkbCharacterDataCharacterControllerInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "modelUpMS")]
    ModelUpMs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "modelForwardMS")]
    ModelForwardMs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "modelRightMS")]
    ModelRightMs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>),
    /// Visitor fields
    #[serde(rename = "numBonesPerLod")]
    NumBonesPerLod(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "characterPropertyValues")]
    CharacterPropertyValues(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "footIkDriverInfo")]
    FootIkDriverInfo(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "handIkDriverInfo")]
    HandIkDriverInfo(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mirroredSkeletonInfo")]
    MirroredSkeletonInfo(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "scale")]
    Scale(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numHands", skip_serializing)]
    NumHands(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "numFloatSlots", skip_serializing)]
    NumFloatSlots(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterControllerInfo" => CharacterControllerInfo(SingleClass<HkbCharacterDataCharacterControllerInfo<'de>>)),
    ("modelUpMS" => ModelUpMs(Primitive<Vector4<f32>>)),
    ("modelForwardMS" => ModelForwardMs(Primitive<Vector4<f32>>)),
    ("modelRightMS" => ModelRightMs(Primitive<Vector4<f32>>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>)),
    ("numBonesPerLod" => NumBonesPerLod(HkArrayNum<i32>)),
    ("characterPropertyValues" => CharacterPropertyValues(Primitive<Cow<'de, str>>)),
    ("footIkDriverInfo" => FootIkDriverInfo(Primitive<Cow<'de, str>>)),
    ("handIkDriverInfo" => HandIkDriverInfo(Primitive<Cow<'de, str>>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
    ("mirroredSkeletonInfo" => MirroredSkeletonInfo(Primitive<Cow<'de, str>>)),
    ("scale" => Scale(Primitive<f32>)),
    ("numHands" => NumHands(Primitive<i16>)),
    ("numFloatSlots" => NumFloatSlots(Primitive<i16>)),
}
