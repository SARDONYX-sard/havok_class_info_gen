//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIStateManagerModifier`
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

/// `BSIStateManagerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x6cb24f2e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsiStateManagerModifier<'a> {
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
    /// -   name:`"iStateVar"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub i_state_var: i32,
    /// # C++ Class Fields Info
    /// -   name:`"stateData"`
    /// -   type: `hkArray<struct BSIStateManagerModifierBSiStateData>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub state_data: HkArrayClass<BsiStateManagerModifierBSiStateData<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"myStateListener"`
    /// -   type: `struct BSIStateManagerModifierBSIStateManagerStateListener`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub my_state_listener: SingleClass<BsiStateManagerModifierBsiStateManagerStateListener<'a>>,
}

impl Serialize for BsiStateManagerModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsiStateManagerModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsiStateManagerModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsiStateManagerModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsiStateManagerModifierVisitor<'a>>> for BsiStateManagerModifier<'a> {
    fn from(_values: Vec<BsiStateManagerModifierVisitor<'a>>) -> Self {
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
            let mut i_state_var = None;
            let mut state_data = None;
            let mut my_state_listener = None;


        for _value in _values {
            match _value {
                BsiStateManagerModifierVisitor::Enable(m) => enable = Some(m),
                BsiStateManagerModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsiStateManagerModifierVisitor::UserData(m) => user_data = Some(m),
                BsiStateManagerModifierVisitor::Name(m) => name = Some(m),
                BsiStateManagerModifierVisitor::Id(m) => id = Some(m),
                BsiStateManagerModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsiStateManagerModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsiStateManagerModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsiStateManagerModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsiStateManagerModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsiStateManagerModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsiStateManagerModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsiStateManagerModifierVisitor::IStateVar(m) => i_state_var = Some(m),
                BsiStateManagerModifierVisitor::StateData(m) => state_data = Some(m),
                BsiStateManagerModifierVisitor::MyStateListener(m) => my_state_listener = Some(m),

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
            i_state_var: i_state_var.unwrap_or_default().into_inner(),
            state_data: state_data.unwrap_or_default(),
            my_state_listener: my_state_listener.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsiStateManagerModifier<'a>> for Vec<BsiStateManagerModifierVisitor<'a>> {
    fn from(data: &BsiStateManagerModifier<'a>) -> Self {
        vec![
            BsiStateManagerModifierVisitor::Enable(data.enable.into()),
            BsiStateManagerModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsiStateManagerModifierVisitor::UserData(data.user_data.into()),
            BsiStateManagerModifierVisitor::Name(data.name.clone().into()),
            BsiStateManagerModifierVisitor::Id(data.id.into()),
            BsiStateManagerModifierVisitor::CloneState(data.clone_state.into()),
            BsiStateManagerModifierVisitor::PadNode(data.pad_node.clone()),
            BsiStateManagerModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsiStateManagerModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsiStateManagerModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsiStateManagerModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsiStateManagerModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsiStateManagerModifierVisitor::IStateVar(data.i_state_var.into()),
            BsiStateManagerModifierVisitor::StateData(data.state_data.clone()),
            BsiStateManagerModifierVisitor::MyStateListener(data.my_state_listener.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsiStateManagerModifier<'de> {
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
enum BsiStateManagerModifierVisitor<'a> {
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
    #[serde(rename = "iStateVar")]
    IStateVar(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "stateData")]
    StateData(HkArrayClass<BsiStateManagerModifierBSiStateData<'a>>),
    /// Visitor fields
    #[serde(rename = "myStateListener", skip_serializing)]
    MyStateListener(SingleClass<BsiStateManagerModifierBsiStateManagerStateListener<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierVisitor<'de>, "@name",
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
    ("iStateVar" => IStateVar(Primitive<i32>)),
    ("stateData" => StateData(HkArrayClass<BsiStateManagerModifierBSiStateData<'de>>)),
    ("myStateListener" => MyStateListener(SingleClass<BsiStateManagerModifierBsiStateManagerStateListener<'de>>)),
}
