//! Flags for field alignment needs, skipping serialization, etc.

bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FlagValues: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;

        /// Flags is empty: 0
        const FLAGS_NONE = 0;
        /// Force 8-byte align: 1 << 7
        const ALIGN_8 = 128;
        /// Forced 16-byte align: 1 << 8
        const ALIGN_16 = 256;
        /// Not owned by class: 1 << 9
        const NOT_OWNED = 512;
        /// Skip serializing: 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl FlagValues {
    /// Does it contain the `SERIALIZE_IGNORED` flag?
    #[inline]
    pub const fn has_skip_serializing(&self) -> bool {
        self.contains(Self::SERIALIZE_IGNORED)
    }

    /// Does it contain the `ALIGN8` flag?
    #[inline]
    pub const fn has_align8(&self) -> bool {
        self.contains(Self::ALIGN_8)
    }

    /// Does it contain the `ALIGN16` flag?
    #[inline]
    pub const fn has_align16(&self) -> bool {
        self.contains(Self::ALIGN_16)
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
            Some(s) => match <FlagValues as std::str::FromStr>::from_str(&s) {
                Ok(flags) => Ok(flags),
                Err(err) => Err(serde::de::Error::custom(err)),
            },
            None => Ok(Self::NULL),
        }
    }
}

impl core::str::FromStr for FlagValues {
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

    #[test]
    fn to_string() {
        assert_eq!(
            FlagValues::FLAGS_NONE.to_string(),
            "0",
            "If the bit is empty, `0` should be displayed"
        );

        assert_eq!(
            (FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED)
                .to_string(),
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED",
            "`FLAGS_NONE` will not be included unless it is manually included in the flag type."
        );
    }

    #[test]
    fn from_str() {
        assert_eq!("0".parse(), Ok(FlagValues::FLAGS_NONE));
        assert_eq!("FLAGS_NONE".parse(), Ok(FlagValues::FLAGS_NONE));

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED".parse(),
            Ok(FlagValues::FLAGS_NONE
                | FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED),
            "FLAGS_NONE must be held at any time."
        );

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|64".parse(),
            Ok(FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
            "Unknown flags should be able to accepted."
        );

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|<!-- UNKNOWN BITS -->64".parse(),
            Ok(FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
            "Comment with unknown flags should be able to accepted."
        );

        // Two or more comments within an item separated by `|` are currently unsupported.
        assert!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|<!-- UNKNOWN BITS -->64<!-- UNKNOWN BITS -->"
                .parse::<FlagValues>()
                .is_err(),
        );
    }
}
