use crate::cpp_info::Class;
use indexmap::IndexMap;
use std::borrow::Cow;

pub type ClassMap<'a> = IndexMap<Cow<'a, str>, Class<'a>>;

/// Enumerate C++ parent class information by recursively tracing from the parent class name of the current class.
///
/// # Returns
/// Vec sorted by deepest parent class.
pub fn get_all_parents_info<'a>(
    current_parent_name: &Cow<'a, str>,
    classes_map: &'a ClassMap,
) -> Vec<&'a Class<'a>> {
    // Cache variables
    let mut current_parent_class_name = current_parent_name;
    let mut parents = Vec::new();

    // Get all parents
    while let Some(parent_class) = classes_map.get(current_parent_class_name) {
        parents.push(parent_class);
        if let Some(parent_name) = &parent_class.parent {
            current_parent_class_name = parent_name;
        } else {
            break; // No more parent to process
        }
    }

    parents.reverse(); // This is because binary reads must be read from the most root parent class.
    parents
}
