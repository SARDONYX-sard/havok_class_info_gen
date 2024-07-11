//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRoleAttribute`
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

/// `hkbRoleAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x3eb2e082`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbRoleAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"role"`
    /// -   type: `enum Role`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub role: Role,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags RoleFlags`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub flags: RoleFlags,
}

impl Serialize for HkbRoleAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbRoleAttributeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbRoleAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbRoleAttributeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbRoleAttributeVisitor>> for HkbRoleAttribute {
    fn from(_values: Vec<HkbRoleAttributeVisitor>) -> Self {
            let mut role = None;
            let mut flags = None;


        for _value in _values {
            match _value {
                HkbRoleAttributeVisitor::Role(m) => role = Some(m),
                HkbRoleAttributeVisitor::Flags(m) => flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            role: role.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbRoleAttribute> for Vec<HkbRoleAttributeVisitor> {
    fn from(data: &HkbRoleAttribute) -> Self {
        vec![
            HkbRoleAttributeVisitor::Role(data.role.clone().into()),
            HkbRoleAttributeVisitor::Flags(data.flags.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbRoleAttribute {
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
enum HkbRoleAttributeVisitor {
    /// Visitor fields
    #[serde(rename = "role")]
    Role(Primitive<Role>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<RoleFlags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRoleAttributeVisitor, "@name",
    ("role" => Role(Primitive<Role>)),
    ("flags" => Flags(Primitive<RoleFlags>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Role {
    #[serde(rename = "ROLE_DEFAULT")]
    #[default]
    RoleDefault = 0,
    #[serde(rename = "ROLE_FILE_NAME")]
    RoleFileName = 1,
    #[serde(rename = "ROLE_BONE_INDEX")]
    RoleBoneIndex = 2,
    #[serde(rename = "ROLE_BONE_INDEX_MAP")]
    RoleBoneIndexMap = 3,
    #[serde(rename = "ROLE_EVENT_ID")]
    RoleEventId = 4,
    #[serde(rename = "ROLE_VARIABLE_INDEX")]
    RoleVariableIndex = 5,
    #[serde(rename = "ROLE_ATTRIBUTE_INDEX")]
    RoleAttributeIndex = 6,
    #[serde(rename = "ROLE_TIME")]
    RoleTime = 7,
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RoleFlags: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_NONE = 0;
        const FLAG_RAGDOLL = 1;
        const FLAG_NORMALIZED = 2;
        const FLAG_NOT_VARIABLE = 4;
        const FLAG_HIDDEN = 8;
        const FLAG_OUTPUT = 16;
        const FLAG_NOT_CHARACTER_PROPERTY = 32;
    }
}

impl Default for RoleFlags {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for RoleFlags {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for RoleFlags {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for RoleFlags {
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

impl<'de> serde::Deserialize<'de> for RoleFlags {
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
                        "FLAG_NONE" => flags |= Self::FLAG_NONE,
                        "FLAG_RAGDOLL" => flags |= Self::FLAG_RAGDOLL,
                        "FLAG_NORMALIZED" => flags |= Self::FLAG_NORMALIZED,
                        "FLAG_NOT_VARIABLE" => flags |= Self::FLAG_NOT_VARIABLE,
                        "FLAG_HIDDEN" => flags |= Self::FLAG_HIDDEN,
                        "FLAG_OUTPUT" => flags |= Self::FLAG_OUTPUT,
                        "FLAG_NOT_CHARACTER_PROPERTY" => flags |= Self::FLAG_NOT_CHARACTER_PROPERTY,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected RoleFlags flags but got '{unknown}'",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected RoleFlags flags or integer, but got '{unknown}'"
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

impl core::fmt::Display for RoleFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl RoleFlags {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_NONE) {
            flags.push("FLAG_NONE");
        }
        if self.contains(Self::FLAG_RAGDOLL) {
            flags.push("FLAG_RAGDOLL");
        }
        if self.contains(Self::FLAG_NORMALIZED) {
            flags.push("FLAG_NORMALIZED");
        }
        if self.contains(Self::FLAG_NOT_VARIABLE) {
            flags.push("FLAG_NOT_VARIABLE");
        }
        if self.contains(Self::FLAG_HIDDEN) {
            flags.push("FLAG_HIDDEN");
        }
        if self.contains(Self::FLAG_OUTPUT) {
            flags.push("FLAG_OUTPUT");
        }
        if self.contains(Self::FLAG_NOT_CHARACTER_PROPERTY) {
            flags.push("FLAG_NOT_CHARACTER_PROPERTY");
        }

        flags.join("|").into()
    }
}
