use cgmath::Vector4;
use roxmltree::Node;

pub fn normalize(str: &str) -> Vec<String> {
    // Split the string using the specified characters and remove empty entries
    str.split(&['(', ')', ',', ' ', '\n', '\r', '\t'][..])
        .filter(|&x| !x.is_empty())
        .map(|x| {
            if x == "-1.#IND00" {
                "0.0".to_string()
            } else {
                x.to_string()
            }
        })
        .collect()
}

pub trait HavokPrimitiveDeserializer<'a> {
    /// Get the texts as valid havok behavior.
    fn string(&self) -> Option<&'a str>;

    fn vector4(&self) -> Vector4<f32>;
}

impl<'a> HavokPrimitiveDeserializer<'a> for Node<'a, 'a> {
    fn string(&self) -> Option<&'a str> {
        let text_in_hkparam = self.text();
        const NULL_CHAR: &str = "\u{2400}";
        match text_in_hkparam == Some(NULL_CHAR) {
            true => None,
            false => text_in_hkparam.map(str::trim),
        }
    }

    fn vector4(&self) -> Vector4<f32> {
        let vec = normalize(self.text().unwrap());

        let mut new_vec = [0f32; 4];
        for n in new_vec.iter_mut() {
            let value = vec[0].parse::<f32>().unwrap();
            *n = value;
        }
        Vector4::from(new_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::finder::HavokFinder as _;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_normalize() {
        let input = "1.0 (2.0, 3.0) -1.#IND00";
        assert_eq!(normalize(input), vec!["1.0", "2.0", "3.0", "0.0"]);
    }

    #[test]
    fn should_deserialize_text() {
        let xml = r#"
            <hkobject>
                <hkparam name="param_name">Expected String</hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let actual = root.children().find_hkparam("param_name").unwrap().string();

        assert_eq!(actual, Some("Expected String"));
    }

    #[test]
    fn should_parse_float() {
        let xml_str = r#"<root>
                            <hkparam name="test">42.0</hkparam>
                          </root>"#;
        let doc = roxmltree::Document::parse(xml_str).unwrap();
        let root = doc.root_element();
        let actual: f32 = root
            .children()
            .find_hkparam("test")
            .unwrap()
            .text()
            .unwrap()
            .parse()
            .unwrap();

        assert_eq!(actual, 42.0);
    }
}
