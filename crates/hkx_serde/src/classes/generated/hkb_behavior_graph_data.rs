//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphData`
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

/// `hkbBehaviorGraphData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x95aca5d`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBehaviorGraphData<'a> {
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
    /// -   name:`"attributeDefaults"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub attribute_defaults: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"variableInfos"`
    /// -   type: `hkArray<struct hkbVariableInfo>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub variable_infos: HkArrayClass<HkbVariableInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray<struct hkbVariableInfo>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub character_property_infos: HkArrayClass<HkbVariableInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"eventInfos"`
    /// -   type: `hkArray<struct hkbEventInfo>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub event_infos: HkArrayClass<HkbEventInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"wordMinVariableValues"`
    /// -   type: `hkArray<struct hkbVariableValue>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub word_min_variable_values: HkArrayClass<HkbVariableValue>,
    /// # C++ Class Fields Info
    /// -   name:`"wordMaxVariableValues"`
    /// -   type: `hkArray<struct hkbVariableValue>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub word_max_variable_values: HkArrayClass<HkbVariableValue>,
    /// # C++ Class Fields Info
    /// -   name:`"variableInitialValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub variable_initial_values: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbBehaviorGraphStringData*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub string_data: Cow<'a, str>,
}

impl Serialize for HkbBehaviorGraphData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBehaviorGraphDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBehaviorGraphData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBehaviorGraphDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBehaviorGraphDataVisitor<'a>>> for HkbBehaviorGraphData<'a> {
    fn from(_values: Vec<HkbBehaviorGraphDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut attribute_defaults = None;
            let mut variable_infos = None;
            let mut character_property_infos = None;
            let mut event_infos = None;
            let mut word_min_variable_values = None;
            let mut word_max_variable_values = None;
            let mut variable_initial_values = None;
            let mut string_data = None;


        for _value in _values {
            match _value {
                HkbBehaviorGraphDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBehaviorGraphDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBehaviorGraphDataVisitor::AttributeDefaults(m) => attribute_defaults = Some(m),
                HkbBehaviorGraphDataVisitor::VariableInfos(m) => variable_infos = Some(m),
                HkbBehaviorGraphDataVisitor::CharacterPropertyInfos(m) => character_property_infos = Some(m),
                HkbBehaviorGraphDataVisitor::EventInfos(m) => event_infos = Some(m),
                HkbBehaviorGraphDataVisitor::WordMinVariableValues(m) => word_min_variable_values = Some(m),
                HkbBehaviorGraphDataVisitor::WordMaxVariableValues(m) => word_max_variable_values = Some(m),
                HkbBehaviorGraphDataVisitor::VariableInitialValues(m) => variable_initial_values = Some(m),
                HkbBehaviorGraphDataVisitor::StringData(m) => string_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            attribute_defaults: attribute_defaults.unwrap_or_default(),
            variable_infos: variable_infos.unwrap_or_default(),
            character_property_infos: character_property_infos.unwrap_or_default(),
            event_infos: event_infos.unwrap_or_default(),
            word_min_variable_values: word_min_variable_values.unwrap_or_default(),
            word_max_variable_values: word_max_variable_values.unwrap_or_default(),
            variable_initial_values: variable_initial_values.unwrap_or_default().into_inner(),
            string_data: string_data.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBehaviorGraphData<'a>> for Vec<HkbBehaviorGraphDataVisitor<'a>> {
    fn from(data: &HkbBehaviorGraphData<'a>) -> Self {
        vec![
            HkbBehaviorGraphDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBehaviorGraphDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbBehaviorGraphDataVisitor::AttributeDefaults(data.attribute_defaults.clone()),
            HkbBehaviorGraphDataVisitor::VariableInfos(data.variable_infos.clone()),
            HkbBehaviorGraphDataVisitor::CharacterPropertyInfos(data.character_property_infos.clone()),
            HkbBehaviorGraphDataVisitor::EventInfos(data.event_infos.clone()),
            HkbBehaviorGraphDataVisitor::WordMinVariableValues(data.word_min_variable_values.clone()),
            HkbBehaviorGraphDataVisitor::WordMaxVariableValues(data.word_max_variable_values.clone()),
            HkbBehaviorGraphDataVisitor::VariableInitialValues(data.variable_initial_values.clone().into()),
            HkbBehaviorGraphDataVisitor::StringData(data.string_data.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBehaviorGraphData<'de> {
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
enum HkbBehaviorGraphDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "attributeDefaults")]
    AttributeDefaults(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "variableInfos")]
    VariableInfos(HkArrayClass<HkbVariableInfo>),
    /// Visitor fields
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>),
    /// Visitor fields
    #[serde(rename = "eventInfos")]
    EventInfos(HkArrayClass<HkbEventInfo>),
    /// Visitor fields
    #[serde(rename = "wordMinVariableValues")]
    WordMinVariableValues(HkArrayClass<HkbVariableValue>),
    /// Visitor fields
    #[serde(rename = "wordMaxVariableValues")]
    WordMaxVariableValues(HkArrayClass<HkbVariableValue>),
    /// Visitor fields
    #[serde(rename = "variableInitialValues")]
    VariableInitialValues(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("attributeDefaults" => AttributeDefaults(HkArrayNum<f32>)),
    ("variableInfos" => VariableInfos(HkArrayClass<HkbVariableInfo>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>)),
    ("eventInfos" => EventInfos(HkArrayClass<HkbEventInfo>)),
    ("wordMinVariableValues" => WordMinVariableValues(HkArrayClass<HkbVariableValue>)),
    ("wordMaxVariableValues" => WordMaxVariableValues(HkArrayClass<HkbVariableValue>)),
    ("variableInitialValues" => VariableInitialValues(Primitive<Cow<'de, str>>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
}
