#[cfg(test)]
mod tests {
    use crate::classes::generated::{
        class_params::ClassParams, HkbVariableValue, HkbVariableValueSet,
    };
    use crate::classes::Class;
    use crate::havok_types::{HkArrayClass, HkArrayRef, HkArrayVector};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let result = quick_xml::se::to_string(&Class {
            name: "#0064".into(),
            class: "hkbVariableValueSet".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValueSet(Box::new(HkbVariableValueSet {
                word_variable_values: HkArrayClass {
                    numelements: 3,
                    classes: vec![
                        HkbVariableValue {
                            value: 1045220557,
                        }
                        .into(),
                        HkbVariableValue { value: 0 }.into(),
                        HkbVariableValue { value: 0 }.into(),
                    ],
                },
                quad_variable_values: HkArrayVector {
                    numelements: 2,
                    values: vec![
                        (63.0, 64.0, 65.0, 66.0).into(),
                        (63.0, 64.0, 65.0, 66.0).into(),
                    ],
                },

                variant_variable_values: HkArrayRef {
                    numelements: 2,
                    values: vec!["#0063".into(), "#0064".into()],
                },
                ..Default::default()
            })),
        })
        .unwrap();

        let expected = "\
        <Class name=\"#0064\" class=\"hkbVariableValueSet\" signature=\"0x27812d8d\">\
          <hkparam name=\"wordVariableValues\" numelements=\"3\">\
            <hkobject>\
              <hkparam name=\"value\">1045220557</hkparam>\
            </hkobject>\
            <hkobject>\
              <hkparam name=\"value\">0</hkparam>\
            </hkobject>\
            <hkobject>\
              <hkparam name=\"value\">0</hkparam>\
            </hkobject>\
          </hkparam>\
          <hkparam name=\"quadVariableValues\" numelements=\"2\">(63.000000 64.000000 65.000000 66.000000)(63.000000 64.000000 65.000000 66.000000)</hkparam>\
          <hkparam name=\"variantVariableValues\" numelements=\"2\">#0063 #0064</hkparam>\
        </Class>";
        assert_eq!(result, expected);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
<hkobject name="#0064" class="hkbVariableValueSet" signature="0x27812d8d">
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
    <hkparam name="quadVariableValues" numelements="2">
        (0.000000 1.000000 0.000000 0.000000)
        (0.000000 0.000000 -1.000000 0.000000)
    </hkparam>
    <hkparam name="variantVariableValues" numelements="2">
        #0063 #0064
    </hkparam>
</hkobject>
"###;

        let result: Class = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            result,
            Class {
                name: "#0064".into(),
                class: "hkbVariableValueSet".into(),
                signature: "0x27812d8d".into(),
                hkparams: ClassParams::HkbVariableValueSet(Box::new(HkbVariableValueSet {
                    word_variable_values: HkArrayClass {
                        numelements: 3,
                        classes: vec![
                            HkbVariableValue {
                                value: 1045220557,
                            }
                            .into(),
                            HkbVariableValue { value: 0 }.into(),
                            HkbVariableValue { value: 0 }.into(),
                        ],
                    },
                    quad_variable_values: HkArrayVector {
                        numelements: 2,
                        values: vec![
                            (63.0, 64.0, 65.0, 66.0).into(),
                            (63.0, 64.0, 65.0, 66.0).into(),
                        ],
                    },

                    variant_variable_values: HkArrayRef {
                        numelements: 2,
                        values: vec!["#0063".into(), "#0064".into()],
                    },
                    ..Default::default()
                })),
            }
        );
    }
}
