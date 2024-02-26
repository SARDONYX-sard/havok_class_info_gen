use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct Class<'a> {
    #[serde(borrow)]
    #[serde(rename = "@name")]
    /// e.g. #0106
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(rename = "@class")]
    /// e.g. "hkbBehaviorGraphStringData"
    pub class_name: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: e.g. `0xc713064e`
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` field containing the hkcstring vector
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkParam<'a>>,
}

/// Each C++ Class field and field's type
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HkParam<'a> {
    #[serde(rename = "@name")]
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(rename = "@numelements")]
    /// Exists for hkArray type
    pub numelements: Option<usize>,

    #[serde(rename = "hkcstring")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkcstrings: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "hkobject")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkobjects: Option<Vec<NestHkParam<'a>>>,

    #[serde(rename = "$value")]
    #[serde(borrow)]
    pub value: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NestHkParam<'a> {
    #[serde(borrow)]
    hkparam: InnerHkParam<'a>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InnerHkParam<'a> {
    #[serde(rename = "@name")]
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(rename = "@numelements")]
    /// Exists for hkArray type
    pub numelements: Option<usize>,

    #[serde(rename = "hkcstring")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkcstrings: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "hkobject")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkobjects: Option<Vec<NestHkParam<'a>>>,

    #[serde(rename = "$value")]
    #[serde(borrow)]
    pub value: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkpackfile")]
pub struct PackFile<'a> {
    #[serde(rename = "@classversion")]
    /// Exists for hkArray type
    pub class_version: usize,

    #[serde(rename = "@contentsversion")]
    #[serde(borrow)]
    pub content_version: Cow<'a, str>,

    #[serde(rename = "@toplevelobject")]
    #[serde(borrow)]
    pub top_level_object: Cow<'a, str>,

    #[serde(borrow)]
    pub hksection: HkSection<'a>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HkSection<'a> {
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    #[serde(rename = "hkobject", borrow)]
    pub hkobjects: Vec<Class<'a>>,
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

        let serialized: Class = from_str(&xml_str).unwrap();
        assert_eq!(
            serialized,
            Class {
                name: "#0085".into(),
                class_name: "hkbBehaviorGraphStringData".into(),
                signature: "0xc713064e".into(),
                hkparams: vec![
                    HkParam {
                        name: "eventNames".into(),
                        numelements: Some(4),
                        hkcstrings: Some(vec![
                            "cannedTurnRight90Flee".into(),
                            "cannedTurnRight180Flee".into(),
                            "cannedTurnLeft90Flee".into(),
                            "cannedTurnLeft180Flee".into(),
                        ]),
                        ..Default::default()
                    },
                    HkParam {
                        name: "attributeNames".into(),
                        numelements: Some(0),
                        ..Default::default()
                    },
                    HkParam {
                        name: "variableNames".into(),
                        numelements: Some(6),
                        hkcstrings: Some(vec![
                            "blendDefault".into(),
                            "blendFast".into(),
                            "blendSlow".into(),
                            "Direction".into(),
                            "IsBlocking".into(),
                            "Speed".into(),
                        ]),
                        ..Default::default()
                    },
                    HkParam {
                        name: "characterPropertyNames".into(),
                        numelements: Some(0),
                        ..Default::default()
                    },
                ],
            }
        );

        let deserialized = quick_xml::se::to_string(&serialized).unwrap();
        assert_eq!(deserialized, xml_str);
    }

    #[test]
    fn should_serde_() {
        let xml_str = "\
 		<hkobject name=\"#0111\" class=\"hkbBehaviorGraphData\" signature=\"0x95aca5d\">\
			<hkparam name=\"attributeDefaults\" numelements=\"0\"></hkparam>\
			<hkparam name=\"variableInfos\" numelements=\"0\"></hkparam>\
			<hkparam name=\"characterPropertyInfos\" numelements=\"0\"></hkparam>\
			<hkparam name=\"eventInfos\" numelements=\"0\"></hkparam>\
			<hkparam name=\"variableInitialValues\">#0110</hkparam>\
			<hkparam name=\"stringData\">#0109</hkparam>\
		</hkobject>\
        ";
        let xml_str = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("1hm_locomotion.xml"),
        )
        .unwrap();
        let xml_str = xml_str.as_str();

        println!(
            "{:#?}",
            quick_xml::de::from_str::<PackFile>(xml_str).unwrap()
        );
    }
}
