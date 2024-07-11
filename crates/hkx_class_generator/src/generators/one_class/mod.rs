pub mod all_fields;
pub mod bitflags;
pub mod enum_tagged;
pub mod generate_enum;
mod one_struct;

use crate::hkxcmd_parser::Enum;

use self::{
    bitflags::generate_bitflags, enum_tagged::generate_tagged_enum, generate_enum::generate_enums,
    one_struct::generate_struct,
};
use super::aliases::{ClassMap, LifeTimeMap};
pub use all_fields::generate_all_fields;

/// Generate Rust XML Serialize/Deserialize structs code from C++ class info>
pub fn generate_one_class(
    cpp_class_name: &str,
    classes_map: &ClassMap,
    life_time_map: &LifeTimeMap,
) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(cpp_class_name).unwrap();
    let class_name = &class.name;

    // File docs & imports
    rust_code.push_str(&format!(r#"//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `{class_name}`
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
use crate::error::{{HkxError, Result}};
use crate::havok_types::*;
"#));

    rust_code.push_str(&generate_struct(cpp_class_name, classes_map, life_time_map));
    let visitor_code = generate_tagged_enum(cpp_class_name, classes_map, life_time_map);
    rust_code.push_str(&visitor_code);

    // Generate flags and enum
    if !class.enums.is_empty() {
        for enum_info in &class.enums {
            let Enum {
                name: enum_name, ..
            } = enum_info;

            if matches!(
                enum_name.as_str(),
                "FlagBits" | "FlagValues" | "Flags" | "HintFlags" | "RoleFlags" | "TransitionFlags"
            ) {
                rust_code.push_str(&generate_bitflags(enum_info));
            } else {
                rust_code.push_str(&generate_enums(enum_info));
            };
        }
    }

    rust_code
}
