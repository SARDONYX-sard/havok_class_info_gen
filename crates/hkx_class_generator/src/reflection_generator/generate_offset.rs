use crate::generators::{aliases::ClassMap, one_class::all_fields::get_all_parents_info};
use crate::hkxcmd_parser::MemberInfo;
use crate::{
    hkx2lib_parser::parse_txt::get_x64_classes_info,
    hkxcmd_parser::{hk_types::Type, parse_class, ClassInfo, FlagValues},
};
use indexmap::IndexMap;
use std::{collections::HashMap, io, path::Path};
use topo_sort::TopoSort;

///  Get the aligned value.
///
/// <https://github.com/rust-lang/rust/blob/1.30.0/src/libcore/alloc.rs#L199-L219>
const fn align(offset: u32, align: u32) -> u32 {
    offset.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}

/// Outputs json files of classes that take into account x86 and x86_64 offsets and size calculations.
pub fn generate_classes_json(output_dir: impl AsRef<Path>, rpt_dir: impl AsRef<Path>) {
    let output_dir = output_dir.as_ref();
    let rpt_dir = rpt_dir.as_ref();

    let mut class_map = IndexMap::new();
    let mut enum_map = HashMap::new();
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let (_remain, class) = parse_class(&content).unwrap();
        push_one_enum_map(&class.members, &mut enum_map);

        // The binary deserializer implementation process requires only four classes, but only parses them for parent class information.
        class_map.insert(class.name.clone(), class.clone());
    }

    generate_offset_info(output_dir, &class_map, &enum_map);
}

/// key: enum_ref, value: (enum or flag type, storage type)
type EnumMap = HashMap<String, (Type, Type)>;
fn push_one_enum_map(members: &[MemberInfo], enum_map: &mut EnumMap) {
    for member in members {
        if matches!(member.hk_type, Type::Flags | Type::Enum) {
            if let Some(ref enum_ref) = member.enum_ref {
                // Valid enum storage type check
                if !matches!(
                    member.sub_type,
                    Type::Int8
                        | Type::Uint8
                        | Type::Int16
                        | Type::Uint16
                        | Type::Int32
                        | Type::Uint32
                        | Type::Int64
                        | Type::Uint64
                ) {
                    panic!(
                        "This enum is invalid storage size type: {enum_ref}({})",
                        member.sub_type
                    );
                };

                // Skip if already registered
                if let Some((hk_type, sub_type)) = enum_map.get(enum_ref) {
                    if member.hk_type != *hk_type && member.sub_type != *sub_type {
                        panic!("There are members of the same enum definition with different sizes. expected: {hk_type}(sub: {sub_type}), actual: {}({})", member.hk_type, member.sub_type)
                    } else {
                        println!("Enum key already has registered: {enum_ref}");
                        continue;
                    };
                };

                enum_map.insert(
                    enum_ref.to_string(),
                    (member.hk_type.clone(), member.sub_type.clone()),
                );
            }
        }
    }
}

fn modify_enum(class_info: &mut ClassInfo, enum_map: &EnumMap) {
    for one_enum in &mut class_info.enums {
        let (hk_type, subtype) = &enum_map.get(&one_enum.name).unwrap_or_else(|| {
            println!("Not found key so fallback. : {}", one_enum.name);
            &(Type::Enum, Type::Void)
        });
        one_enum.hk_type = hk_type.clone();
        one_enum.sub_type = subtype.clone();
    }
}

pub fn generate_offset_info(
    output_dir: impl AsRef<Path>,
    class_map: &ClassMap,
    enum_map: &EnumMap,
) {
    // This block identifies dependencies and inserts data into a topological sort.
    let mut topo_sort = TopoSort::with_capacity(class_map.len());
    for (cpp_class_name, class_info) in class_map {
        let mut deps: Vec<&String> = class_info
            .parent
            .as_ref()
            .map(|parent| {
                get_all_parents_info(&parent.0, class_map)
                    .iter()
                    .map(|info| &info.name)
                    .collect()
            })
            .unwrap_or_default();

        for member in &class_info.members {
            if let Some(ref class_ref) = member.class_ref {
                if member.hk_type == Type::Struct && !deps.contains(&class_ref) {
                    deps.push(class_ref);
                };
            }
        }

        tracing::debug!("{:?},{:#?}", &cpp_class_name, &deps);
        topo_sort.insert(cpp_class_name, deps)
    }

    let ptr_size = 8;
    let x64_class_map = get_x64_classes_info();
    match topo_sort.into_vec_nodes() {
        topo_sort::SortResults::Full(sorted_classes) => {
            let mut size_map = HashMap::new();
            let mut max_member_size_map = HashMap::new(); // The largest size map in its class.(For tailing align struct)
            let mut first_struct_field_size_map = HashMap::new();

            // Get C++ class information from vec sorted by root order of dependencies and make json.
            for cpp_class_name in sorted_classes {
                tracing::debug!("cpp_class_name: {cpp_class_name:?}");
                let mut class_info = class_map[cpp_class_name].clone();
                let mut current_offset = 0;
                let mut max_member_size = 0; // Need this item for struct tailing alignment.

                // Fix enum type
                modify_enum(&mut class_info, enum_map);

                // C++ Parent class size
                if let Some(ref parent) = class_info.parent {
                    // When inheriting from a parent class, the starting point is the size of the parent class to .
                    let parent_size = size_map[&parent.0];
                    let parent_max_size = max_member_size_map[&parent.0];

                    current_offset += parent_size;
                    max_member_size = parent_max_size;
                } else if class_info.members.is_empty() && class_info.vtable {
                    // Even if it is an empty field, in the case of a vtable, there is a ptr to the vtable, so this is taken into account.
                    current_offset = ptr_size;
                    max_member_size = ptr_size;
                    first_struct_field_size_map.insert(cpp_class_name, ptr_size);
                } else if class_info.members.is_empty() && !class_info.vtable {
                    // The C++ empty class itself has a size of at least 1 to determine its address,
                    // because it cannot be addressed by 0.
                    // See: https://en.cppreference.com/w/cpp/language/ebo
                    //
                    // However, when an empty class inherits from another empty class,
                    // the empty class is treated as having a size of zero, which is called Empty Base Optimization (EBO).
                    current_offset = 1;
                    max_member_size = 1;
                    first_struct_field_size_map.insert(cpp_class_name, 1);
                };

                // Calculate C++ Members offset
                let mut prev_size = 0; // The previous field size is needed to align the next field.
                for (index, member) in &mut class_info.members.iter_mut().enumerate() {
                    let (mut current_member_size, mut current_max_size) =
                        if member.hk_type == Type::Struct {
                            let cpp_struct_name = member.class_ref.as_ref().unwrap();
                            (
                                size_map[cpp_struct_name],
                                max_member_size_map[cpp_struct_name],
                            )
                        } else {
                            (
                                member.type_size(&member.hk_type, ptr_size),
                                member.size_of_align(&member.hk_type, ptr_size),
                            )
                        };

                    if member.c_style_array_size > 0 {
                        current_member_size *= member.c_style_array_size as u32;
                    };

                    // Perform offset calculation for the current member.
                    //
                    // The next field must be a multiple of the current size.
                    if index == 0 {
                        if class_info.parent.is_some() {
                            let align_size = if member.hk_type == Type::Struct {
                                let field_class_info =
                                    &class_map[member.class_ref.as_ref().unwrap()];
                                get_first_field_size(&field_class_info.name, class_map, ptr_size)
                                    .unwrap_or(current_max_size)
                            } else {
                                current_max_size
                            };
                            first_struct_field_size_map.insert(cpp_class_name, align_size);
                            current_offset = align(current_offset, align_size);
                        };
                    } else {
                        current_offset = align(current_offset + prev_size, current_max_size);
                    };

                    // Alignment flags are enforced even in the first field if it has align flag.
                    if member.flags.contains(FlagValues::ALIGN_16) {
                        current_offset = align(current_offset, 16);
                        if current_max_size < 16 {
                            current_max_size = 16;
                        }
                    } else if member.flags.contains(FlagValues::ALIGN_8) {
                        current_offset = align(current_offset, 8);
                        if current_max_size < 8 {
                            current_max_size = 8;
                        }
                    };

                    // Correct information
                    member.offset_x86_64 = current_offset;
                    member.type_size_x86 = match member.hk_type == Type::Struct {
                        true => class_map[member.class_ref.as_ref().unwrap()].size_x86,
                        false => {
                            let type_size = member.type_size(&member.hk_type, 4);
                            if member.c_style_array_size > 0 {
                                type_size * (member.c_style_array_size as u32)
                            } else {
                                type_size
                            }
                        }
                    };
                    member.type_size_x86_64 = current_member_size;
                    member.has_string = if member.hk_type == Type::Struct
                        || (matches!(member.hk_type, Type::Array | Type::SimpleArray)
                            && member.sub_type == Type::Struct)
                    {
                        has_ref_member(member.class_ref.as_ref().unwrap(), class_map)
                    } else {
                        member.hk_type == Type::CString
                            || member.sub_type == Type::CString
                            || member.hk_type == Type::StringPtr
                            || member.sub_type == Type::StringPtr
                    };

                    prev_size = current_member_size;

                    // Calculate for tailing alignment of struct with max member size.
                    if let Some(ref parent) = class_info.parent {
                        let parent_max_size = max_member_size_map[&parent.0];
                        if parent_max_size > current_max_size {
                            current_max_size = parent_max_size;
                        }
                    };

                    if current_max_size > max_member_size {
                        max_member_size = current_max_size;
                    };
                }

                // Need tailing alignment for struct with max member size.
                let struct_size = align(current_offset + prev_size, max_member_size);
                class_info.size_x86_64 = struct_size;
                max_member_size_map.insert(class_info.name.clone(), max_member_size);

                // If the correct information for x64 already exists, overwrite it there.
                if let Some(x64_class) = x64_class_map.get(cpp_class_name) {
                    merge_class_info(&mut class_info, x64_class);
                };

                // Cache class information for next class offset calculation.
                size_map.insert(class_info.name.clone(), class_info.size_x86_64);

                class_info.has_string = has_ref_member(cpp_class_name, class_map);
                if let Some((name, _)) = &class_info.parent {
                    class_info.parent_has_string = has_ref_member(name, class_map);
                }

                write_json(&output_dir, &class_info).unwrap();
            }

            tracing::debug!("size_map = {size_map:#?}");
            tracing::debug!("max_size_map = {max_member_size_map:#?}");
            tracing::debug!("first_struct_field_size_map = {first_struct_field_size_map:#?}",);
        }
        topo_sort::SortResults::Partial(_) => todo!(),
    }
}

/// Determine the size of the first field from the given C++ Class name and C++ Classes data.
///
/// - If inherited from a parent class -> First field size of the oldest parent class
/// - No field but vtable ptr is present -> ptr size
/// - Other -> 0
///
/// This will reveal the starting point of the offset for target class if there is a parent class.
fn get_first_field_size(class_name: &str, class_map: &ClassMap, ptr_size: u32) -> Option<u32> {
    let class_info = class_map.get(class_name)?;
    if let Some((parent_name, _)) = &class_info.parent {
        let parent_info = get_all_parents_info(parent_name, class_map)[0];

        if let Some(first_member) = parent_info.members.first() {
            Some(first_member.size_of_align(&first_member.hk_type, ptr_size))
        } else if parent_info.members.is_empty() && class_info.vtable {
            Some(ptr_size)
        } else {
            None
        }
    } else if class_info.members.is_empty() && class_info.vtable {
        Some(ptr_size)
    } else {
        None
    }
}

/// Whether `CString` or `StringPtr` is contained in its own member or in a member of its parent?
///
/// This information is needed for the lifetime annotation (life of the reference) calculation.
fn has_ref_member(class_name: &str, class_map: &ClassMap) -> bool {
    let class_info = match class_map.get(class_name) {
        Some(class_info) => class_info,
        None => panic!("classMap get failed {class_name}"),
    };

    // Does the current class have a String?
    for member in &class_info.members {
        let type_is_string = matches!(member.hk_type, Type::CString | Type::StringPtr);
        let subtype_is_string = matches!(member.sub_type, Type::CString | Type::StringPtr);
        if type_is_string || subtype_is_string {
            return true;
        }

        if (member.hk_type == Type::Struct
            || (matches!(member.hk_type, Type::Array | Type::SimpleArray)
                && member.sub_type == Type::Struct))
            && has_ref_member(member.class_ref.as_ref().unwrap(), class_map)
        {
            return true;
        };
    }

    // Does the parent class have a String?
    if let Some((parent_name, _)) = &class_info.parent {
        let parents = get_all_parents_info(parent_name, class_map);

        for parent_info in &parents {
            for member in &parent_info.members {
                let type_is_string = matches!(member.hk_type, Type::CString | Type::StringPtr);
                let subtype_is_string = matches!(member.sub_type, Type::CString | Type::StringPtr);
                if type_is_string || subtype_is_string {
                    return true;
                };
                if (member.hk_type == Type::Struct
                    || (matches!(member.hk_type, Type::Array | Type::SimpleArray)
                        && member.sub_type == Type::Struct))
                    && has_ref_member(member.class_ref.as_ref().unwrap(), class_map)
                {
                    return true;
                };
            }
        }
        false
    } else {
        false
    }
}

/// Update x86 class_info with x64 information.
/// # Note
/// Update only the offset and size, since the current implementation is unusable except for the x64 offset and size.
fn merge_class_info(class_info: &mut ClassInfo, x64_class_info: &ClassInfo) {
    // Merge basic fields
    // class_info.version = x64_class_info.version;
    // class_info.signature = x64_class_info.signature;
    class_info.size_x86_64 = x64_class_info.size_x86_64;
    // class_info.vtable = x64_class_info.vtable;

    // Merge enums
    // class_info.enums.extend_from_slice(&x64_class_info.enums);

    // Merge parent class
    // if let Some(parent) = &x64_class_info.parent {
    //     class_info.parent = Some(parent.clone());
    // }

    // Merge members by name
    for x64_member in &x64_class_info.members {
        if let Some(existing_member) = class_info
            .members
            .iter_mut()
            .find(|m| m.name == x64_member.name)
        {
            // Update existing member
            existing_member.offset_x86_64 = x64_member.offset_x86_64;
            existing_member.class_ref.clone_from(&x64_member.class_ref);
            // existing_member.enum_ref.clone_from(&x64_member.enum_ref);
            // existing_member.type_name.clone_from(&x64_member.type_name);
            existing_member.hk_type = x64_member.hk_type.clone();
            existing_member.sub_type = x64_member.sub_type.clone();
            existing_member.c_style_array_size = x64_member.c_style_array_size;
            existing_member.flags = x64_member.flags;
            // existing_member.default_value = x64_member.default_value;
        } else {
            // Add new member
            // class_info.members.push(x64_member.clone());
        }
    }
}

fn write_json(output_dir: impl AsRef<Path>, class_info: &ClassInfo) -> io::Result<()> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir)?;
    let mut path = output_dir.join(class_info.name.clone());
    path.set_extension("json");
    let json = serde_json::ser::to_string_pretty(&class_info).unwrap();
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[quick_tracing::init(test = "should_x86_64_json", level = "Debug")]
    #[test]
    pub fn should_generate_x86_64_json() {
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("classes");
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        generate_classes_json(output_dir, rpt_dir)
    }
}
