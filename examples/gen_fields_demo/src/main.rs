use sql_tool_core::FieldsAttributeMacro;
use sql_tool_macros::GenFields;

#[derive(GenFields, Debug)]
struct MyStruct {
    pub filed1: i32,
    #[field(ignore)] // 这段将被忽略
    pub field2: i32,
    #[field(rename = "rename_field")]
    pub field3: i32,
}

fn main() {
    println!("{:?}", MyStruct::generate_fields_clause());
}
