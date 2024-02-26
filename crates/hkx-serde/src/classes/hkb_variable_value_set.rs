use super::Deserializer;
use crate::de::finder::HavokFinder as _;
use crate::de::primitive::HavokPrimitiveDeserializer as _;
use crate::error::{HkxError, Result};
use cgmath::Vector4;
use roxmltree::Node;

struct HkbVariableValue {
    value: i32,
}

impl Deserializer for HkbVariableValue {
    fn deserialize<'a>(node: &Node<'a, 'a>) -> Result<Self> {
        let text = node
            .children()
            .find_hkparam("value")
            .unwrap()
            .text()
            .unwrap();
        let value = text
            .parse::<i32>()
            .map_err(|_e| HkxError::ParseError(text.into(), "i32".into()))?;
        Ok(Self { value })
    }
}

struct HkbVariableValueSet<T> {
    word_variable_values: Vec<HkbVariableValue>,
    quad_variable_values: Vec<Vector4<f32>>,
    variant_variable_values: Vec<T>,
}

impl<T: Deserializer> Deserializer for HkbVariableValueSet<T> {
    fn deserialize<'a>(node: &Node<'a, 'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut word_variable_values = Vec::new();
        for i in &node
            .children()
            .find_hkparam("wordVariableValues")
            .unwrap()
            .children()
            .classes()
        {
            word_variable_values.push(HkbVariableValue::deserialize(i).unwrap());
        }

        let mut quad_variable_values = Vec::new();
        for i in &node
            .children()
            .find_hkparam("quadVariableValues")
            .unwrap()
            .children()
            .classes()
        {
            quad_variable_values.push(i.vector4());
        }

        let mut variant_variable_values = Vec::new();
        for i in &node
            .children()
            .find_hkparam("variantVariableValues")
            .unwrap()
            .children()
            .classes()
        {
            variant_variable_values.push(T::deserialize(i)?);
        }

        Ok(Self {
            word_variable_values,
            quad_variable_values,
            variant_variable_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::finder::HavokFinder;

    #[test]
    fn test_read_class_array_with_invalid_attributes() {
        let xml = r#"
<hkobject name="\#0060" class="hkbVariableValueSet" signature="0x27812d8d">
      <hkparam name="wordVariableValues" numelements="17">
        <hkobject>
          <hkparam name="value">1045220557</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
    </hkparam>
</hkobject>
<hkobject name="\#0064" class="hkbVariableValueSet" signature="0x27812d8d">
      <hkparam name="wordVariableValues" numelements="3">
        <hkobject>
          <hkparam name="value">1045220557</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
          <hkparam name="value">0</hkparam>
        </hkobject>
    </hkparam>
    <hkparam name="quadVariableValues" numelements="0">
    </hkparam>
    <hkparam name="variantVariableValues" numelements="1">
        #0063
    </hkparam>
</hkobject>
"#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let node = doc.root_element();

        // let result = HkbVariableValueSet::deserialize(&node).unwrap();

        // assert_eq!(result, vec![1045220557, 0, 0, 0, 0]);
    }
}
