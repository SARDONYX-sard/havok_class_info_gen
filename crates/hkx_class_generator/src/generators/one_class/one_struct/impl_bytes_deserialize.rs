use crate::{generators::aliases::FieldMap, hkxcmd_parser::MemberInfo};

pub fn generate_impl_bytes_deserialize(
    rust_class_name_with_life_time: &str,
    cpp_members: &[MemberInfo],
    rust_fields: &FieldMap,
) -> String {
    let rust_class_name_with_life_time = rust_class_name_with_life_time.replace("<'a>", "<'de>");

    let mut rust_code = String::new();
    let byte_read_code = String::new();
    let mut rust_fields_struct = String::new();

    for (member, rust_field) in cpp_members.iter().zip(rust_fields.iter()) {
        let MemberInfo {
            name: member_name,
            type_name,
            offset_x86: offset,
            flags,
            hk_type,
            ..
        } = member;
        let (_, (rust_field_name, rust_type)) = rust_field;

        // byte_read_code.push_str(&format!("           let {rust_field_name} = de.read_{}"));

        rust_fields_struct.push_str(&format!("           {rust_field_name},"));
    }

    rust_code.push_str(&format!(
        r#"
impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for {rust_class_name_with_life_time} {{
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut ByteDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {{
{byte_read_code}
        Ok(Self {{
{rust_fields_struct}
        }})
    }}
}}
"#
    ));

    rust_code
}
