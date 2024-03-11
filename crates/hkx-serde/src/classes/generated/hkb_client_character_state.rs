//! A Rust structure that implements a serializer/deserializer corresponding to `hkbClientCharacterState`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 208
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbClientCharacterState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbClientCharacterState"`: The original C++ class name.
    #[serde(default = "HkbClientCharacterState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa2624c97`: Unique value of this class.
    #[serde(default = "HkbClientCharacterState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbClientCharacterStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbClientCharacterStateHkParam<'a>>
}

impl HkbClientCharacterState<'_> {
    /// Return `"hkbClientCharacterState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbClientCharacterState".into()
    }

    /// Return `"0xa2624c97"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa2624c97".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClientCharacterStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"deformableSkinIds"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deformableSkinIds")]
    DeformableSkinIds(Vec<Primitive<u64>>),
    /// # Field information in the original C++ class
    /// -   name:`"rigidSkinIds"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidSkinIds")]
    RigidSkinIds(Vec<Primitive<u64>>),
    /// # Field information in the original C++ class
    /// -   name:`"externalEventIds"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "externalEventIds")]
    ExternalEventIds(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"auxiliaryInfo"`
    /// -   type: `hkArray&lt;hkbAuxiliaryNodeInfo*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "auxiliaryInfo")]
    AuxiliaryInfo(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"activeEventIds"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeEventIds")]
    ActiveEventIds(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"activeVariableIds"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeVariableIds")]
    ActiveVariableIds(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"instanceName"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "instanceName")]
    InstanceName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"templateName"`
    /// -   type: `hkStringPtr`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "templateName")]
    TemplateName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"fullPathToProject"`
    /// -   type: `hkStringPtr`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fullPathToProject")]
    FullPathToProject(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"behaviorData"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorData")]
    BehaviorData(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"behaviorInternalState"`
    /// -   type: `struct hkbBehaviorGraphInternalState*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorInternalState")]
    BehaviorInternalState(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"nodeIdToInternalStateMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeIdToInternalStateMap", skip_serializing)]
    NodeIdToInternalStateMap(()),
    /// # Field information in the original C++ class
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"elapsedSimulationTime"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elapsedSimulationTime")]
    ElapsedSimulationTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeleton")]
    Skeleton(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"worldFromModel"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModel")]
    WorldFromModel(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"poseModelSpace"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseModelSpace")]
    PoseModelSpace(Vec<QsTransform<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"rigidAttachmentTransforms"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidAttachmentTransforms")]
    RigidAttachmentTransforms(Vec<QsTransform<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbClientCharacterStateHkParam<'de>, "@name",
    ("deformableSkinIds" => DeformableSkinIds(Vec<Primitive<u64>>)),
    ("rigidSkinIds" => RigidSkinIds(Vec<Primitive<u64>>)),
    ("externalEventIds" => ExternalEventIds(Vec<Primitive<i16>>)),
    ("auxiliaryInfo" => AuxiliaryInfo(Vec<Cow<'a, str>>)),
    ("activeEventIds" => ActiveEventIds(Vec<Primitive<i16>>)),
    ("activeVariableIds" => ActiveVariableIds(Vec<Primitive<i16>>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("instanceName" => InstanceName(Primitive<Cow<'a, str>>)),
    ("templateName" => TemplateName(Primitive<Cow<'a, str>>)),
    ("fullPathToProject" => FullPathToProject(Primitive<Cow<'a, str>>)),
    ("behaviorData" => BehaviorData(Cow<'a, str>)),
    ("behaviorInternalState" => BehaviorInternalState(Cow<'a, str>)),
    ("nodeIdToInternalStateMap" => NodeIdToInternalStateMap(())),
    ("visible" => Visible(Primitive<bool>)),
    ("elapsedSimulationTime" => ElapsedSimulationTime(Primitive<f32>)),
    ("skeleton" => Skeleton(Cow<'a, str>)),
    ("worldFromModel" => WorldFromModel(QsTransform<f32>)),
    ("poseModelSpace" => PoseModelSpace(Vec<QsTransform<f32>>)),
    ("rigidAttachmentTransforms" => RigidAttachmentTransforms(Vec<QsTransform<f32>>)),
}
