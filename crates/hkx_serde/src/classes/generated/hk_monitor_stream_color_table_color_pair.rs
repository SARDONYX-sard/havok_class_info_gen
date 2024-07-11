//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMonitorStreamColorTableColorPair`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMonitorStreamColorTableColorPair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x738fca05`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamColorTableColorPair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"colorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "colorName")]
    ColorName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"color"`
    /// -   type: `enum ExtendedColors`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "color")]
    Color(Primitive<ExtendedColors>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamColorTableColorPair<'de>, "@name",
    ("colorName" => ColorName(Primitive<Cow<'de, str>>)),
    ("color" => Color(Primitive<ExtendedColors>)),
}
