use serde::{Deserialize, Serialize};

/// Size-limited C Style arrays and for performing normal (De)serialization.
///
/// # Usage
///
/// `CStyleArray<[u32; 8]>`, etc.
///
/// # Example XML
/// `padding`(`hkUint[8]`) in `hkpSetupStabilizationAtom`
/// ```xml
/// <hkparam name="padding">0 0 0 0 0 0 0 0</hkparam>
/// ```
///
/// # Note
/// - Overflow in a given array is ignored.(e.g. passed `hkUint[10]` => `hkUint[8]`)
/// - In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
///
///   The `name` attribute is required for `hkparam` but is not included in this structure.
///   This is because the value of the `name` attribute corresponds to a C++ field name,
///   and the processing must be changed according to the value.
///   And to do that, we need the parent enum that wraps this structure.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct CStyleArray<T> {
    /// Limit Array which stores [`[u32; 4`], etc.
    #[serde(rename = "$text", default)]
    pub value: T,
}

impl<T> CStyleArray<T> {
    /// Take inner value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> From<T> for CStyleArray<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data = CStyleArray {
            value: [0, 1, 2, 3, 4, 5, 6, 7],
        };
        let serialized = quick_xml::se::to_string(&data).unwrap();

        assert_eq!(serialized, "<hkparam>0 1 2 3 4 5 6 7</hkparam>");
    }

    #[test]
    fn should_deserialize() {
        let xml = " <hkparam name=\"padding\">0 1 2 3 4 5 6 7 8</hkparam>";
        let deserialized: CStyleArray<[i32; 8]> = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(deserialized, [0, 1, 2, 3, 4, 5, 6, 7].into());
    }
}
