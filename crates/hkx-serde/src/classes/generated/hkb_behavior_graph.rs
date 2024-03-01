//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorGraph`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 176
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraph<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraph"`: Name of this class.
    #[serde(default = "HkbBehaviorGraph::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb1218f86`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraph::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphHkParam<'a>>
}

impl HkbBehaviorGraph<'_> {
    /// Return `"hkbBehaviorGraph"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBehaviorGraph".into()
    }

    /// Return `"0xb1218f86"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb1218f86".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"variableMode"`
    /// -   type: `enum VariableMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableMode")]
    VariableMode(VariableMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"uniqueIdPool"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uniqueIdPool", skip_serializing)]
    UniqueIdPool(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"idToStateMachineTemplateMap"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "idToStateMachineTemplateMap", skip_serializing)]
    IdToStateMachineTemplateMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"mirroredExternalIdMap"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredExternalIdMap", skip_serializing)]
    MirroredExternalIdMap(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"pseudoRandomGenerator"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pseudoRandomGenerator", skip_serializing)]
    PseudoRandomGenerator(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootGenerator")]
    RootGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Box<HkbBehaviorGraphData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rootGeneratorClone"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootGeneratorClone", skip_serializing)]
    RootGeneratorClone(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"activeNodes"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodes", skip_serializing)]
    ActiveNodes(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"activeNodeTemplateToIndexMap"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodeTemplateToIndexMap", skip_serializing)]
    ActiveNodeTemplateToIndexMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"activeNodesChildrenIndices"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodesChildrenIndices", skip_serializing)]
    ActiveNodesChildrenIndices(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"globalTransitionData"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "globalTransitionData", skip_serializing)]
    GlobalTransitionData(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"attributeIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributeIdMap", skip_serializing)]
    AttributeIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableValueSet"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableValueSet", skip_serializing)]
    VariableValueSet(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"nodeTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeTemplateToCloneMap", skip_serializing)]
    NodeTemplateToCloneMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"nodeCloneToTemplateMap"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeCloneToTemplateMap", skip_serializing)]
    NodeCloneToTemplateMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"stateListenerTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateListenerTemplateToCloneMap", skip_serializing)]
    StateListenerTemplateToCloneMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"nodePartitionInfo"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodePartitionInfo", skip_serializing)]
    NodePartitionInfo(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"numIntermediateOutputs"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numIntermediateOutputs", skip_serializing)]
    NumIntermediateOutputs(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"jobs"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "jobs", skip_serializing)]
    Jobs(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"allPartitionMemory"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "allPartitionMemory", skip_serializing)]
    AllPartitionMemory(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numStaticNodes"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numStaticNodes", skip_serializing)]
    NumStaticNodes(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextUniqueId"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextUniqueId", skip_serializing)]
    NextUniqueId(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isLinked"`
    /// -   type: `hkBool`
    /// - offset: 173
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isLinked", skip_serializing)]
    IsLinked(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 175
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphHkParam<'de>, "@name",
    ("variableMode" => VariableMode(VariableMode)),
    ("uniqueIdPool" => UniqueIdPool(Vec<()>)),
    ("idToStateMachineTemplateMap" => IdToStateMachineTemplateMap(())),
    ("mirroredExternalIdMap" => MirroredExternalIdMap(Vec<()>)),
    ("pseudoRandomGenerator" => PseudoRandomGenerator(())),
    ("rootGenerator" => RootGenerator(Box<HkbGenerator>)),
    ("data" => Data(Box<HkbBehaviorGraphData>)),
    ("rootGeneratorClone" => RootGeneratorClone(())),
    ("activeNodes" => ActiveNodes(())),
    ("activeNodeTemplateToIndexMap" => ActiveNodeTemplateToIndexMap(())),
    ("activeNodesChildrenIndices" => ActiveNodesChildrenIndices(())),
    ("globalTransitionData" => GlobalTransitionData(())),
    ("eventIdMap" => EventIdMap(())),
    ("attributeIdMap" => AttributeIdMap(())),
    ("variableIdMap" => VariableIdMap(())),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(())),
    ("variableValueSet" => VariableValueSet(())),
    ("nodeTemplateToCloneMap" => NodeTemplateToCloneMap(())),
    ("nodeCloneToTemplateMap" => NodeCloneToTemplateMap(())),
    ("stateListenerTemplateToCloneMap" => StateListenerTemplateToCloneMap(())),
    ("nodePartitionInfo" => NodePartitionInfo(())),
    ("numIntermediateOutputs" => NumIntermediateOutputs(i32)),
    ("jobs" => Jobs(Vec<()>)),
    ("allPartitionMemory" => AllPartitionMemory(Vec<()>)),
    ("numStaticNodes" => NumStaticNodes(i16)),
    ("nextUniqueId" => NextUniqueId(i16)),
    ("isActive" => IsActive(bool)),
    ("isLinked" => IsLinked(bool)),
    ("updateActiveNodes" => UpdateActiveNodes(bool)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VariableMode {
    #[serde(rename = "VARIABLE_MODE_DISCARD_WHEN_INACTIVE")]
    VariableModeDiscardWhenInactive = 0,
    #[serde(rename = "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE")]
    VariableModeMaintainValuesWhenInactive = 1,
}
