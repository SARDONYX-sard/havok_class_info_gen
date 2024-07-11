use super::aliases::{FieldMap, LifeTimeMap};

/// Return `"<'a>"` if there is at least one lifetime annotation in the information that has been changed from a C++ field type to a Rust type.
///
/// This exists to get the information of the lifetime annotation, because if the field has a lifetime, the lifetime annotation must be declared in advance in struct.
pub fn get_lifetime_from_fields(index_map: &FieldMap) -> &'static str {
    for (_field_name, (_, rust_type)) in index_map {
        if rust_type.find("'a").is_some() {
            return "<'a>";
        }
    }
    ""
}

/// Get the given string and generate a modified string with `'a` added
///
/// e.g. key `EventProperty` => `EventProperty<'a>` or `EventProperty` value from [`HashMap`]
pub fn get_type_with_lifetime<'a>(
    rust_type_key: &'a str,
    life_time_map: Option<&'a LifeTimeMap>,
) -> Option<String> {
    let type_with_life_time = life_time_map.map(|map: &LifeTimeMap| map.get(rust_type_key))??;
    Some(type_with_life_time.to_owned())
}

/// Assign lifetime generics to the passed array type according to the type found in [`HashMap`].
pub fn add_lifetime_to_array(rust_type: &str, life_time_map: Option<&LifeTimeMap>) -> String {
    // Extract the portion between '<' and '>'
    let start_index = rust_type.find('<').unwrap() + 1;
    let end_index = rust_type.rfind('>').unwrap();
    let inner = &rust_type[start_index..end_index];
    let inner = get_type_with_lifetime(inner, life_time_map).unwrap_or(inner.to_owned());

    // Concatenate the prefix of the original string with the generated string
    format!("{}{inner}>", &rust_type[..start_index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;

    #[test]
    fn should_return_lifetime_if_present() {
        let mut index_map = FieldMap::new();
        let field1 = "field1".to_string();
        let field2 = "field2".to_string();
        index_map.insert(&field1, ("String".to_string(), Cow::Borrowed("String<'a>")));
        index_map.insert(&field2, ("i32".to_string(), Cow::Borrowed("i32")));

        assert_eq!(get_lifetime_from_fields(&index_map), "<'a>");
    }

    #[test]
    fn should_not_return_lifetime_if_not_present() {
        let field1 = "field1".to_string();
        let field2 = "field2".to_string();

        let mut index_map = FieldMap::new();
        index_map.insert(&field1, ("String".to_string(), Cow::Borrowed("String")));
        index_map.insert(&field2, ("i32".to_string(), Cow::Borrowed("i32")));

        assert_eq!(get_lifetime_from_fields(&index_map), "");
    }

    #[test]
    fn should_add_lifetime_to_array_type() {
        let rust_type = "Vec<EventProperty>";

        let mut life_time_map = LifeTimeMap::new();
        life_time_map.insert("EventProperty".to_string(), "EventProperty<'a>".to_string());

        let expected_result = "Vec<EventProperty<'a>>";
        assert_eq!(
            add_lifetime_to_array(rust_type, Some(&life_time_map)),
            expected_result
        );
    }

    #[test]
    fn should_not_add_lifetime_to_array_type_if_not_found_in_map() {
        let rust_type = "Vec<HashMap<String, EventProperty>>";
        let life_time_map = LifeTimeMap::new();
        let expected_result = "Vec<HashMap<String, EventProperty>>";
        assert_eq!(
            add_lifetime_to_array(rust_type, Some(&life_time_map)),
            expected_result
        );
    }

    #[test]
    fn should_get_type_with_lifetime_from_map() {
        let mut life_time_map = LifeTimeMap::new();
        life_time_map.insert("EventProperty".to_string(), "EventProperty<'a>".to_string());
        assert_eq!(
            get_type_with_lifetime("EventProperty", Some(&life_time_map)),
            Some("EventProperty<'a>".to_string())
        );
    }

    #[test]
    fn should_return_none_if_type_not_found_in_map() {
        let life_time_map = LifeTimeMap::new();
        assert_eq!(
            get_type_with_lifetime("EventProperty", Some(&life_time_map)),
            None
        );
    }
}
