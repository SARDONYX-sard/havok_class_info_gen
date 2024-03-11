use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class name.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphStringData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    #[serde(default)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraphStringData"`: Name of this class.
    #[serde(default = "HkbBehaviorGraphStringData::class_name")]
    #[serde(rename = "@class", borrow, skip_deserializing)]
    pub class: Cow<'a, str>,

    /// `0xc713064e`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraphStringData::signature")]
    #[serde(rename = "@signature", borrow, skip_deserializing)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphStringDataHkparam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphStringDataHkparam<'a>>,
}

impl HkbBehaviorGraphStringData<'_> {
    /// Name of this class.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorGraphStringData".into()
    }

    /// Signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc713064e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphStringDataHkparam<'a> {
    /// `"eventNames"`
    #[serde(rename = "eventNames")]
    Event(HkArray<'a>),
    /// `"attributeNames"`
    #[serde(rename = "attributeNames")]
    Attribute(HkArray<'a>),
    /// `"variableNames"`
    #[serde(rename = "variableNames")]
    Variable(HkArray<'a>),
    /// `"characterPropertyNames"`
    #[serde(rename = "characterPropertyNames")]
    CharacterProperty(HkArray<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphStringDataHkparam<'de>, "@name",
    ("eventNames"    => Event(HkArray)),
    ("attributeNames" => Attribute(HkArray)),
    ("variableNames"  => Variable(HkArray)),
    ("characterPropertyNames" => CharacterProperty(HkArray)),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HkArray<'a> {
    #[serde(rename = "@numelements")]
    /// `self.hkcstrings.len()`
    pub numelements: usize,
    #[serde(default, borrow)]
    #[serde(rename = "hkcstring")]
    pub hkcstrings: Vec<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quick_xml::de;

    #[test]
    fn should_serde() {
        let _xml_str = "\
<hkobject name=\"#0085\" class=\"hkbBehaviorGraphStringData\" signature=\"0xc713064e\">\
    <hkparam name=\"eventNames\" numelements=\"4\">\
      <hkcstring>cannedTurnRight90Flee</hkcstring>\
      <hkcstring>cannedTurnRight180Flee</hkcstring>\
      <hkcstring>cannedTurnLeft90Flee</hkcstring>\
      <hkcstring>cannedTurnLeft180Flee</hkcstring>\
    </hkparam>\
    <hkparam name=\"attributeNames\" numelements=\"0\"/>\
    <hkparam name=\"variableNames\" numelements=\"6\">\
      <hkcstring>blendDefault</hkcstring>\
      <hkcstring>blendFast</hkcstring>\
        <hkcstring>blendSlow</hkcstring>\
        <hkcstring>Direction</hkcstring>\
        <hkcstring>IsBlocking</hkcstring>\
        <hkcstring>Speed</hkcstring>\
      </hkparam>\
    <hkparam name=\"characterPropertyNames\" numelements=\"0\"/>\
</hkobject>\
";

        let xml_str = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("crates/hkx-serde/assets")
                .join("dummy.xml"),
        )
        .unwrap()
        .replace('\n', "")
        .replace(" />", "/>")
        .replace("> <", "><")
        .replace(">  <", "><")
        .replace(">   <", "><")
        .replace(">    <", "><");
        let deserialized: HkbBehaviorGraphStringData = de::from_str(&xml_str).unwrap();
        let expected = HkbBehaviorGraphStringData {
            name: "#0085".into(),
            class: "hkbBehaviorGraphStringData".into(),
            signature: "0xc713064e".into(),
            hkparams: vec![
                HkbBehaviorGraphStringDataHkparam::Event(HkArray {
                    numelements: 4,
                    hkcstrings: vec![
                        "cannedTurnRight90Flee".into(),
                        "cannedTurnRight180Flee".into(),
                        "cannedTurnLeft90Flee".into(),
                        "cannedTurnLeft180Flee".into(),
                    ],
                }),
                HkbBehaviorGraphStringDataHkparam::Attribute(HkArray {
                    numelements: 0,
                    hkcstrings: vec![],
                }),
                HkbBehaviorGraphStringDataHkparam::Variable(HkArray {
                    numelements: 6,
                    hkcstrings: vec![
                        "blendDefault".into(),
                        "blendFast".into(),
                        "blendSlow".into(),
                        "Direction".into(),
                        "IsBlocking".into(),
                        "Speed".into(),
                    ],
                }),
                HkbBehaviorGraphStringDataHkparam::CharacterProperty(HkArray {
                    numelements: 0,
                    hkcstrings: vec![],
                }),
            ],
        };

        assert_eq!(deserialized, expected);

        let serialized = quick_xml::se::to_string(&deserialized).unwrap();
        assert_eq!(serialized, xml_str);
    }
}
