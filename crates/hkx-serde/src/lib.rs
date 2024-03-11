mod bytes;
mod classes;
mod error;
mod flag_values;
pub mod generators;
pub mod havok_types;
mod helpers;
mod hk_types;
mod parse_rpt;

use crate::{generators::rust::generate_code, parse_rpt::parse_class};
use convert_case::{Case, Casing};

pub fn generate_classes() {
    let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("classes")
        .join("generated");
    std::fs::create_dir_all(&output_dir).unwrap();

    let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("hkxcmd_help")
        .join("rpt");

    let mut mod_indexes = Vec::new();
    let entries = jwalk::WalkDir::new(rpt_dir);

    #[cfg(test)]
    let mut test_index = 0;
    #[cfg(test)]
    let test_max = 1000;
    for entry in entries.into_iter() {
        #[cfg(test)]
        {
            match test_index >= test_max {
                true => break,
                false => test_index += 1,
            }
        }

        let path = entry.unwrap().path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("xml")) {
            continue;
        }

        // Exclude some problematic classes that aren't needed
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        if matches!(
            file_name,
            "hkaiTraversalAnalysis"
                | "hkaiTraversalAnnotationLibraryAnnotation"
                | "hkaiTraversalAnnotationLibrary"
                | "hkaiTraversalAnalysisOutputSection"
                | "hkaiTraversalAnalysisOutput"
                | "hkbnpPhysicsInterface"
        ) {
            continue;
        }

        // Remove tailing version(e.g. _1)
        let file_stem = file_name
            .rsplit('_')
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_case(Case::Snake);
        mod_indexes.push(format!("mod {};\nuse {}::*;\n", file_stem, file_stem));

        let content = std::fs::read_to_string(path).unwrap();
        let (remain, class) = parse_class(&content).unwrap();
        tracing::debug!("remain = {:?}", remain);
        tracing::debug!("class = {:?}", class);

        let rust_file = output_dir.join(format!("{file_stem}.rs"));
        let rust_code = generate_code(&class);
        std::fs::write(rust_file, rust_code).unwrap();
    }

    std::fs::write(output_dir.join("mod.rs"), mod_indexes.join("\n")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_generate_classes() {
        generate_classes()
    }
}
