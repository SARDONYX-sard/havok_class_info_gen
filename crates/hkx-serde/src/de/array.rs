use super::finder::HavokFinder as _;
use super::primitive::HavokPrimitiveDeserializer;
use crate::error::Result;
use roxmltree::Children;
use roxmltree::Node;

pub trait HavokArrayDeserializer<'a> {
    fn class_c_style_array<F, U>(
        self,
        search_name: &str,
        expected_len: usize,
        deserialize: F,
    ) -> Result<Vec<U>>
    where
        F: Fn(&Node) -> Result<U>;

    /// Get the texts of the `hkcstring` tags in the `hkparam` tag.
    fn hkcstrings(self) -> Vec<&'a str>;
}

impl<'a> HavokArrayDeserializer<'a> for Children<'a, 'a> {
    fn hkcstrings(self) -> Vec<&'a str> {
        self.filter(|child| child.tag_name().name() == "hkcstring")
            .map(|child| child.string().unwrap_or_default())
            .collect::<Vec<_>>()
    }

    fn class_c_style_array<F, U>(
        self,
        search_name: &str,
        expected_len: usize,
        deserialize: F,
    ) -> Result<Vec<U>>
    where
        F: Fn(&Node) -> Result<U>,
    {
        let class_nodes = self.classes();

        if expected_len != class_nodes.len() {
            return Err(format!(
            "Content's elements mismatch property required. Property: {}, expected: {} actual: {}",
            search_name,
            expected_len,
            class_nodes.len()
        )
            .into());
        }

        let mut array = Vec::with_capacity(expected_len);
        for ele in &class_nodes {
            array.push(deserialize(ele)?);
        }

        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_deserialize_string_array() {
        let xml = r#"
            <hkobject>
                <hkparam name="param_name" numelements="0">
                    <hkcstring>String 1</hkcstring>
                    <hkcstring>String 2</hkcstring>
                    <hkcstring>String 3</hkcstring>
                </hkparam>

                <!-- Not found the following array -->
                <hkparam name="param_name" numelements="0">
                    <hkcstring>String 4</hkcstring>
                    <hkcstring>String 5</hkcstring>
                    <hkcstring>String 6</hkcstring>
                </hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let element = doc.root_element();

        let result = element.children().hkcstrings();

        assert_eq!(result, vec!["String 1", "String 2", "String 3"]);
    }
}
