use crate::generators::{
    aliases::FieldMap, lifetime_manager::get_lifetime_from_fields,
    one_class::enum_tagged::ENUM_UNIQUE_NAME, utils::is_copyable,
};
use convert_case::Casing;

/// Generate code to define the conversion for `Tagged enum for XML` <-> `Struct`.
pub fn generate_impl_from(rust_struct_name: &str, fields: &FieldMap) -> String {
    let mut return_rust_code = String::new();

    let fields_len = fields.len();
    let lifetime = get_lifetime_from_fields(fields);

    let mut fields_values_code = String::new();
    let mut enum_match_inner = String::new();
    let mut visitor_to_struct_fields = String::new();
    let mut struct_ref_to_enum_fields = String::new();

    for (_, (struct_field, field_type)) in fields {
        let enum_field = struct_field.to_case(convert_case::Case::Pascal);
        let enum_field = match enum_field.chars().next().map_or(false, |c| c.is_numeric()) {
            true => format!("_{enum_field}"),
            false => enum_field,
        };

        fields_values_code.push_str(&format!(
            r#"        let mut {struct_field} = None;
"#,
        ));

        enum_match_inner.push_str(&format!(
        r#"                {rust_struct_name}{ENUM_UNIQUE_NAME}::{enum_field}(m) => {struct_field} = Some(m),
"#
    ));

        visitor_to_struct_fields.push_str(&format!(
            r#"            {struct_field}: {struct_field}.unwrap_or_default().into_inner(),
"#
        ));

        let clone_method = if is_copyable(field_type) {
            ""
        } else {
            ".clone()"
        };
        struct_ref_to_enum_fields.push_str(&format!(
            r#"            {rust_struct_name}{ENUM_UNIQUE_NAME}::{enum_field}(data.{struct_field}{clone_method}.into()),
"#
        ));
    }

    // serde only supports up to `[T; 32]`, so use `Vec` if it is larger than that.
    let enum_array_type = if fields_len > 0 {
        format!("Vec<{rust_struct_name}{ENUM_UNIQUE_NAME}{lifetime}>")
    } else {
        format!("[{rust_struct_name}{ENUM_UNIQUE_NAME}{lifetime}; {fields_len}]")
    };
    let vec_macro = if fields_len > 0 { "vec!" } else { "" };
    // Tagged enum to Struct Code
    return_rust_code.push_str(&format!(
        r#"
impl{lifetime} From<{enum_array_type}> for {rust_struct_name}{lifetime} {{
    fn from(_values: {enum_array_type}) -> Self {{
{fields_values_code}

        for _value in _values {{
            match _value {{
{enum_match_inner}
            }}
        }}

        // This `unwrap_or_default` is never called because it depends on the default value of `{ENUM_UNIQUE_NAME}
        Self {{
{visitor_to_struct_fields}
        }}
    }}
}}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl{lifetime} From<&{rust_struct_name}{lifetime}> for {enum_array_type} {{
    fn from(data: &{rust_struct_name}{lifetime}) -> Self {{
        {vec_macro}[
{struct_ref_to_enum_fields}
        ]
    }}
}}
"#
    ));

    return_rust_code
}
