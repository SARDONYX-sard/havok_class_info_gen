//! If you use the `HkArray` implementation as is, the serialization will include `numelements` attribute.
//! If I set it to [`Option`] and branch the process, I don't know if it is an error or not.
//! Therefore, the code is almost the same, but the code is divided.
use serde::{Deserialize, Serialize};

/// In C++, this is the case when the type of the in-class field is a class.
///
/// In XML, A type to hold another class of `hkparam` as an array within `hkparam`.
///
/// # XML Example
/// ```xml
/// <hkparam name="variantVariableValues">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
///     <hkobject>
///         <hkparam name="another_class_field">#0064</hkparam>
///     </hkobject>
/// </hkparam>
/// ```
///
/// This is almost the same as for `hkArrayClass`, but with the following differences.
/// - There is no `numelements` attribute.
/// - The number of elements is limited.
///
/// # Error
/// - If there is an extra array.(e.g. if `[i32; 10]` is gotten when `[i32; 5]` is expected).
///
/// # Note
/// - In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
///
///   The `name` attribute is required for `hkparam` but is not included in this structure.
///   This is because the value of the `name` attribute corresponds to a C++ field name,
///   and the processing must be changed according to the value.
///   And to do that, we need the parent enum that wraps this structure.
pub type CStyleArrayClass<T, const N: usize> = CStyleArrayClassT<[CStyleArrayClassParam<T>; N]>;

impl<T, const N: usize> CStyleArrayClass<T, N>
where
    T: core::fmt::Debug,
{
    /// Take value from wrapper
    pub fn into_inner(self) -> [T; N] {
        let t = self
            .classes
            .into_iter()
            .map(|param| param.into_inner())
            .collect::<Vec<_>>();
        t.try_into().unwrap()
    }
}

impl<T, const N: usize> From<[T; N]> for CStyleArrayClass<T, N>
where
    T: core::fmt::Debug,
{
    fn from(classes: [T; N]) -> Self {
        let classes = classes
            .into_iter()
            .map(|value| CStyleArrayClassParam::from(value))
            .collect::<Vec<_>>();
        CStyleArrayClass {
            classes: classes.try_into().unwrap(),
        }
    }
}

/// # Use the associated type.
/// The reason for using associated types is that trying to hold `[T; N]` in `classes` results in a trait bound error.
///
/// Therefore, the associated type is used to avoid this error.
/// This is an internal part of the `classes`, and since `From` is also implemented, there is no need to call it directly.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct CStyleArrayClassT<T>
where
    T: core::fmt::Debug,
{
    /// An array that receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    #[serde(rename = "hkobject")]
    pub classes: T,
}

/// A field(`hkparam`) wrapper of C++ Class(`hkobject`)
///
/// ```xml
/// <hkobject>
///     <hkparam>#0063</hkparam> <!-- This line -->
/// </hkobject>
/// ````
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CStyleArrayClassParam<T> {
    /// A field of the class.
    ///
    /// In XML `hkparam`
    #[serde(rename = "hkparam")]
    pub hkparam: T,
}

impl<T> CStyleArrayClassParam<T> {
    /// Take inner hkparam
    fn into_inner(self) -> T {
        self.hkparam
    }
}

impl<T> From<T> for CStyleArrayClassParam<T> {
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
        let data: CStyleArrayClass<i32, 2> = [1045220557, 0].into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam>\
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
            <hkparam name="variantVariableValues">
                <hkobject>
                    <hkparam name="class_field">#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam name="another_class_field">#0064</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: CStyleArrayClass<&str, 2> = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(deserialized, ["#0063", "#0064"].into());
    }

    #[test]
    fn should_fail_deserialize_over_limit_elements() {
        let xml = r###"
            <hkparam name="variantVariableValues">
                <hkobject>
                    <hkparam name="class_field">#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam name="another_class_field">#0064</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam>#0065</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let result: Result<CStyleArrayClass<&str, 2>, _> = quick_xml::de::from_str(xml);

        assert!(result.is_err());
    }
}
