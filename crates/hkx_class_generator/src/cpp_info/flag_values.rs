//! Flags for field alignment needs, skipping serialization, etc.

bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_with::SerializeDisplay, serde_with::DeserializeFromStr)]
    #[repr(transparent)]
    pub struct FlagValues: u16 {
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

    /// Does it contain the `ALIGN_8` flag?
    #[inline]
    pub const fn has_align8(&self) -> bool {
        self.contains(Self::ALIGN_8)
    }

    /// Does it contain the `ALIGN_16` flag?
    #[inline]
    pub const fn has_align16(&self) -> bool {
        self.contains(Self::ALIGN_16)
    }
}

impl Default for FlagValues {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl core::fmt::Display for FlagValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            return write!(f, "0");
        }

        let mut flags: Vec<std::borrow::Cow<'_, str>> = Vec::new();
        for flag in self.iter() {
            match flag {
                FlagValues::FLAGS_NONE => flags.push("FLAGS_NONE".into()),
                FlagValues::ALIGN_8 => flags.push("ALIGN_8".into()),
                FlagValues::ALIGN_16 => flags.push("ALIGN_16".into()),
                FlagValues::NOT_OWNED => flags.push("NOT_OWNED".into()),
                FlagValues::SERIALIZE_IGNORED => flags.push("SERIALIZE_IGNORED".into()),
                remain => flags.push(remain.bits().to_string().into()),
            };
        }

        write!(f, "{}", flags.join("|"))
    }
}

impl core::str::FromStr for FlagValues {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(FlagValues::empty());
        }

        let mut flags = FlagValues::empty();
        for token in s.split('|') {
            let token = token.trim();

            // XML コメント処理
            let token = if let Some(start) = token.find("<!--") {
                if let Some(end) = token[start..].find("-->") {
                    let before_comment = token[..start].trim();
                    let after_comment = token[(start + end + 3)..].trim();
                    if !before_comment.is_empty() {
                        before_comment
                    } else {
                        after_comment
                    }
                } else {
                    token[..start].trim()
                }
            } else {
                token
            };

            if token.is_empty() {
                continue;
            }

            match token {
                "FLAGS_NONE" => flags |= FlagValues::FLAGS_NONE,
                "ALIGN_8" => flags |= FlagValues::ALIGN_8,
                "ALIGN_16" => flags |= FlagValues::ALIGN_16,
                "NOT_OWNED" => flags |= FlagValues::NOT_OWNED,
                "SERIALIZE_IGNORED" => flags |= FlagValues::SERIALIZE_IGNORED,
                unknown => {
                    let bits = parse_int::parse(unknown)
                        .map_err(|_| format!("Invalid FlagValues: '{unknown}'"))?;
                    flags |= FlagValues::from_bits_retain(bits);
                }
            }
        }

        Ok(flags)
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
            Ok(FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED),
        );

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|64".parse(),
            Ok(FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
        );

        assert_eq!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|<!-- UNKNOWN BITS -->64".parse(),
            Ok(FlagValues::ALIGN_8
                | FlagValues::ALIGN_16
                | FlagValues::SERIALIZE_IGNORED
                | FlagValues::from_bits_retain(64)),
        );

        assert!(
            "ALIGN_8|ALIGN_16|SERIALIZE_IGNORED|<!-- UNKNOWN BITS -->64<!-- UNKNOWN BITS -->"
                .parse::<FlagValues>()
                .is_err(),
        );
    }
}
