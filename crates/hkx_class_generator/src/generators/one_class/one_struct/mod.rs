mod impl_bytes_deserialize;
mod impl_from;
mod impl_serde;
pub mod struct_fields;

use self::{
    impl_bytes_deserialize::generate_impl_bytes_deserialize, impl_from::generate_impl_from,
    impl_serde::generate_impl_serde, struct_fields::generate_struct_fields,
};
use super::all_fields::generate_all_fields;
use crate::{
    generators::{
        aliases::{ClassMap, LifeTimeMap},
        lifetime_manager::get_lifetime_from_fields,
    },
    hkxcmd_parser::ClassInfo,
};
use convert_case::{Case, Casing as _};

/// Generate Rust XML Serialize/Deserialize structs code from C++ class info>
pub fn generate_struct(
    cpp_class_name: &str,
    classes_map: &ClassMap,
    life_time_map: &LifeTimeMap,
) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(cpp_class_name).unwrap();
    let ClassInfo {
        signature,
        vtable,
        name: class_name,
        parent,
        size_x86: size,
        version,
        members,
        ..
    } = class;

    let parent_doc = parent
        .as_ref()
        .map(|(name, signature)| format!("\n/// -    parent: `{name}`/`{signature:#x}`"))
        .unwrap_or_default();

    // ! The lifetime annotation of a structure cannot be made without first calculating whether or not the fields has a lifetime.
    let (rust_fields_code, fields) = generate_all_fields(
        class,
        classes_map,
        Some(life_time_map),
        generate_struct_fields,
    );

    // Because the manual deserializer cannot be implemented if there is nothing in the tag of the enum representing all fields in the C++ Class.
    // Derive `serde::Deserialize`.
    let derive_serde = if fields.is_empty() {
        ", Serialize, Deserialize"
    } else {
        ""
    };

    // e.g. `HkColor<'a>`
    let rust_struct_name = class_name.to_case(Case::Pascal);
    let life_time = get_lifetime_from_fields(&fields);
    let rust_struct_name_with_life_time = format!("{rust_struct_name}{life_time}");

    //? - Generate Struct
    rust_code.push_str(&format!(
        r#"
/// `{class_name}`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: {size}
/// -    vtable: {vtable}{parent_doc}
/// - signature: `{signature:#x}`
/// -   version: {version}
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq{derive_serde})]
pub struct {rust_struct_name_with_life_time} {{
{rust_fields_code}}}
"#
    ));

    // If there is no field, leave it to `derive`. Otherwise, implement manually.
    if !fields.is_empty() {
        let impl_serde_code = generate_impl_serde(&rust_struct_name, life_time, fields.len());

        rust_code.push_str(&impl_serde_code);
        rust_code.push_str(&generate_impl_from(&rust_struct_name, &fields));
    }
    let impl_bytes_de_code =
        generate_impl_bytes_deserialize(&rust_struct_name_with_life_time, members, &fields);
    rust_code.push_str(&impl_bytes_de_code);

    rust_code
}
