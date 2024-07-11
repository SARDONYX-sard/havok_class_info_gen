use super::{
    class_params::generate_class_params,
    lifetime_manager::get_lifetime_from_fields,
    one_class::{
        enum_tagged::tagged_fields::generate_tagged_fields, generate_all_fields, generate_one_class,
    },
};
use crate::hkxcmd_parser::parse_class;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use std::{collections::HashMap, path::Path};

/// Generate all C++ Havok classes(To Rust enum types)
pub fn generate_classes(output_dir: impl AsRef<Path>, rpt_dir: impl AsRef<Path>) {
    let output_dir = output_dir.as_ref();
    let rpt_dir = rpt_dir.as_ref();

    let mut class_map = IndexMap::new();
    let mut mod_indexes = Vec::new();

    // 1. Create C++ class information and index codes from hkxcmd rpt files.
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        let path = path.as_path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("xml")) {
            continue;
        }

        // Exclude some problematic classes that aren't needed
        let file_name = path.file_stem().unwrap().to_str().unwrap();

        // Remove tailing version(e.g. _1)
        let file = file_name.rsplit('_').collect::<Vec<_>>();
        let rpt_file_name = *file.last().unwrap();

        if matches!(
            rpt_file_name,
            "hkaiTraversalAnalysis"
            | "hkaiTraversalAnalysisOutput"
            | "hkaiTraversalAnalysisOutputSection"
            | "hkaiTraversalAnnotationLibrary"
            | "hkaiTraversalAnnotationLibraryAnnotation"
            | "hkbnpPhysicsInterface"
            //
            // Classes that currently error out
            | "hkClassMember" // duplicate enum error DeprecatedFlagValues
            | "hkColor" // ExtendedColors is duplicate enum error
            | "hkpConstraintData"// duplicate enum error ConstraintType
            | "hkpWheelConstraintDataAtoms" // duplicate enum error Axis
            //
            | "hkpAgent1nSector" // Unsupported stack size [u8;460]
            | "hkpSerializedAgentNnEntry"  // Unsupported stack size [u8;160] nnEntryData
            //
            // dependencies above
            | "hkClass" //HkClassMember
            | "hkMonitorStreamColorTable" // hkMonitorStreamColorTableColorPair
            | "hkMonitorStreamColorTableColorPair" // ExtendedColors
            | "hkpWheelConstraintData"
        ) {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let (remain, class) = parse_class(&content).unwrap();
        tracing::debug!("remain = {remain:?}\nclass = {class:?}");

        // The binary deserializer implementation process requires only four classes, but only parses them for parent class information.
        class_map.insert(class.name.clone(), class.clone());

        #[cfg(debug_assertions)]
        // To extract only defaultmale.kkx classes for debugging purposes during the binary deserializer implementation process
        if !matches!(
            rpt_file_name,
            "hkRootLevelContainer"
                | "hkRootLevelContainerNamedVariant" // depended by `hkRootLevelContainer`
                | "hkbProjectStringData"
                | "hkbProjectData"
                | "hkbTransitionEffect" // For enum EventMode(depended by `hkbProjectData`)
        ) {
            continue;
        }

        let rust_file_name = rpt_file_name.to_case(Case::Snake);
        let import_code = format!("mod {rust_file_name};\npub use {rust_file_name}::*;\n");
        mod_indexes.push(import_code);
    }
    mod_indexes.push("pub mod class_params;\n".into());
    std::fs::write(output_dir.join("mod.rs"), mod_indexes.join("\n")).unwrap();

    // 2. Calculation of the first lifetime annotation (for standard library types already known to require annotation)
    // Example
    // field_name: Cow<'a, str>, => ClassName<'a>
    let mut life_time_name_map = HashMap::new();
    for (_sig, class) in class_map.clone().into_iter() {
        // fields = IndexMap<"C++ field name", ("rust enum tag name", "rust type name")>
        let (_rust_fields_code, fields) =
            generate_all_fields(&class, &class_map, None, generate_tagged_fields);

        let life_time = get_lifetime_from_fields(&fields);
        let rust_struct_name = class.name.to_case(Case::Pascal);
        let rust_struct_name_with_life_time = format!("{rust_struct_name}{life_time}");

        life_time_name_map.insert(rust_struct_name, rust_struct_name_with_life_time);
    }

    // 3. Compute the second lifetime annotation (annotate the propagated lifetime annotation, i.e., for the class of classes)
    // Example
    // field_name: ClassName<'a>, => ClassOfClass<'a>
    for (_sig, class) in &class_map {
        let (_rust_fields_code, fields) = generate_all_fields(
            class,
            &class_map,
            Some(&life_time_name_map),
            generate_tagged_fields,
        );

        let life_time = get_lifetime_from_fields(&fields);
        let rust_struct_name = class.name.to_case(Case::Pascal);
        let rust_struct_name_with_life_time = format!("{rust_struct_name}{life_time}");

        life_time_name_map.insert(rust_struct_name, rust_struct_name_with_life_time);
    }

    // 4. Generate havok class code.
    for (_sig, class) in &class_map {
        let rust_file = output_dir.join(format!("{}.rs", class.name.to_case(Case::Snake)));
        let rust_code = generate_one_class(&class.name, &class_map, &life_time_name_map);
        std::fs::write(rust_file, rust_code).unwrap();
    }

    // 5. Generate a set type of all havok class.
    let class_params = generate_class_params(&class_map, &life_time_name_map);
    std::fs::write(output_dir.join("class_params.rs"), class_params).unwrap();
}
