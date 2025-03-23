use crate::class_map::ClassMap;
use crate::cpp_info::{Class, FlagValues, Member, Signature, TypeKind};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, hex_digit1, multispace0, not_line_ending},
    combinator::{map_res, recognize},
    error::context,
    sequence::tuple,
};
use std::{borrow::Cow, str::FromStr};

type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

/// parse 64bit offset C++ Class info
pub fn parse_x86_64_class_info(input: &str) -> IResult<&str, Class<'static>> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("//")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, name) = context("class name", take_while1(|c| c != ' '))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("Signatire:")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, signature) =
        map_res(recognize(tuple((tag("0x"), hex_digit1))), parse_int::parse)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("size:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, size) = context("size", map_res(digit1, parse_int::parse))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("flags:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _flags) = not_line_ending(input)?;
    let (input, _) = multispace0(input)?;

    let mut members = Vec::new();
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (_remain, member) = parse_member_info(line)?;
        members.push(member);
    }

    Ok((
        input,
        Class {
            name: name.to_string().into(),
            version: 0,
            signature: Signature::new(signature),
            size_x86: 0,
            size_x86_64: size,
            parent: None,
            parent_has_ref: false,
            vtable: false,
            has_ref: false,
            enums: Vec::new(),
            members,
        },
    ))
}

fn parse_member_info(input: &str) -> IResult<&str, Member<'static>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("//")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _name_prefix) = tag("m_")(input)?;
    let (input, name) = context("member name", take_while1(|c| c != ' '))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("m_class:")(input)?;
    let (input, _) = multispace0(input)?;

    let mut input = input;
    let mut class_ref = None;
    if !input.starts_with("Type.") {
        let (inner_input, ctype) = take_while1(|c: char| c != ' ')(input)?;
        class_ref = Some(ctype.into());
        let (inner_input, _) = multispace0(inner_input)?;
        input = inner_input;
    }

    let (input, _) = tag("Type.")(input)?;
    let (input, vtype) = context(
        "Havok vtype",
        map_res(take_while1(|c| c != ' '), TypeKind::from_str),
    )(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("Type.")(input)?;
    let (input, vsubtype) = context(
        "Havok vsubtype",
        map_res(take_while1(|c| c != ' '), TypeKind::from_str),
    )(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("arrSize:")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, arrsize) = context("array size", map_res(digit1, parse_int::parse))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("offset:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, offset) = context("offset", map_res(digit1, parse_int::parse))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = context("flags", tag("flags:"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, flags) = context(
        "flags",
        map_res(take_while1(|c| c != ' '), FlagValues::from_str),
    )(input)?;

    Ok((
        input,
        Member {
            name: name.to_string().into(),
            offset_x86: 0, // Dummy
            offset_x86_64: offset,
            class_ref: class_ref.map(|c: Cow<'_, str>| c.to_string().into()),
            enum_ref: None,
            has_ref: false,
            ctype: Default::default(),
            vtype,
            vsubtype,
            type_size_x86: 0,    // Dummy
            type_size_x86_64: 0, // Dummy
            arrsize,
            flags,
            default: 0,
        },
    ))
}

fn report(input: &str) -> Class<'static> {
    match parse_x86_64_class_info(input) {
        Ok((_, class_info)) => class_info,
        Err(e) => {
            let e = match e {
                nom::Err::Incomplete(e) => panic!("{:?}", e),
                nom::Err::Error(err) | nom::Err::Failure(err) => err,
            };
            panic!("Error: {}", nom::error::convert_error(input, e));
        }
    }
}

pub fn get_x64_classes_info() -> ClassMap<'static> {
    let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("hkx2lib");

    let mut map = ClassMap::new();

    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let class_info = report(&content);
        map.insert(class_info.name.clone(), class_info);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member_report(input: &str) -> Member {
        match parse_member_info(input) {
            Ok((_, member_info)) => member_info,
            Err(e) => {
                let e = match e {
                    nom::Err::Incomplete(e) => panic!("{:?}", e),
                    nom::Err::Error(err) | nom::Err::Failure(err) => err,
                };
                panic!("Error: {}", nom::error::convert_error(input, e));
            }
        }
    }

    #[test]
    fn should_parse_members() {
        let input = r#"
    // m_pSequence m_class:  Type.TYPE_CSTRING Type.TYPE_VOID arrSize: 0 offset: 72 flags: FLAGS_NONE enum:
    // m_eBlendModeFunction m_class:  Type.TYPE_ENUM Type.TYPE_INT8 arrSize: 0 offset: 80 flags: FLAGS_NONE enum: BlendModeFunction
    // m_fPercent m_class:  Type.TYPE_REAL Type.TYPE_VOID arrSize: 0 offset: 84 flags: FLAGS_NONE enum:
    // m_events m_class:  Type.TYPE_ARRAY Type.TYPE_VOID arrSize: 0 offset: 88 flags: SERIALIZE_IGNORED|FLAGS_NONE enum:
    // m_fTime m_class:  Type.TYPE_REAL Type.TYPE_VOID arrSize: 0 offset: 104 flags: SERIALIZE_IGNORED|FLAGS_NONE enum:
    // m_bDelayedActivate m_class:  Type.TYPE_BOOL Type.TYPE_VOID arrSize: 0 offset: 108 flags: SERIALIZE_IGNORED|FLAGS_NONE enum:
    // m_bLooping m_class:  Type.TYPE_BOOL Type.TYPE_VOID arrSize: 0 offset: 109 flags: SERIALIZE_IGNORED|FLAGS_NONE enum:
    "#;

        dbg!(member_report(input));
    }

    #[test]
    fn should_parse_classes() {
        let input = r#"
// BSBoneSwitchGenerator Signatire: 0xf33d3eea size: 112 flags: FLAGS_NONE

    // m_pDefaultGenerator m_class: hkbGenerator Type.TYPE_POINTER Type.TYPE_STRUCT arrSize: 0 offset: 80 flags: ALIGN_16|FLAGS_NONE enum:
    // m_ChildrenA m_class: BSBoneSwitchGeneratorBoneData Type.TYPE_ARRAY Type.TYPE_POINTER arrSize: 0 offset: 88 flags: FLAGS_NONE enum:
    "#;

        dbg!(report(input));
    }

    #[ignore = "need hkx2lib rpt files"]
    #[test]
    #[quick_tracing::init(test = "should_parse_all_class", level = "Debug")]
    fn should_parse_all_class_hkx2lib() {
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkx2lib");

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

            let content = std::fs::read_to_string(path).unwrap();
            let class_info = report(&content);
            let json = serde_json::to_string_pretty(&class_info).unwrap();

            let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("x86_64")
                .join("classes");
            std::fs::create_dir_all(&output_dir).unwrap();
            let mut output_file = output_dir.join(rpt_file_name);
            output_file.set_extension("json");

            std::fs::write(output_file, json).unwrap();
        }
    }
}
