use sql_tool_core::SelectAttributeMacro;
use sql_tool_macros::GenSelect;

#[derive(GenSelect, Debug)]
struct MyStruct {
    field1: i32,
    #[select(ignore)]
    field2: i32,
    #[select(rename = "NULL::varchar as city_name")]
    field3: i32,
    #[select(rename = "CASE WHEN f.follower_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_followed")]
    field4: bool,
}

fn main() {
    println!("{:?}", MyStruct::generate_select_clause());
}
