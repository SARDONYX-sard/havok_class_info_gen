use crate::{
    de::{array::HavokArrayDeserializer, finder::HavokFinder},
    error::Result,
};
use roxmltree::Node;

use super::Deserializer;

#[derive(Debug, PartialEq)]
struct HkbBehaviorGraphStringData {
    event_names: Vec<String>,
    variable_names: Vec<String>,
    attribute_names: Vec<String>,
    character_property_names: Vec<String>,
}

// Implement XmlDeserializer trait for the struct
impl Deserializer for HkbBehaviorGraphStringData {
    fn deserialize<'a>(node: &Node<'a, 'a>) -> Result<Self> {
        let class = node.attribute("class");
        if class != Some("hkbBehaviorGraphStringData") {
            return Err(format!(
                "Expected class name: `hkbBehaviorGraphStringData`, but got: {class:?}",
            )
            .into());
        }

        let variable_names = node
            .children()
            .find_hkparam("variableNames")
            .unwrap()
            .children()
            .hkcstrings()
            .into_iter()
            .map(|s| s.into())
            .collect();
        let event_names = node
            .children()
            .find_hkparam("eventNames")
            .unwrap()
            .children()
            .hkcstrings()
            .into_iter()
            .map(|s| s.into())
            .collect();
        let attribute_names = node
            .children()
            .find_hkparam("attributeNames")
            .unwrap()
            .children()
            .hkcstrings()
            .into_iter()
            .map(|s| s.into())
            .collect();
        let character_property_names = node
            .children()
            .find_hkparam("characterPropertyNames")
            .unwrap()
            .children()
            .hkcstrings()
            .into_iter()
            .map(|s| s.into())
            .collect();

        Ok(HkbBehaviorGraphStringData {
            event_names,
            variable_names,
            attribute_names,
            character_property_names,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serde() {
        let xml_str = "\
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

        let doc = roxmltree::Document::parse(xml_str).unwrap();
        let root_element = doc.root_element();
        let actual = HkbBehaviorGraphStringData::deserialize(&root_element).unwrap();

        assert_eq!(
            actual,
            HkbBehaviorGraphStringData {
                event_names: vec![
                    "cannedTurnRight90Flee",
                    "cannedTurnRight180Flee",
                    "cannedTurnLeft90Flee",
                    "cannedTurnLeft180Flee"
                ]
                .into_iter()
                .map(|s| s.into())
                .collect(),
                variable_names: vec![
                    "blendDefault",
                    "blendFast",
                    "blendSlow",
                    "Direction",
                    "IsBlocking",
                    "Speed"
                ]
                .into_iter()
                .map(|s| s.into())
                .collect(),
                attribute_names: vec![],
                character_property_names: vec![]
            }
        );
    }
}
