//! Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
//! - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
use std::str::FromStr;

bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FlagValues: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;

        const FLAGS_NONE = 0;
        /// Member has forced 8 byte alignment. 1 << 7
        const ALIGN_8 = 128;
        /// Member has forced 16 byte alignment. 1 << 8
        const ALIGN_16 = 256;
        /// The members memory contents is not owned by this object 1 << 9
        const NOT_OWNED = 512;
        /// This member should not be written when serializing 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl Default for FlagValues {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for FlagValues {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for FlagValues {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for FlagValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> serde::Deserialize<'de> for FlagValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => match FlagValues::from_str(&s) {
                Ok(flags) => Ok(flags),
                Err(err) => Err(serde::de::Error::custom(err)),
            },
            None => Ok(Self::NULL),
        }
    }
}

impl FromStr for FlagValues {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(FlagValues::NULL);
        }

        let mut flags = FlagValues::empty();
        for token in s.split('|') {
            match token.trim() {
                "FLAGS_NONE" => flags |= FlagValues::FLAGS_NONE,
                "ALIGN_8" => flags |= FlagValues::ALIGN_8,
                "ALIGN_16" => flags |= FlagValues::ALIGN_16,
                "NOT_OWNED" => flags |= FlagValues::NOT_OWNED,
                "SERIALIZE_IGNORED" => flags |= FlagValues::SERIALIZE_IGNORED,
                unknown => match parse_int::parse(unknown) {
                    Ok(int) => {
                        if let Some(bits) = FlagValues::from_bits(int) {
                            flags |= bits
                        } else {
                            return Err(format!("Expected FlagValues flags but got '{}'", unknown));
                        };
                    }
                    Err(_) => {
                        return Err(format!(
                            "Expected FlagValues flags or integer, but got '{}'",
                            unknown
                        ))
                    }
                },
            }
        }
        Ok(flags)
    }
}

impl core::fmt::Display for FlagValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.human_readable())
    }
}

impl FlagValues {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAGS_NONE) {
            flags.push("FLAGS_NONE");
        }
        if self.contains(Self::ALIGN_8) {
            flags.push("ALIGN_8");
        }
        if self.contains(Self::ALIGN_16) {
            flags.push("ALIGN_16");
        }
        if self.contains(Self::NOT_OWNED) {
            flags.push("NOT_OWNED");
        }
        if self.contains(Self::SERIALIZE_IGNORED) {
            flags.push("SERIALIZE_IGNORED");
        }

        flags.join("|").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "hkparam")]
    struct TestRoot<T>(T);

    #[test]
    fn should_serialize_flags_null_to_0() {
        let flags = FlagValues::NULL;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(serialized, "<hkparam>0</hkparam>");
    }

    #[test]
    fn should_serialize_flags_none() {
        let flags = FlagValues::FLAGS_NONE;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(serialized, "<hkparam>FLAGS_NONE</hkparam>");
    }

    #[test]
    fn should_serialize_multiple_flags() {
        let flags = FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(
            serialized,
            "<hkparam>ALIGN_8|ALIGN_16|SERIALIZE_IGNORED</hkparam>"
        );
    }

    #[test]
    fn should_deserialize_0_to_flags_null() {
        let deserialized: FlagValues = from_str("<hkparam>0</hkparam>").unwrap();
        assert_eq!(deserialized, FlagValues::NULL);
    }

    #[test]
    fn should_deserialize_single_str_to_flags_none() {
        let deserialized: FlagValues = from_str("<hkparam>FLAGS_NONE</hkparam>").unwrap();
        assert_eq!(deserialized, FlagValues::FLAGS_NONE);
    }

    #[test]
    fn should_deserialize_str_flags_some() {
        let deserialized: FlagValues =
            from_str("<hkparam>ALIGN_8|ALIGN_16|SERIALIZE_IGNORED</hkparam>").unwrap();
        let expected = FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED;
        assert_eq!(deserialized, expected);
    }
}
