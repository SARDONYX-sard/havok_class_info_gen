//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIsActiveModifier`
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

/// `BSIsActiveModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xb0fde45a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsIsActiveModifier<'a> {
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
    /// -   name:`"bIsActive0"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub b_is_active_0: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive0"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    pub b_invert_active_0: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive1"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    pub b_is_active_1: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive1"`
    /// -   type: `hkBool`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    pub b_invert_active_1: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive2"`
    /// -   type: `hkBool`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub b_is_active_2: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive2"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    pub b_invert_active_2: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive3"`
    /// -   type: `hkBool`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    pub b_is_active_3: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive3"`
    /// -   type: `hkBool`
    /// - offset: 51
    /// -  flags: `FLAGS_NONE`
    pub b_invert_active_3: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive4"`
    /// -   type: `hkBool`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub b_is_active_4: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive4"`
    /// -   type: `hkBool`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    pub b_invert_active_4: bool,
}

impl Serialize for BsIsActiveModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsIsActiveModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsIsActiveModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsIsActiveModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsIsActiveModifierVisitor<'a>>> for BsIsActiveModifier<'a> {
    fn from(_values: Vec<BsIsActiveModifierVisitor<'a>>) -> Self {
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
            let mut b_is_active_0 = None;
            let mut b_invert_active_0 = None;
            let mut b_is_active_1 = None;
            let mut b_invert_active_1 = None;
            let mut b_is_active_2 = None;
            let mut b_invert_active_2 = None;
            let mut b_is_active_3 = None;
            let mut b_invert_active_3 = None;
            let mut b_is_active_4 = None;
            let mut b_invert_active_4 = None;


        for _value in _values {
            match _value {
                BsIsActiveModifierVisitor::Enable(m) => enable = Some(m),
                BsIsActiveModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsIsActiveModifierVisitor::UserData(m) => user_data = Some(m),
                BsIsActiveModifierVisitor::Name(m) => name = Some(m),
                BsIsActiveModifierVisitor::Id(m) => id = Some(m),
                BsIsActiveModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsIsActiveModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsIsActiveModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsIsActiveModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsIsActiveModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsIsActiveModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsIsActiveModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsIsActiveModifierVisitor::BIsActive0(m) => b_is_active_0 = Some(m),
                BsIsActiveModifierVisitor::BInvertActive0(m) => b_invert_active_0 = Some(m),
                BsIsActiveModifierVisitor::BIsActive1(m) => b_is_active_1 = Some(m),
                BsIsActiveModifierVisitor::BInvertActive1(m) => b_invert_active_1 = Some(m),
                BsIsActiveModifierVisitor::BIsActive2(m) => b_is_active_2 = Some(m),
                BsIsActiveModifierVisitor::BInvertActive2(m) => b_invert_active_2 = Some(m),
                BsIsActiveModifierVisitor::BIsActive3(m) => b_is_active_3 = Some(m),
                BsIsActiveModifierVisitor::BInvertActive3(m) => b_invert_active_3 = Some(m),
                BsIsActiveModifierVisitor::BIsActive4(m) => b_is_active_4 = Some(m),
                BsIsActiveModifierVisitor::BInvertActive4(m) => b_invert_active_4 = Some(m),

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
            b_is_active_0: b_is_active_0.unwrap_or_default().into_inner(),
            b_invert_active_0: b_invert_active_0.unwrap_or_default().into_inner(),
            b_is_active_1: b_is_active_1.unwrap_or_default().into_inner(),
            b_invert_active_1: b_invert_active_1.unwrap_or_default().into_inner(),
            b_is_active_2: b_is_active_2.unwrap_or_default().into_inner(),
            b_invert_active_2: b_invert_active_2.unwrap_or_default().into_inner(),
            b_is_active_3: b_is_active_3.unwrap_or_default().into_inner(),
            b_invert_active_3: b_invert_active_3.unwrap_or_default().into_inner(),
            b_is_active_4: b_is_active_4.unwrap_or_default().into_inner(),
            b_invert_active_4: b_invert_active_4.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsIsActiveModifier<'a>> for Vec<BsIsActiveModifierVisitor<'a>> {
    fn from(data: &BsIsActiveModifier<'a>) -> Self {
        vec![
            BsIsActiveModifierVisitor::Enable(data.enable.into()),
            BsIsActiveModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsIsActiveModifierVisitor::UserData(data.user_data.into()),
            BsIsActiveModifierVisitor::Name(data.name.clone().into()),
            BsIsActiveModifierVisitor::Id(data.id.into()),
            BsIsActiveModifierVisitor::CloneState(data.clone_state.into()),
            BsIsActiveModifierVisitor::PadNode(data.pad_node.clone()),
            BsIsActiveModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsIsActiveModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsIsActiveModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsIsActiveModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsIsActiveModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsIsActiveModifierVisitor::BIsActive0(data.b_is_active_0.into()),
            BsIsActiveModifierVisitor::BInvertActive0(data.b_invert_active_0.into()),
            BsIsActiveModifierVisitor::BIsActive1(data.b_is_active_1.into()),
            BsIsActiveModifierVisitor::BInvertActive1(data.b_invert_active_1.into()),
            BsIsActiveModifierVisitor::BIsActive2(data.b_is_active_2.into()),
            BsIsActiveModifierVisitor::BInvertActive2(data.b_invert_active_2.into()),
            BsIsActiveModifierVisitor::BIsActive3(data.b_is_active_3.into()),
            BsIsActiveModifierVisitor::BInvertActive3(data.b_invert_active_3.into()),
            BsIsActiveModifierVisitor::BIsActive4(data.b_is_active_4.into()),
            BsIsActiveModifierVisitor::BInvertActive4(data.b_invert_active_4.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsIsActiveModifier<'de> {
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
enum BsIsActiveModifierVisitor<'a> {
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
    #[serde(rename = "bIsActive0")]
    BIsActive0(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bInvertActive0")]
    BInvertActive0(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bIsActive1")]
    BIsActive1(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bInvertActive1")]
    BInvertActive1(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bIsActive2")]
    BIsActive2(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bInvertActive2")]
    BInvertActive2(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bIsActive3")]
    BIsActive3(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bInvertActive3")]
    BInvertActive3(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bIsActive4")]
    BIsActive4(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bInvertActive4")]
    BInvertActive4(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsIsActiveModifierVisitor<'de>, "@name",
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
    ("bIsActive0" => BIsActive0(Primitive<bool>)),
    ("bInvertActive0" => BInvertActive0(Primitive<bool>)),
    ("bIsActive1" => BIsActive1(Primitive<bool>)),
    ("bInvertActive1" => BInvertActive1(Primitive<bool>)),
    ("bIsActive2" => BIsActive2(Primitive<bool>)),
    ("bInvertActive2" => BInvertActive2(Primitive<bool>)),
    ("bIsActive3" => BIsActive3(Primitive<bool>)),
    ("bInvertActive3" => BInvertActive3(Primitive<bool>)),
    ("bIsActive4" => BIsActive4(Primitive<bool>)),
    ("bInvertActive4" => BInvertActive4(Primitive<bool>)),
}
