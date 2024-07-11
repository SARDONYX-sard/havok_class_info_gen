use crate::hkxcmd_parser::ClassInfo;
use indexmap::IndexMap;
use std::{borrow::Cow, collections::HashMap};

/// (C++ class name, C++ class info)
///
/// e.g. `(hkColor, ClassInfo)`
pub type ClassMap = IndexMap<String, ClassInfo>;

/// (`rust enum name`, `Rust enum name with lifeTime`)
///
/// e.g. `(HkColor, HkColor<'a>)`
pub type LifeTimeMap = HashMap<String, String>;

/// # When Visitor
/// - `C++ field name`, (`Rust tag name of enum`, `Rust type name`)
/// - e.g. `referenceCount`, (`ReferenceCount`, `Primitive<i16>`)
///
/// # When struct
/// - `C++ field name`, (`Rust field name`, `Rust type name`)
/// - e.g. `referenceCount`, (`reference_count`, `i16`)
pub type FieldMap<'a> = IndexMap<&'a String, (String, Cow<'a, str>)>;
