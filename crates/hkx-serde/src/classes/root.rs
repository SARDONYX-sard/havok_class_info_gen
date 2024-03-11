use super::AllClass;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkpackfile")]
pub struct Hkx<'a> {
    #[serde(rename = "@classversion")]
    pub class_version: u64,
    #[serde(rename = "@contentsversion")]
    pub content_version: Cow<'a, str>,
    #[serde(rename = "@toplevelobject")]
    pub top_level_object: Cow<'a, str>,

    #[serde(rename = "hksection")]
    #[serde(bound(deserialize = "HkSection<'a>: Deserialize<'de>"))]
    pub hk_section: HkSection<'a>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hksection")]
pub struct HkSection<'a> {
    #[serde(rename = "@name", borrow, skip_deserializing)]
    #[serde(default = "default_section_root")]
    pub name: Cow<'a, str>,

    #[serde(bound(deserialize = "Vec<AllClass<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkobject", borrow)]
    pub classes: Vec<AllClass<'a>>,
}

fn default_section_root() -> Cow<'static, str> {
    "__data__".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::hkb_behavior_graph_string_data::HkbBehaviorGraphStringData;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0056".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: vec![AllClass::HkbBehaviorGraphStringData(
                    HkbBehaviorGraphStringData {
                        name: "#0057".into(),
                        class: "hkbBehaviorGraphStringData".into(),
                        signature: "0xc713064e".into(),
                        hkparams: vec![],
                    },
                )],
            },
        };
        let result = quick_xml::se::to_string(&class).unwrap();

        let expected_xml = "\
<hkpackfile classversion=\"8\" contentsversion=\"hk_2010.2.0-r1\" toplevelobject=\"#0056\">\
    <hksection name=\"__data__\">\
        <hkobject name=\"#0057\" signature=\"HkbBehaviorGraphStringData\" name=\"\" class=\"\" signature=\"\"/>\
    </hksection>\
</hkpackfile>";

        assert_eq!(result, expected_xml);
        dbg!(result);
    }

    #[test]
    fn should_deserialize() -> anyhow::Result<()> {
        let test_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests")
            .join("test3.xml");

        let xml = std::fs::read_to_string(test_path)?;

        let result: Hkx = quick_xml::de::from_str(&xml)?;
        let expected = Hkx::default();
        assert_eq!(result, expected);
        Ok(())
    }
}
