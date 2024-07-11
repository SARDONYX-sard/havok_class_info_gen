use crate::{
    generators::one_class::bitflags::generate_bitflags,
    hkxcmd_parser::{hk_types::Type, Enum, EnumItem},
};
#[allow(unused)]
use pretty_assertions::assert_eq;

#[test]
fn should_generate_bitflags() {
    // Define an example enum info
    let enum_info = Enum {
        name: "FlagValues".into(),
        hk_type: Type::Enum,
        sub_type: Type::Int32,
        enum_item: vec![
            ("FLAGS_NONE", 0),
            ("ALIGN8", 1 << 7),
            ("ALIGN16", 1 << 8),
            ("NOT_OWNED", 1 << 9),
            ("SERIALIZE_IGNORED", 1 << 10),
        ]
        .into_iter()
        .map(|(tag, value)| EnumItem {
            name: tag.into(),
            value,
        })
        .collect(),
        flags: "00000000".into(),
    };
    let generated_code = generate_bitflags(&enum_info);

    insta::assert_snapshot!(generated_code);
}
