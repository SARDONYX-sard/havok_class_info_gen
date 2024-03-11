//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorGraphData`, a class defined in C++
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
/// -    size: 88
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraphData"`: The original C++ class name.
    #[serde(default = "HkbBehaviorGraphData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x95aca5d`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraphData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphDataHkParam<'a>>
}

impl HkbBehaviorGraphData<'_> {
    /// Return `"hkbBehaviorGraphData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorGraphData".into()
    }

    /// Return `"0x95aca5d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x95aca5d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"attributeDefaults"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeDefaults")]
    AttributeDefaults(Vec<Primitive<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"variableInfos"`
    /// -   type: `hkArray&lt;struct hkbVariableInfo&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableInfos")]
    VariableInfos(Vec<HkbVariableInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray&lt;struct hkbVariableInfo&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(Vec<HkbVariableInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"eventInfos"`
    /// -   type: `hkArray&lt;struct hkbEventInfo&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventInfos")]
    EventInfos(Vec<HkbEventInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"wordMinVariableValues"`
    /// -   type: `hkArray&lt;struct hkbVariableValue&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordMinVariableValues")]
    WordMinVariableValues(Vec<HkbVariableValue>),
    /// # Field information in the original C++ class
    /// -   name:`"wordMaxVariableValues"`
    /// -   type: `hkArray&lt;struct hkbVariableValue&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordMaxVariableValues")]
    WordMaxVariableValues(Vec<HkbVariableValue>),
    /// # Field information in the original C++ class
    /// -   name:`"variableInitialValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableInitialValues")]
    VariableInitialValues(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"stringData"`
    /// -   type: `struct hkbBehaviorGraphStringData*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphDataHkParam<'de>, "@name",
    ("attributeDefaults" => AttributeDefaults(Vec<Primitive<f32>>)),
    ("variableInfos" => VariableInfos(Vec<HkbVariableInfo>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(Vec<HkbVariableInfo>)),
    ("eventInfos" => EventInfos(Vec<HkbEventInfo>)),
    ("wordMinVariableValues" => WordMinVariableValues(Vec<HkbVariableValue>)),
    ("wordMaxVariableValues" => WordMaxVariableValues(Vec<HkbVariableValue>)),
    ("variableInitialValues" => VariableInitialValues(Cow<'a, str>)),
    ("stringData" => StringData(Cow<'a, str>)),
}
