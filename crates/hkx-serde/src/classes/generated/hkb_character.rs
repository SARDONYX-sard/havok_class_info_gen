//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacter`, a class defined in C++
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
/// -    size: 88
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacter"`: Name of this class.
    #[serde(default = "HkbCharacter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3088a5c5`: Unique value of this class.
    #[serde(default = "HkbCharacter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterHkParam<'a>>
}

impl HkbCharacter<'_> {
    /// Return `"hkbCharacter"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacter".into()
    }

    /// Return `"0x3088a5c5"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3088a5c5".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"nearbyCharacters"`
    /// -   type: `hkArray&lt;hkbCharacter*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nearbyCharacters")]
    NearbyCharacters(Vec<Box<HkbCharacter>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentLod"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentLod")]
    CurrentLod(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numTracksInLod"`
    /// -   type: `hkInt16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numTracksInLod", skip_serializing)]
    NumTracksInLod(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"ragdollDriver"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ragdollDriver", skip_serializing)]
    RagdollDriver(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterControllerDriver"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterControllerDriver", skip_serializing)]
    CharacterControllerDriver(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"footIkDriver"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "footIkDriver", skip_serializing)]
    FootIkDriver(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"handIkDriver"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "handIkDriver", skip_serializing)]
    HandIkDriver(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"setup"`
    /// -   type: `struct hkbCharacterSetup*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setup")]
    Setup(Box<HkbCharacterSetup>),
    /// # Information on fields in the original C++ class
    /// -   name:`"behaviorGraph"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorGraph")]
    BehaviorGraph(Box<HkbBehaviorGraph>),
    /// # Information on fields in the original C++ class
    /// -   name:`"projectData"`
    /// -   type: `struct hkbProjectData*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "projectData")]
    ProjectData(Box<HkbProjectData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"raycastInterface"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "raycastInterface", skip_serializing)]
    RaycastInterface(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | NOT_OWNED | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"worldFromModel"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldFromModel", skip_serializing)]
    WorldFromModel(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"poseLocal"`
    /// -   type: `hkSimpleArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "poseLocal", skip_serializing)]
    PoseLocal(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"deleteWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deleteWorldFromModel", skip_serializing)]
    DeleteWorldFromModel(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"deletePoseLocal"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deletePoseLocal", skip_serializing)]
    DeletePoseLocal(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterHkParam<'de>, "@name",
    ("nearbyCharacters" => NearbyCharacters(Vec<Box<HkbCharacter>>)),
    ("currentLod" => CurrentLod(i16)),
    ("numTracksInLod" => NumTracksInLod(i16)),
    ("name" => Name(String)),
    ("ragdollDriver" => RagdollDriver(())),
    ("characterControllerDriver" => CharacterControllerDriver(())),
    ("footIkDriver" => FootIkDriver(())),
    ("handIkDriver" => HandIkDriver(())),
    ("setup" => Setup(Box<HkbCharacterSetup>)),
    ("behaviorGraph" => BehaviorGraph(Box<HkbBehaviorGraph>)),
    ("projectData" => ProjectData(Box<HkbProjectData>)),
    ("animationBindingSet" => AnimationBindingSet(())),
    ("raycastInterface" => RaycastInterface(())),
    ("world" => World(())),
    ("eventQueue" => EventQueue(())),
    ("worldFromModel" => WorldFromModel(())),
    ("poseLocal" => PoseLocal(Vec<()>)),
    ("deleteWorldFromModel" => DeleteWorldFromModel(bool)),
    ("deletePoseLocal" => DeletePoseLocal(bool)),
}
