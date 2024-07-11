use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// String Array for havok
///
/// Used in: `hkbBehaviorGraphStringData` class `eventNames` field, etc.
///
/// # XML Example
///
/// ```xml
/// <hkparam name="variableNames" numelements="6">
///   <hkcstring>blendDefault</hkcstring>
///   <hkcstring>blendFast</hkcstring>
///   <hkcstring>blendSlow</hkcstring>
///   <hkcstring>Direction</hkcstring>
///   <hkcstring>IsBlocking</hkcstring>
///   <hkcstring>Speed</hkcstring>
/// </hkparam>
/// ```
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value. And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayStringPtr<'a> {
    /// Length of the pointer array stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// In XMl, a vector of strings enclosed in `hkcstring` tags
    #[serde(default, borrow)]
    #[serde(rename = "hkcstring")]
    pub hkcstrings: Vec<Cow<'a, str>>,
}

impl<'a> HkArrayStringPtr<'a> {
    /// Take inner value.
    pub fn into_inner(self) -> Vec<Cow<'a, str>> {
        self.hkcstrings
    }
}

impl<'a> From<Vec<&'a str>> for HkArrayStringPtr<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        Self {
            numelements: value.len(),
            hkcstrings: value.into_iter().map(Cow::Borrowed).collect(),
        }
    }
}

impl<'a> From<Vec<String>> for HkArrayStringPtr<'a> {
    fn from(value: Vec<String>) -> Self {
        Self {
            numelements: value.len(),
            hkcstrings: value.into_iter().map(Cow::Owned).collect(),
        }
    }
}

impl<'a> From<Vec<Cow<'a, str>>> for HkArrayStringPtr<'a> {
    fn from(value: Vec<Cow<'a, str>>) -> Self {
        Self {
            numelements: value.len(),
            hkcstrings: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: HkArrayStringPtr = vec![
            "blendDefault",
            "blendFast",
            "blendSlow",
            "Direction",
            "IsBlocking",
            "Speed",
        ]
        .into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"6\">\
              <hkcstring>blendDefault</hkcstring>\
              <hkcstring>blendFast</hkcstring>\
              <hkcstring>blendSlow</hkcstring>\
              <hkcstring>Direction</hkcstring>\
              <hkcstring>IsBlocking</hkcstring>\
              <hkcstring>Speed</hkcstring>\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variableNames" numelements="6">
              <hkcstring>blendDefault</hkcstring>
              <hkcstring>blendFast</hkcstring>
              <hkcstring>blendSlow</hkcstring>
              <hkcstring>Direction</hkcstring>
              <hkcstring>IsBlocking</hkcstring>
              <hkcstring>Speed</hkcstring>
            </hkparam>
        "###;
        let deserialized: HkArrayStringPtr = quick_xml::de::from_str(xml).unwrap();

        let expected: HkArrayStringPtr = vec![
            "blendDefault",
            "blendFast",
            "blendSlow",
            "Direction",
            "IsBlocking",
            "Speed",
        ]
        .into();
        assert_eq!(deserialized, expected);
    }
}
