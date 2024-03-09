//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValue`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::havok_types::Primitive;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 4
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableValue<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableValue"`: Name of C++ class.
    #[serde(default = "HkbVariableValue::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb99bd6a`: Unique value of this class.
    #[serde(default = "HkbVariableValue::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(rename = "hkparam")]
    pub hkparams: HkbVariableValueHkParam,
}

impl HkbVariableValue<'_> {
    /// Return `"hkbVariableValue"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbVariableValue".into()
    }

    /// Return `"0xb99bd6a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb99bd6a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableValueHkParam {
    /// # Information on fields in the original C++ class
    /// -   name:`"value"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<i32>),
}

impl Default for HkbVariableValueHkParam {
    fn default() -> Self {
        Self::Value(0.into())
    }
}

impl From<i32> for HkbVariableValueHkParam {
    fn from(value: i32) -> Self {
        Self::Value(value.into())
    }
}

impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueHkParam, "@name",
    ("value"=> Value(Primitive<i32>))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = HkbVariableValue {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: HkbVariableValueHkParam::Value(1045220557.into()),
        };

        let result = quick_xml::se::to_string(&class).unwrap();

        let expected_xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        assert_eq!(result, expected_xml);
        dbg!(result);
    }

    #[test]
    fn should_deserialize() {
        let xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        let result: HkbVariableValue = quick_xml::de::from_str(xml).unwrap();
        let expected = HkbVariableValue {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: HkbVariableValueHkParam::Value(1045220557.into()),
        };
        assert_eq!(result, expected);
    }
}
