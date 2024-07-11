#[cfg(test)]
mod tests {
    use crate::classes::{
        class_params::ClassParams,
        root::{HkSection, Hkx},
        Class, EventMode, HkRootLevelContainer, HkRootLevelContainerNamedVariant, HkbProjectData,
        HkbProjectStringData,
    };
    use crate::havok_types::{HkArrayClass, HkArrayClassParam, HkArrayStringPtr, Vector4};
    use crate::helpers::serde::to_string_pretty_xml;
    use ordered_float::OrderedFloat;
    use pretty_assertions::assert_eq;

    const DEFAULT_MALE_XML: &str = r##"<hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject="#0050">
  <hksection name="__data__">
    <hkobject name="#0050" class="hkRootLevelContainer" signature="0x2772c11e">
      <hkparam name="namedVariants" numelements="1">
        <hkobject>
          <hkparam name="name">
            hkbProjectData
          </hkparam>
          <hkparam name="className">
            hkbProjectData
          </hkparam>
          <hkparam name="variant">
            #0051
          </hkparam>
        </hkobject>
      </hkparam>
    </hkobject>
    <hkobject name="#0051" class="hkbProjectData" signature="0x13a39ba7">
      <hkparam name="worldUpWS">
        (0.000000 0.000000 1.000000 0.000000)
      </hkparam>
      <hkparam name="stringData">
        #0052
      </hkparam>
      <hkparam name="defaultEventMode">
        EVENT_MODE_IGNORE_FROM_GENERATOR
      </hkparam>
    </hkobject>
    <hkobject name="#0052" class="hkbProjectStringData" signature="0x076ad60a">
      <hkparam name="animationFilenames" numelements="0"/>
      <hkparam name="behaviorFilenames" numelements="0"/>
      <hkparam name="characterFilenames" numelements="1">
        <hkcstring>Characters\DefaultMale.hkx</hkcstring>
      </hkparam>
      <hkparam name="eventNames" numelements="0"/>
      <hkparam name="animationPath"/>
      <hkparam name="behaviorPath"/>
      <hkparam name="characterPath"/>
      <hkparam name="fullPathToSource"/>
    </hkobject>
  </hksection>
</hkpackfile>"##;

    #[test]
    fn should_serialize() {
        let class = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0050".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: [
                    Class {
                        name: 50.into(),
                        class: "hkRootLevelContainer".into(),
                        signature: 0x2772c11e.into(),
                        hkparams: ClassParams::HkRootLevelContainer(Box::new(
                            HkRootLevelContainer {
                                named_variants: HkArrayClass {
                                    numelements: 1,
                                    classes: [HkArrayClassParam {
                                        hkparam: HkRootLevelContainerNamedVariant {
                                            name: "hkbProjectData".into(),
                                            class_name: "hkbProjectData".into(),
                                            variant: "#0051".into(),
                                        },
                                    }]
                                    .to_vec(),
                                },
                            },
                        )),
                    },
                    Class {
                        name: 51.into(),
                        class: "hkbProjectData".into(),
                        signature: 0x13a39ba7.into(),
                        hkparams: ClassParams::HkbProjectData(Box::new(HkbProjectData {
                            mem_size_and_flags: 0,
                            reference_count: 0,
                            world_up_ws: Vector4 {
                                x: OrderedFloat(0.0),
                                y: OrderedFloat(0.0),
                                z: OrderedFloat(1.0),
                                w: OrderedFloat(0.0),
                            },
                            string_data: "#0052".into(),
                            default_event_mode: EventMode::EventModeIgnoreFromGenerator,
                        })),
                    },
                    Class {
                        name: 52.into(),
                        class: "hkbProjectStringData".into(),
                        signature: 0x076ad60a.into(),
                        hkparams: ClassParams::HkbProjectStringData(Box::new(
                            HkbProjectStringData {
                                character_filenames: HkArrayStringPtr {
                                    numelements: 1,
                                    hkcstrings: ["Characters\\DefaultMale.hkx".into()].to_vec(),
                                },
                                ..Default::default()
                            },
                        )),
                    },
                ]
                .to_vec(),
            },
        };

        let result = to_string_pretty_xml(&class, ' ', 2).unwrap();
        assert_eq!(result, DEFAULT_MALE_XML);
    }

    #[test]
    #[quick_tracing::init(test = "should_deserialize(root)", level = "Debug")]
    fn should_deserialize() -> anyhow::Result<()> {
        let test_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests")
            .join("defaultmale.xml");
        // .join("1hm_behavior_x86.xml");

        let xml = std::fs::read_to_string(test_path)?;
        let result: Hkx = quick_xml::de::from_str(&xml)?;

        let expected = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0050".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: [
                    Class {
                        name: 50.into(),
                        class: "hkRootLevelContainer".into(),
                        signature: 0x2772c11e.into(),
                        hkparams: ClassParams::HkRootLevelContainer(Box::new(
                            HkRootLevelContainer {
                                named_variants: HkArrayClass {
                                    numelements: 1,
                                    classes: [HkArrayClassParam {
                                        hkparam: HkRootLevelContainerNamedVariant {
                                            name: "hkbProjectData".into(),
                                            class_name: "hkbProjectData".into(),
                                            variant: "#0051".into(),
                                        },
                                    }]
                                    .to_vec(),
                                },
                            },
                        )),
                    },
                    Class {
                        name: 51.into(),
                        class: "hkbProjectData".into(),
                        signature: 0x13a39ba7.into(),
                        hkparams: ClassParams::HkbProjectData(Box::new(HkbProjectData {
                            mem_size_and_flags: 0,
                            reference_count: 0,
                            world_up_ws: Vector4 {
                                x: OrderedFloat(0.0),
                                y: OrderedFloat(0.0),
                                z: OrderedFloat(1.0),
                                w: OrderedFloat(0.0),
                            },
                            string_data: "#0052".into(),
                            default_event_mode: EventMode::EventModeIgnoreFromGenerator,
                        })),
                    },
                    Class {
                        name: 52.into(),
                        class: "hkbProjectStringData".into(),
                        signature: 0x076ad60a.into(),
                        hkparams: ClassParams::HkbProjectStringData(Box::new(
                            HkbProjectStringData {
                                character_filenames: HkArrayStringPtr {
                                    numelements: 1,
                                    hkcstrings: ["Characters\\DefaultMale.hkx".into()].to_vec(),
                                },
                                ..Default::default()
                            },
                        )),
                    },
                ]
                .to_vec(),
            },
        };

        tracing::debug!("{:#?}", &result);
        assert_eq!(result, expected);
        Ok(())
    }
}
