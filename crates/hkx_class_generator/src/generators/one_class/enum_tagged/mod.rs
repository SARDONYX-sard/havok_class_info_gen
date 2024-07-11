use self::{impl_deserialize::generate_impl_deserialize, tagged_fields::generate_tagged_fields};
use super::all_fields::generate_all_fields;
use crate::generators::{
    aliases::{ClassMap, LifeTimeMap},
    lifetime_manager::get_lifetime_from_fields,
};
use convert_case::{Case, Casing as _};

mod impl_deserialize;
pub mod tagged_fields;

pub const ENUM_UNIQUE_NAME: &str = "Tagged";

/// Generate Rust XML Serialize/Deserialize structs code from C++ class info>
pub fn generate_tagged_enum(
    cpp_class_name: &str,
    classes_map: &ClassMap,
    life_time_map: &LifeTimeMap,
) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(cpp_class_name).unwrap();
    let class_name = &class.name;

    // ! The lifetime annotation of a structure cannot be made without first calculating whether or not the fields has a lifetime.
    let (rust_fields_code, fields) = generate_all_fields(
        class,
        classes_map,
        Some(life_time_map),
        generate_tagged_fields,
    );

    // e.g. `HkColor<'a>`
    let rust_enum_name = class_name.to_case(Case::Pascal);
    let life_time = get_lifetime_from_fields(&fields);
    let rust_enum_name_with_life_time = format!("{rust_enum_name}{ENUM_UNIQUE_NAME}{life_time}");

    // Because the manual deserializer cannot be implemented if there is nothing in the tag of the enum representing all fields in the C++ Class.
    // Derive `serde::Deserialize`.
    let derive_de = if fields.is_empty() {
        ", Deserialize"
    } else {
        ""
    };

    //? - Generate Struct
    rust_code.push_str(&format!(
        r#"

/// # Why use this?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize{derive_de})]
#[serde(tag = "@name")]
enum {rust_enum_name_with_life_time} {{
{rust_fields_code}}}
"#
    ));

    // If there is no field, leave it to `derive`. Otherwise, implement manually.
    if !fields.is_empty() {
        let deserializer = generate_impl_deserialize(&rust_enum_name_with_life_time, &fields);
        rust_code.push_str(&deserializer);
    }

    rust_code
}
