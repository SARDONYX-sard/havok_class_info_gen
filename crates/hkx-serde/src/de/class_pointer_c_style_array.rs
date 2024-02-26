use super::finder::HavokFinder as _;
use crate::error::Result;
use roxmltree::Node;
use std::{collections::HashMap, rc::Rc};

pub fn class_pointer_c_style_array<'a, F, U>(
    element: &'a Node<'a, 'a>,
    name: &str,
    length: usize,
    objects_name_map: &mut HashMap<String, Rc<U>>,
    node_dict: &HashMap<String, Node<'a, 'a>>,
    deserialize: F,
) -> Result<Vec<Option<Rc<U>>>>
where
    F: Fn(&Node) -> Result<U>,
{
    // Get the property element
    let ele = match element.children().find_hkparam(name) {
        Some(ele) => ele,
        None => return Err("Not found hkparam".into()),
    };

    let mut arr = Vec::with_capacity(length);

    let ref_names: Vec<&str> = ele.text().unwrap_or("").split_whitespace().collect();
    if ref_names.len() != length {
        return Err(format!(
            "Content's elements mismatch property required. Property: {}, expected: {}, got: {}",
            name,
            length,
            arr.len()
        )
        .into());
    }

    for ref_name in ref_names {
        if ref_name == "null" {
            arr.push(None);
            continue;
        }

        if let Some(value) = objects_name_map.get(ref_name) {
            arr.push(Some(value.clone()));
            continue;
        }

        let node = node_dict.get(ref_name).ok_or_else(|| {
            format!(
                "Reference symbol '{}' not found. Make sure it is defined somewhere.",
                ref_name
            )
        })?;

        let res = Rc::new(deserialize(node)?);
        objects_name_map.insert(ref_name.to_string(), Rc::clone(&res));
        arr.push(Some(Rc::clone(&res)));
    }

    Ok(arr)
}
