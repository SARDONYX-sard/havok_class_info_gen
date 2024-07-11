use serde::{Deserialize, Serialize};

/// # Vector of Class in Class field
/// - In C++, this is the case when the type of the in-class field is a class.
/// - In XML, A type to hold another class of `hkparam` as an array within `hkparam`.
///
/// e.g. `wordVariableValues` field of `hkbVariableValueSet` class
///
/// # XML Example
/// ```xml
/// <hkparam name="variantVariableValues" numelements="2">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
///     <hkobject>
///         <hkparam name="another_class_field">#0064</hkparam>
///     </hkobject>
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayClass<T> {
    /// Length of the class
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// An array that receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    ///
    /// # Note
    /// [document](https://docs.rs/quick-xml/latest/quick_xml/de/index.html#sequences-xsall-and-xssequence-xml-schema-types) in quick_xml.
    /// If we try to use `#[serde(default)` for a enum(Represent C++ class), we will get stuck with the Default implementation,
    /// so you can get away with not serializing [`Option`].
    ///
    /// If there is no skip, an extra one is created like `<hkparam><hkobject /></hkparam>`.
    #[serde(rename = "hkobject")]
    #[serde(default)]
    pub classes: Vec<HkArrayClassParam<T>>,
}

impl<T> Default for HkArrayClass<T> {
    fn default() -> Self {
        Self {
            numelements: 0,
            classes: Vec::new(),
        }
    }
}

impl<T> HkArrayClass<T> {
    /// Take inner value.
    pub fn into_inner(self) -> Vec<T> {
        self.classes
            .into_iter()
            .map(|param_wrapper| param_wrapper.into_inner())
            .collect()
    }
}

impl<T> From<Vec<HkArrayClassParam<T>>> for HkArrayClass<T> {
    fn from(classes: Vec<HkArrayClassParam<T>>) -> Self {
        Self {
            numelements: classes.len(),
            classes,
        }
    }
}

impl<T> From<Vec<T>> for HkArrayClass<T> {
    fn from(classes: Vec<T>) -> Self {
        Self {
            numelements: classes.len(),
            classes: classes.into_iter().map(HkArrayClassParam::from).collect(),
        }
    }
}

/// A field wrapper of a class in `HkArray`
///
/// ```xml
/// <hkobject>
///     <hkparam>#0063</hkparam> <!-- This lines -->
///     <hkparam>#0064</hkparam> <!-- This lines -->
/// </hkobject>
/// ````
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HkArrayClassParam<T> {
    /// A field of the class.
    ///
    /// In XML `hkparam`
    #[serde(rename = "hkparam")]
    pub hkparam: T,
}

impl<T> HkArrayClassParam<T> {
    /// Takes inner hkparam value from wrapper.
    pub fn into_inner(self) -> T {
        self.hkparam
    }
}

impl<T> From<T> for HkArrayClassParam<T> {
    fn from(value: T) -> Self {
        Self { hkparam: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: HkArrayClass<Vec<i32>> = vec![vec![1045220557], vec![0]].into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"2\">\
                <hkobject>\
                    <hkparam>\
                        1045220557\
                    </hkparam>\
                </hkobject>\
                <hkobject>\
                    <hkparam>\
                        0\
                    </hkparam>\
                </hkobject>\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                <hkobject>
                    <hkparam>#0063</hkparam>
                    <hkparam>#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam>#0064</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: HkArrayClass<Vec<&str>> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayClass {
            numelements: 2,
            classes: vec![
                HkArrayClassParam {
                    hkparam: vec!["#0063", "#0063"],
                },
                HkArrayClassParam {
                    hkparam: vec!["#0064"],
                },
            ],
        };

        assert_eq!(deserialized, expected);
    }
}
