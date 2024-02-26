//! Finder that `hkparam`
use roxmltree::{Children, Node};

/// Finder defined for `Havok behavior XML`.
pub trait HavokFinder<'a> {
    /// Find the `hkparam` tag with the `name` attribute value.
    fn find_hkparam(&mut self, name: &str) -> Option<Node<'a, 'a>>;

    /// Find the `hkparam` tag with the expected name attribute value in the parent node.
    fn hkparams(self) -> Vec<Node<'a, 'a>>;

    /// find all classes(`hkobject`) in `target`'s `hkparam`
    fn classes(self) -> Vec<Node<'a, 'a>>;
}

impl<'a> HavokFinder<'a> for Children<'a, 'a> {
    fn find_hkparam(&mut self, name: &str) -> Option<Node<'a, 'a>> {
        self.find(|e| {
            e.tag_name().name() == "hkparam"
                && e.attribute("name")
                    .map_or(false, |attr_value| attr_value == name)
        })
    }

    fn hkparams(self) -> Vec<Node<'a, 'a>> {
        self.filter(|e| e.tag_name().name() == "hkparam").collect()
    }

    fn classes(self) -> Vec<Node<'a, 'a>> {
        self.filter(|e| e.tag_name().name() == "hkobject").collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_specified_hkparam() {
        let xml = r#"
            <hkobject>
                <hkparam name="another_param_name">Value</hkparam>
                <hkparam name="param_name">Value</hkparam>
                <hkparam name="another_param_name">Value</hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let element = doc.root_element();

        assert!(&element.children().find_hkparam("param_name").is_some());
    }

    #[test]
    fn should_not_find_first_hkparam() {
        let xml = r#"
            <hkobject>
                <hkparam name="another_param_name">Value</hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let element = doc.root_element();

        assert!(element.children().find_hkparam("param_name").is_none());
    }

    #[test]
    fn should_get_all_hkparams() {
        let xml = r#"
            <hkobject>
                <hkparam name="param_name">Value1</hkparam>
                <hkparam name="param_name">Value2</hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let element = doc.root_element();

        assert_eq!(element.children().hkparams().len(), 2);
    }

    #[test]
    fn should_find_all_classes_in_hkparam() {
        let xml = r#"
            <hkobject>
                <hkparam name="param_name">
                    <hkobject>Class1</hkobject>
                    <hkobject>Class2</hkobject>
                </hkparam>
            </hkobject>
        "#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let classes = doc
            .root_element()
            .children()
            .find_hkparam("param_name")
            .unwrap()
            .children()
            .classes();

        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].text(), Some("Class1"));
        assert_eq!(classes[1].text(), Some("Class2"));
    }
}
