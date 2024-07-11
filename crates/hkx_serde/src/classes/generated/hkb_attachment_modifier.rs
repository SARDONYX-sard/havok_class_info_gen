//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttachmentModifier`
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

/// `hkbAttachmentModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 108
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xcc0aab32`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbAttachmentModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_modifier: CStyleArray<[bool; 3]>,

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: i16,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub clone_state: (),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_node: CStyleArray<[bool; 1]>,

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variable_binding_set: Cow<'a, str>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub cached_bindables: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub are_bindables_cached: bool,

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacherOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub send_to_attacher_on_attach: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacheeOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub send_to_attachee_on_attach: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacherOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub send_to_attacher_on_detach: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacheeOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub send_to_attachee_on_detach: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"attachmentSetup"`
    /// -   type: `struct hkbAttachmentSetup*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub attachment_setup: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"attacherHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub attacher_handle: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"attacheeHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub attachee_handle: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"attacheeLayer"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub attachee_layer: i32,
    /// # C++ Class Fields Info
    /// -   name:`"attacheeRB"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub attachee_rb: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"oldMotionType"`
    /// -   type: `enum unknown`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub old_motion_type: (),
    /// # C++ Class Fields Info
    /// -   name:`"oldFilterInfo"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub old_filter_info: i32,
    /// # C++ Class Fields Info
    /// -   name:`"attachment"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub attachment: Cow<'a, str>,
}

impl Serialize for HkbAttachmentModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbAttachmentModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbAttachmentModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbAttachmentModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbAttachmentModifierVisitor<'a>>> for HkbAttachmentModifier<'a> {
    fn from(_values: Vec<HkbAttachmentModifierVisitor<'a>>) -> Self {
            let mut enable = None;
            let mut pad_modifier = None;
            let mut user_data = None;
            let mut name = None;
            let mut id = None;
            let mut clone_state = None;
            let mut pad_node = None;
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut send_to_attacher_on_attach = None;
            let mut send_to_attachee_on_attach = None;
            let mut send_to_attacher_on_detach = None;
            let mut send_to_attachee_on_detach = None;
            let mut attachment_setup = None;
            let mut attacher_handle = None;
            let mut attachee_handle = None;
            let mut attachee_layer = None;
            let mut attachee_rb = None;
            let mut old_motion_type = None;
            let mut old_filter_info = None;
            let mut attachment = None;


        for _value in _values {
            match _value {
                HkbAttachmentModifierVisitor::Enable(m) => enable = Some(m),
                HkbAttachmentModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbAttachmentModifierVisitor::UserData(m) => user_data = Some(m),
                HkbAttachmentModifierVisitor::Name(m) => name = Some(m),
                HkbAttachmentModifierVisitor::Id(m) => id = Some(m),
                HkbAttachmentModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbAttachmentModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbAttachmentModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbAttachmentModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbAttachmentModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbAttachmentModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbAttachmentModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbAttachmentModifierVisitor::SendToAttacherOnAttach(m) => send_to_attacher_on_attach = Some(m),
                HkbAttachmentModifierVisitor::SendToAttacheeOnAttach(m) => send_to_attachee_on_attach = Some(m),
                HkbAttachmentModifierVisitor::SendToAttacherOnDetach(m) => send_to_attacher_on_detach = Some(m),
                HkbAttachmentModifierVisitor::SendToAttacheeOnDetach(m) => send_to_attachee_on_detach = Some(m),
                HkbAttachmentModifierVisitor::AttachmentSetup(m) => attachment_setup = Some(m),
                HkbAttachmentModifierVisitor::AttacherHandle(m) => attacher_handle = Some(m),
                HkbAttachmentModifierVisitor::AttacheeHandle(m) => attachee_handle = Some(m),
                HkbAttachmentModifierVisitor::AttacheeLayer(m) => attachee_layer = Some(m),
                HkbAttachmentModifierVisitor::AttacheeRb(m) => attachee_rb = Some(m),
                HkbAttachmentModifierVisitor::OldMotionType(m) => old_motion_type = Some(m),
                HkbAttachmentModifierVisitor::OldFilterInfo(m) => old_filter_info = Some(m),
                HkbAttachmentModifierVisitor::Attachment(m) => attachment = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enable: enable.unwrap_or_default().into_inner(),
            pad_modifier: pad_modifier.unwrap_or_default(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),
            clone_state: clone_state.unwrap_or_default().into_inner(),
            pad_node: pad_node.unwrap_or_default(),
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            send_to_attacher_on_attach: send_to_attacher_on_attach.unwrap_or_default(),
            send_to_attachee_on_attach: send_to_attachee_on_attach.unwrap_or_default(),
            send_to_attacher_on_detach: send_to_attacher_on_detach.unwrap_or_default(),
            send_to_attachee_on_detach: send_to_attachee_on_detach.unwrap_or_default(),
            attachment_setup: attachment_setup.unwrap_or_default().into_inner(),
            attacher_handle: attacher_handle.unwrap_or_default().into_inner(),
            attachee_handle: attachee_handle.unwrap_or_default().into_inner(),
            attachee_layer: attachee_layer.unwrap_or_default().into_inner(),
            attachee_rb: attachee_rb.unwrap_or_default().into_inner(),
            old_motion_type: old_motion_type.unwrap_or_default().into_inner(),
            old_filter_info: old_filter_info.unwrap_or_default().into_inner(),
            attachment: attachment.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbAttachmentModifier<'a>> for Vec<HkbAttachmentModifierVisitor<'a>> {
    fn from(data: &HkbAttachmentModifier<'a>) -> Self {
        vec![
            HkbAttachmentModifierVisitor::Enable(data.enable.into()),
            HkbAttachmentModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbAttachmentModifierVisitor::UserData(data.user_data.into()),
            HkbAttachmentModifierVisitor::Name(data.name.clone().into()),
            HkbAttachmentModifierVisitor::Id(data.id.into()),
            HkbAttachmentModifierVisitor::CloneState(data.clone_state.into()),
            HkbAttachmentModifierVisitor::PadNode(data.pad_node.clone()),
            HkbAttachmentModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbAttachmentModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbAttachmentModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbAttachmentModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbAttachmentModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbAttachmentModifierVisitor::SendToAttacherOnAttach(data.send_to_attacher_on_attach.clone()),
            HkbAttachmentModifierVisitor::SendToAttacheeOnAttach(data.send_to_attachee_on_attach.clone()),
            HkbAttachmentModifierVisitor::SendToAttacherOnDetach(data.send_to_attacher_on_detach.clone()),
            HkbAttachmentModifierVisitor::SendToAttacheeOnDetach(data.send_to_attachee_on_detach.clone()),
            HkbAttachmentModifierVisitor::AttachmentSetup(data.attachment_setup.clone().into()),
            HkbAttachmentModifierVisitor::AttacherHandle(data.attacher_handle.clone().into()),
            HkbAttachmentModifierVisitor::AttacheeHandle(data.attachee_handle.clone().into()),
            HkbAttachmentModifierVisitor::AttacheeLayer(data.attachee_layer.into()),
            HkbAttachmentModifierVisitor::AttacheeRb(data.attachee_rb.clone().into()),
            HkbAttachmentModifierVisitor::OldMotionType(data.old_motion_type.into()),
            HkbAttachmentModifierVisitor::OldFilterInfo(data.old_filter_info.into()),
            HkbAttachmentModifierVisitor::Attachment(data.attachment.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbAttachmentModifier<'de> {
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
enum HkbAttachmentModifierVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// Visitor fields
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "sendToAttacherOnAttach")]
    SendToAttacherOnAttach(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "sendToAttacheeOnAttach")]
    SendToAttacheeOnAttach(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "sendToAttacherOnDetach")]
    SendToAttacherOnDetach(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "sendToAttacheeOnDetach")]
    SendToAttacheeOnDetach(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "attachmentSetup")]
    AttachmentSetup(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attacherHandle")]
    AttacherHandle(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attacheeHandle")]
    AttacheeHandle(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attacheeLayer")]
    AttacheeLayer(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "attacheeRB", skip_serializing)]
    AttacheeRb(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "oldMotionType", skip_serializing)]
    OldMotionType(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "oldFilterInfo", skip_serializing)]
    OldFilterInfo(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "attachment", skip_serializing)]
    Attachment(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentModifierVisitor<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("sendToAttacherOnAttach" => SendToAttacherOnAttach(SingleClass<HkbEventProperty<'de>>)),
    ("sendToAttacheeOnAttach" => SendToAttacheeOnAttach(SingleClass<HkbEventProperty<'de>>)),
    ("sendToAttacherOnDetach" => SendToAttacherOnDetach(SingleClass<HkbEventProperty<'de>>)),
    ("sendToAttacheeOnDetach" => SendToAttacheeOnDetach(SingleClass<HkbEventProperty<'de>>)),
    ("attachmentSetup" => AttachmentSetup(Primitive<Cow<'de, str>>)),
    ("attacherHandle" => AttacherHandle(Primitive<Cow<'de, str>>)),
    ("attacheeHandle" => AttacheeHandle(Primitive<Cow<'de, str>>)),
    ("attacheeLayer" => AttacheeLayer(Primitive<i32>)),
    ("attacheeRB" => AttacheeRb(Primitive<Cow<'de, str>>)),
    ("oldMotionType" => OldMotionType(Primitive<()>)),
    ("oldFilterInfo" => OldFilterInfo(Primitive<i32>)),
    ("attachment" => Attachment(Primitive<Cow<'de, str>>)),
}
