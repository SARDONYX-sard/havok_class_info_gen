use num_traits::Num;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Number array.
///
/// In XML, it is just an array of space-delimited numbers.
///
/// # XML Example
/// ```xml
/// <hkparam name="boneWeights" numelements="99">
///   1.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
///   0.000000 0.000000 0.000000
/// </hkparam>
/// ```
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayNum<T: Num> {
    /// Length of the pointer array stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// A vector of number.
    ///
    /// In XML, it is just an array of space-delimited number.
    #[serde(rename = "$text", default)]
    pub values: Vec<T>,
}

impl<T: Num> HkArrayNum<T> {
    /// Take inner value.
    pub fn into_inner(self) -> Vec<T> {
        self.values
    }
}

// ! Quick_xml's `$text` made an error involving multiple numbers when taking a floating point mixed with tab.
// ! To avoid the error, implement the deserializer manually.
impl<'de, T> Deserialize<'de> for HkArrayNum<T>
where
    T: Deserialize<'de> + Num,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Inner<'a> {
            #[serde(rename = "@numelements")]
            numelements: usize,
            #[serde(rename = "$text", borrow, default)]
            value: Cow<'a, str>,
        }

        let inner = Inner::deserialize(deserializer)?;

        // Parse the space-delimited string into Vec<T>
        let numbers = inner
            .value
            .split_whitespace()
            .map(|s| parse_int::parse::<T>(s))
            .collect::<Result<Vec<T>, _>>()
            .map_err(|_err| serde::de::Error::custom("error"))?;

        let numbers_len = numbers.len();
        let mut numelements = inner.numelements;
        if numelements != numbers_len {
            tracing::warn!(
                "numelements attribute value != array length. Expected {numelements}, but got {numbers_len}. Auto fixed numelements to {numbers_len}"
            );
            numelements = numbers_len;
        }

        Ok(HkArrayNum {
            numelements,
            values: numbers,
        })
    }
}

impl<T: Num> From<Vec<T>> for HkArrayNum<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            numelements: values.len(),
            values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_deserialize_float_array() {
        let xml = r###"
			<hkparam name="boneWeights" numelements="99">
				1.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
				0.000000 0.000000 0.000000
			</hkparam>
        "###;
        let deserialized: HkArrayNum<f32> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayNum {
            numelements: 99,
            values: vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ],
        };

        assert_eq!(deserialized, expected);
    }
}
