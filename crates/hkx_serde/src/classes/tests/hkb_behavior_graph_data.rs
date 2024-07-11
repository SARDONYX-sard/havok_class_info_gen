#[cfg(test)]
mod tests {
    use crate::classes::generated::class_params::ClassParams;
    use crate::classes::generated::{HkbBehaviorGraphData, HkbVariableInfo, VariableType};
    use crate::classes::Class;
    use crate::havok_types::{HkArrayClass, HkArrayClassParam};
    use pretty_assertions::assert_eq;

    const HKB_BEHAVIOR_GRAPH_DATA: &str = r###"
<hkobject name="#0087" class="hkbBehaviorGraphData" signature="0x95aca5d">
  <hkparam name="attributeDefaults" numelements="0"></hkparam>
  <hkparam name="variableInfos" numelements="101">
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_REAL</hkparam>
    </hkobject>
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_INT32</hkparam>
    </hkobject>
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_BOOL</hkparam>
    </hkobject>
  </hkparam>
  <hkparam name="wordMinVariableValues" numelements="0"></hkparam>
  <hkparam name="wordMaxVariableValues" numelements="0"></hkparam>
  <hkparam name="variableInitialValues">#0086</hkparam>
  <hkparam name="stringData">#0085</hkparam>
</hkobject>
"###;

    #[test]
    fn should_serialize() {
        let class = Class {
            name: "#0087a".into(),
            class: "hkbBehaviorGraphData".into(),
            signature: "0x95aca5d".into(),
            hkparams: ClassParams::HkbBehaviorGraphData(Box::new(HkbBehaviorGraphData {
                variable_infos: HkArrayClass {
                    numelements: 101,
                    classes: vec![
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeReal,
                            ..Default::default()
                        },
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeInt32,
                            ..Default::default()
                        },
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeBool,
                            ..Default::default()
                        },
                    ]
                    .into_iter()
                    .map(HkArrayClassParam::from)
                    .collect(),
                },
                variable_initial_values: "#0086".into(),
                string_data: "#0085".into(),
                ..Default::default()
            })),
        };

        let serialized = quick_xml::se::to_string(&class).unwrap();
        assert_eq!(serialized, HKB_BEHAVIOR_GRAPH_DATA);
    }

    #[test]
    fn should_deserialize() {
        let deserialized: Class = quick_xml::de::from_str(HKB_BEHAVIOR_GRAPH_DATA).unwrap();

        let expected = Class {
            name: "#0087a".into(),
            class: "hkbBehaviorGraphData".into(),
            signature: "0x95aca5d".into(),
            hkparams: ClassParams::HkbBehaviorGraphData(Box::new(HkbBehaviorGraphData {
                variable_infos: HkArrayClass {
                    numelements: 101,
                    classes: vec![
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeReal,
                            ..Default::default()
                        },
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeInt32,
                            ..Default::default()
                        },
                        HkbVariableInfo {
                            _type: VariableType::VariableTypeBool,
                            ..Default::default()
                        },
                    ]
                    .into_iter()
                    .map(HkArrayClassParam::from)
                    .collect(),
                },
                variable_initial_values: "#0086".into(),
                string_data: "#0085".into(),
                ..Default::default()
            })),
        };
        assert_eq!(deserialized, expected);
    }
}
