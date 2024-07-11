#[cfg(test)]
mod tests {
    use crate::classes::generated::{class_params::ClassParams, HkbBehaviorGraphStringData};
    use crate::classes::Class;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Class {
            name: "#0085".into(),
            class: "hkbBehaviorGraphStringData".into(),
            signature: "0xc713064e".into(),
            hkparams: ClassParams::HkbBehaviorGraphStringData(Box::new(
                HkbBehaviorGraphStringData {
                    event_names: vec![
                        "cannedTurnRight90Flee",
                        "cannedTurnRight180Flee",
                        "cannedTurnLeft90Flee",
                        "cannedTurnLeft180Flee",
                    ]
                    .into(),
                    variable_names: vec![
                        "blendDefault",
                        "blendFast",
                        "blendSlow",
                        "Direction",
                        "IsBlocking",
                        "Speed",
                    ]
                    .into(),
                    ..Default::default()
                },
            )),
        };
        let serialized = quick_xml::se::to_string(&class).unwrap();

        let expected = "\
<Class name=\"#0085\" class=\"hkbBehaviorGraphStringData\" signature=\"0xc713064e\">\
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
</Class>\
";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize() {
        let xml_str = "\
<Class name=\"#0085\" class=\"hkbBehaviorGraphStringData\" signature=\"0xc713064e\">\
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
</Class>\
";
        let deserialized: Class = quick_xml::de::from_str(xml_str).unwrap();

        let expected = Class {
            name: "#0085".into(),
            class: "hkbBehaviorGraphStringData".into(),
            signature: "0xc713064e".into(),
            hkparams: ClassParams::HkbBehaviorGraphStringData(Box::new(
                HkbBehaviorGraphStringData {
                    event_names: vec![
                        "cannedTurnRight90Flee",
                        "cannedTurnRight180Flee",
                        "cannedTurnLeft90Flee",
                        "cannedTurnLeft180Flee",
                    ]
                    .into(),
                    variable_names: vec![
                        "blendDefault",
                        "blendFast",
                        "blendSlow",
                        "Direction",
                        "IsBlocking",
                        "Speed",
                    ]
                    .into(),
                    ..Default::default()
                },
            )),
        };
        assert_eq!(deserialized, expected);
    }
}
