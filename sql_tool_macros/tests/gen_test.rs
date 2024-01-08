//! 数据生成测试
use sql_tool_core::*;
use sql_tool_macros::*;

#[cfg(test)]
#[derive(GenSelect)]
pub struct SelectStruct {
    pub id: i32,
    #[select(ignore)]
    pub phone: String,
    #[select(rename = "user_name")]
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

#[cfg(test)]
#[derive(GenFields)]
pub struct FieldsStruct {
    pub id: i32,
    #[field(ignore)]
    pub phone: String,
    #[field(rename = "user_name")]
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

#[cfg(test)]
#[derive(GenValues)]
#[config(database = "postgres")]
pub struct PgValuesStruct {
    pub id: i32,
    pub phone: String,
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

#[test]
fn accuracy_test() {
    let select_data = vec![
        "id".to_string(),
        "user_name".to_string(),
        "email".to_string(),
        "description".to_string(),
    ];
    assert_eq!(select_data, SelectStruct::generate_select_clause());
    assert_eq!(select_data, FieldsStruct::generate_fields_clause());
}
