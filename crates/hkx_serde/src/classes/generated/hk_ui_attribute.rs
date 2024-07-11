//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkUiAttribute`
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

/// `hkUiAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xeb6e96e3`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkUiAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub visible: bool,
    /// # C++ Class Fields Info
    /// -   name:`"hideInModeler"`
    /// -   type: `enum HideInModeler`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub hide_in_modeler: HideInModeler,
    /// # C++ Class Fields Info
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub label: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"group"`
    /// -   type: `char*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub group: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"hideBaseClassMembers"`
    /// -   type: `char*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub hide_base_class_members: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"endGroup"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub end_group: bool,
    /// # C++ Class Fields Info
    /// -   name:`"endGroup2"`
    /// -   type: `hkBool`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    pub end_group_2: bool,
    /// # C++ Class Fields Info
    /// -   name:`"advanced"`
    /// -   type: `hkBool`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    pub advanced: bool,
}

impl Serialize for HkUiAttribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkUiAttributeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkUiAttribute<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkUiAttributeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkUiAttributeVisitor<'a>>> for HkUiAttribute<'a> {
    fn from(_values: Vec<HkUiAttributeVisitor<'a>>) -> Self {
            let mut visible = None;
            let mut hide_in_modeler = None;
            let mut label = None;
            let mut group = None;
            let mut hide_base_class_members = None;
            let mut end_group = None;
            let mut end_group_2 = None;
            let mut advanced = None;


        for _value in _values {
            match _value {
                HkUiAttributeVisitor::Visible(m) => visible = Some(m),
                HkUiAttributeVisitor::HideInModeler(m) => hide_in_modeler = Some(m),
                HkUiAttributeVisitor::Label(m) => label = Some(m),
                HkUiAttributeVisitor::Group(m) => group = Some(m),
                HkUiAttributeVisitor::HideBaseClassMembers(m) => hide_base_class_members = Some(m),
                HkUiAttributeVisitor::EndGroup(m) => end_group = Some(m),
                HkUiAttributeVisitor::EndGroup2(m) => end_group_2 = Some(m),
                HkUiAttributeVisitor::Advanced(m) => advanced = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            visible: visible.unwrap_or_default().into_inner(),
            hide_in_modeler: hide_in_modeler.unwrap_or_default().into_inner(),
            label: label.unwrap_or_default().into_inner(),
            group: group.unwrap_or_default().into_inner(),
            hide_base_class_members: hide_base_class_members.unwrap_or_default().into_inner(),
            end_group: end_group.unwrap_or_default().into_inner(),
            end_group_2: end_group_2.unwrap_or_default().into_inner(),
            advanced: advanced.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkUiAttribute<'a>> for Vec<HkUiAttributeVisitor<'a>> {
    fn from(data: &HkUiAttribute<'a>) -> Self {
        vec![
            HkUiAttributeVisitor::Visible(data.visible.into()),
            HkUiAttributeVisitor::HideInModeler(data.hide_in_modeler.clone().into()),
            HkUiAttributeVisitor::Label(data.label.clone().into()),
            HkUiAttributeVisitor::Group(data.group.clone().into()),
            HkUiAttributeVisitor::HideBaseClassMembers(data.hide_base_class_members.clone().into()),
            HkUiAttributeVisitor::EndGroup(data.end_group.into()),
            HkUiAttributeVisitor::EndGroup2(data.end_group_2.into()),
            HkUiAttributeVisitor::Advanced(data.advanced.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkUiAttribute<'de> {
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
enum HkUiAttributeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "hideInModeler")]
    HideInModeler(Primitive<HideInModeler>),
    /// Visitor fields
    #[serde(rename = "label")]
    Label(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "group")]
    Group(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "hideBaseClassMembers")]
    HideBaseClassMembers(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "endGroup")]
    EndGroup(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "endGroup2")]
    EndGroup2(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "advanced")]
    Advanced(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkUiAttributeVisitor<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("hideInModeler" => HideInModeler(Primitive<HideInModeler>)),
    ("label" => Label(Primitive<Cow<'de, str>>)),
    ("group" => Group(Primitive<Cow<'de, str>>)),
    ("hideBaseClassMembers" => HideBaseClassMembers(Primitive<Cow<'de, str>>)),
    ("endGroup" => EndGroup(Primitive<bool>)),
    ("endGroup2" => EndGroup2(Primitive<bool>)),
    ("advanced" => Advanced(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum HideInModeler {
    #[serde(rename = "NONE")]
    #[default]
    None = 0,
    #[serde(rename = "MAX")]
    Max = 1,
    #[serde(rename = "MAYA")]
    Maya = 2,
}
