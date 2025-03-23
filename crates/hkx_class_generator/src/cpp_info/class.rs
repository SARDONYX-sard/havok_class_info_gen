//! C++ Havok class information.
use super::signature::Signature;
use super::{member::Member, Enum};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// C++ Havok class information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Class<'a> {
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    /// Class name(e.g. `hkRootContainer`)
    pub name: Cow<'a, str>,

    /// Havok engine revision version(Maybe)
    pub version: u32,

    /// Class signature
    pub signature: Signature,

    /// Class size for x86
    pub size_x86: u32,

    /// Class size for x86_64
    pub size_x86_64: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(bound(deserialize = "Option<Cow<'a, str>>: Deserialize<'de>"))]
    /// Super class name
    pub parent: Option<Cow<'a, str>>,

    /// When type or subtype is `struct`, does it contain a `CString` or `StringPtr`, or "struct containing them" type?
    ///
    /// This information is needed for the lifetime annotation (life of the reference) calculation.
    pub parent_has_ref: bool,

    /// Is virtual table C++ class?
    pub vtable: bool,

    /// The reference type(e.g. `CString`, `StringPtr`) is contained in its own member or in a member of its parent?
    /// (To calculate lifetime annotation)
    pub has_ref: bool,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    /// Vector of enum names & enum fields
    pub enums: Vec<Enum<'a>>,

    /// C++ Class member Information
    pub members: Vec<Member<'a>>,
}
