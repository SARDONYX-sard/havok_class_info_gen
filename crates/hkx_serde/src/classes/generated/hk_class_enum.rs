//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkClassEnum`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkClassEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0x8a3609cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkClassEnum<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"items"`
    /// -   type: `hkSimpleArray<struct hkClassEnumItem>`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub items: HkArrayClass<HkClassEnumItem<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub attributes: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub flags: FlagValues,
}

impl Serialize for HkClassEnum<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkClassEnumVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkClassEnum<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkClassEnumVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkClassEnumVisitor<'a>>> for HkClassEnum<'a> {
    fn from(_values: Vec<HkClassEnumVisitor<'a>>) -> Self {
            let mut name = None;
            let mut items = None;
            let mut attributes = None;
            let mut flags = None;


        for _value in _values {
            match _value {
                HkClassEnumVisitor::Name(m) => name = Some(m),
                HkClassEnumVisitor::Items(m) => items = Some(m),
                HkClassEnumVisitor::Attributes(m) => attributes = Some(m),
                HkClassEnumVisitor::Flags(m) => flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            name: name.unwrap_or_default().into_inner(),
            items: items.unwrap_or_default(),
            attributes: attributes.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkClassEnum<'a>> for Vec<HkClassEnumVisitor<'a>> {
    fn from(data: &HkClassEnum<'a>) -> Self {
        vec![
            HkClassEnumVisitor::Name(data.name.clone().into()),
            HkClassEnumVisitor::Items(data.items.clone()),
            HkClassEnumVisitor::Attributes(data.attributes.clone().into()),
            HkClassEnumVisitor::Flags(data.flags.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkClassEnum<'de> {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkClassEnumVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "items")]
    Items(HkArrayClass<HkClassEnumItem<'a>>),
    /// Visitor fields
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<FlagValues>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkClassEnumVisitor<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("items" => Items(HkArrayClass<HkClassEnumItem<'de>>)),
    ("attributes" => Attributes(Primitive<Cow<'de, str>>)),
    ("flags" => Flags(Primitive<FlagValues>)),
}
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
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(Self::NULL);
                };
                let mut flags = Self::empty();
                for token in s.split('|') {
                    match token.trim() {
                        "FLAGS_NONE" => flags |= Self::FLAGS_NONE,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected FlagValues flags but got '{unknown}'",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected FlagValues flags or integer, but got '{unknown}'"
                                )))
                            }
                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}

impl core::fmt::Display for FlagValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
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

        flags.join("|").into()
    }
}
