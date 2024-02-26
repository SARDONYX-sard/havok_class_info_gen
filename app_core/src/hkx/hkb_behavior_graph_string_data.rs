use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphStringData<'a> {
    #[serde(borrow)]
    #[serde(rename = "@name")]
    /// #0106
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "HkbBehaviorGraphStringData::class_name")]
    #[serde(rename = "@class")]
    /// "hkbBehaviorGraphStringData"
    pub class: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "HkbBehaviorGraphStringData::signature")]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: `0xc713064e`
    pub signature: Cow<'a, str>,
    /// The `"hkparam"` field containing the hkcstring vector
    pub hkparam: Vec<HkParam<'a>>,
}

impl HkbBehaviorGraphStringData<'_> {
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorGraphStringData".into()
    }

    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc713064e".into()
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Names {
    #[default]
    #[serde(rename = "eventNames")]
    /// `"eventNames"`
    Event,
    #[serde(rename = "attributeNames")]
    /// `"attributeNames"`
    Attribute,
    #[serde(rename = "variableNames")]
    /// `"variableNames"`
    Variable,
    #[serde(rename = "characterPropertyNames")]
    /// `"characterPropertyNames"`
    CharacterProperty,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkParam<'a> {
    #[serde(rename = "@name")]
    /// `"eventNames"` | `"attributeNames"` | `"variableNames"` | `"characterPropertyNames"`
    pub name: Names,
    #[serde(rename = "@numelements")]
    /// `self.hkcstrings.len()`
    pub numelements: usize,
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "hkcstring")]
    pub hkcstrings: Vec<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;

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
                .join("dummy.xml"),
        )
        .unwrap()
        .replace('\n', "")
        .replace(" />", "/>")
        .replace("> <", "><")
        .replace(">  <", "><")
        .replace(">   <", "><")
        .replace(">    <", "><");
        let serialized: HkbBehaviorGraphStringData = from_str(&xml_str).unwrap();
        assert_eq!(
            serialized,
            HkbBehaviorGraphStringData {
                name: "#0085".into(),
                class: "hkbBehaviorGraphStringData".into(),
                signature: "0xc713064e".into(),
                hkparam: vec![
                    HkParam {
                        name: Names::Event,
                        numelements: 4,
                        hkcstrings: vec![
                            "cannedTurnRight90Flee".into(),
                            "cannedTurnRight180Flee".into(),
                            "cannedTurnLeft90Flee".into(),
                            "cannedTurnLeft180Flee".into(),
                        ],
                    },
                    HkParam {
                        name: Names::Attribute,
                        numelements: 0,
                        hkcstrings: vec![],
                    },
                    HkParam {
                        name: Names::Variable,
                        numelements: 6,
                        hkcstrings: vec![
                            "blendDefault".into(),
                            "blendFast".into(),
                            "blendSlow".into(),
                            "Direction".into(),
                            "IsBlocking".into(),
                            "Speed".into(),
                        ],
                    },
                    HkParam {
                        name: Names::CharacterProperty,
                        numelements: 0,
                        hkcstrings: vec![],
                    },
                ],
            }
        );

        let deserialized = quick_xml::se::to_string(&serialized).unwrap();
        assert_eq!(deserialized, xml_str);
    }
}
