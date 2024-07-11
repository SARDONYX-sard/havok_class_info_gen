//! Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
//! - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
use crate::hkxcmd_parser::{Enum, EnumItem};

/// Generate u16 bit flags Rust code
pub fn generate_bitflags(enum_info: &Enum) -> String {
    let mut rust_code = String::new();
    let Enum {
        name: enum_name,
        enum_item: enum_pair_info,
        ..
    } = enum_info;

    rust_code.push_str(&format!(
        r#"bitflags::bitflags! {{
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct {enum_name}: u16 {{
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
"#
    ));

    for EnumItem {
        name: enum_tag,
        value: enum_value,
    } in enum_pair_info
    {
        rust_code.push_str(&format!("        const {enum_tag} = {enum_value};\n"));
    }

    rust_code.push_str(&format!(
        r#"    }}
}}

impl Default for {enum_name} {{
    fn default() -> Self {{
        Self::NULL
    }}
}}

impl TryFrom<usize> for {enum_name} {{
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {{
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {{value}}"))
    }}
}}

impl TryFrom<u16> for {enum_name} {{
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {{
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {{value}}"))
    }}
}}

impl serde::Serialize for {enum_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        if self.contains(Self::NULL) {{
            serializer.serialize_str("0")
        }} else {{
            serializer.serialize_str(&self.human_readable())
        }}
    }}
}}

impl<'de> serde::Deserialize<'de> for {enum_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {{
            Some(s) => {{
                if s.as_ref() == "0" {{
                    return Ok(Self::NULL);
                }};
                let mut flags = Self::empty();
                for token in s.split('|') {{
                    match token.trim() {{
"#
    ));

    for EnumItem { name: enum_tag, .. } in enum_pair_info {
        rust_code.push_str(&format!(
            "                        \"{enum_tag}\" => flags |= Self::{enum_tag},\n"
        ));
    }

    rust_code.push_str(&format!(
        r#"                        unknown => match parse_int::parse(unknown) {{
                            Ok(int) => {{
                                if let Some(bits) = Self::from_bits(int) {{
                                    flags |= bits
                                }} else {{
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected {enum_name} flags but got '{{unknown}}'",
                                    )));
                                }};
                            }}
                            Err(_err) => {{
                                return Err(serde::de::Error::custom(format!(
                                    "Expected {enum_name} flags or integer, but got '{{unknown}}'"
                                )))
                            }}
"#
    ));

    rust_code.push_str(
        r#"                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}"#,
    );

    rust_code.push_str(&format!(
        r#"

impl core::fmt::Display for {enum_name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        write!(f, "{{}}", self.bits())
    }}
}}

impl {enum_name} {{
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {{
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {{
            return "0".into();
        }};
"#
    ));

    for EnumItem { name: enum_tag, .. } in enum_pair_info {
        rust_code.push_str(&format!(
            r#"
        if self.contains(Self::{enum_tag}) {{
            flags.push("{enum_tag}");
        }}"#
        ));
    }

    rust_code.push_str(
        r#"

        flags.join("|").into()
    }
}
"#,
    );

    rust_code
}
