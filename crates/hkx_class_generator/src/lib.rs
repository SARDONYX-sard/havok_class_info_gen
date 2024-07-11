mod cpp_type_parser;
pub mod generators;
#[cfg(test)]
mod generators_tests;
pub mod hkx2lib_parser;
pub mod hkxcmd_parser;
pub mod reflection_generator;

pub use generators::generate_classes;

#[cfg(test)]
mod tests {
    use crate::generators::generate_classes;

    #[ignore]
    #[test]
    #[quick_tracing::init(test = "should_generate_classes", level = "Error")]
    pub fn should_generate_classes() {
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("hkx_serde")
            .join("src")
            .join("classes")
            .join("generated");
        std::fs::create_dir_all(&output_dir).unwrap();

        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");
        generate_classes(output_dir, rpt_dir)
    }
}
