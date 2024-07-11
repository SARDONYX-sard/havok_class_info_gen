use crate::{
    generators::aliases::{ClassMap, FieldMap, LifeTimeMap},
    hkxcmd_parser::{ClassInfo, MemberInfo},
};
use indexmap::IndexMap;

/// Return (Rust code, Rust enum tags that C++ Class fields)
///
/// # Information
/// - The lifetime annotation of a structure cannot be made without first calculating whether or not the field has a lifetime.
/// - Flatten the C++ parent's inherited moves to the fields of the current class.
pub fn generate_all_fields<'a>(
    class: &'a ClassInfo,
    classes_map: &'a ClassMap,
    life_time_map: Option<&LifeTimeMap>,
    generate_fields: fn(&'a [MemberInfo], Option<&LifeTimeMap>) -> (String, FieldMap<'a>),
) -> (String, FieldMap<'a>) {
    let mut all_fields_code = String::new();
    let mut fields = IndexMap::new();

    //? - All parent class fields of Current C++ class if exists.
    if let Some((current_parent_class_name, _)) = &class.parent {
        // 1. Get & The oldest parent class should be first.
        let parents = get_all_parents_info(current_parent_class_name, classes_map);

        // 2. Generate all parents
        //
        // # Why not flatten parent fields?
        // Since Rust has no field inheritance, all fields of the parent class are plugged into its structure.
        // (At first glance, you could just create a parent field, but in hkx XML, the field is determined by
        // the `hkparam` name attribute, so it becomes an internally tagged enum in quick_xml and must be represented by an enum.
        // Since enums cannot be flattened by expressions such as `parent', they must be enumerated in their entirety.
        for parent_class in parents {
            let (fields_code, field) = generate_fields(&parent_class.members, life_time_map);
            fields.extend(field);

            let parent_name = &parent_class.name;
            let parent_of_parent = &parent_class
                .parent
                .as_ref()
                .map(|(name, _)| name.as_str())
                .unwrap_or("None");

            let fields_code = match fields_code.is_empty() {
            true => format!("    // C++ Parent class(`{parent_name}` => parent: `{parent_of_parent}`) has no fields\n    //"),
            false => {
                let parent_info = format!(
                    "C++ Parent class(`{parent_name}` => parent: `{parent_of_parent}`) field Info"
                );
                fields_code.replace("C++ Class Fields Info", &parent_info)
            }
        };
            all_fields_code.push_str(&fields_code);
            all_fields_code.push('\n');
        }
    }

    //? - Current C++ class fields
    let (fields_code, field) = generate_fields(&class.members, life_time_map);
    fields.extend(field);
    all_fields_code.push_str(&fields_code);

    (all_fields_code, fields)
}

/// Enumerate C++ parent class information by recursively tracing from the parent class name of the current class.
///
/// # Returns
/// Vec sorted by deepest parent class.
pub fn get_all_parents_info<'a>(
    current_parent_name: &String,
    classes_map: &'a ClassMap,
) -> Vec<&'a ClassInfo> {
    // Cache variables
    let mut current_parent_class_name = current_parent_name;
    let mut parents = Vec::new();

    // Get all parents
    while let Some(parent_class) = classes_map.get(current_parent_class_name) {
        parents.push(parent_class);
        if let Some((parent_name, _parent_signature)) = &parent_class.parent {
            current_parent_class_name = parent_name;
        } else {
            break; // No more parent to process
        }
    }

    parents.reverse(); // This is because binary reads must be read from the most root parent class.
    parents
}
