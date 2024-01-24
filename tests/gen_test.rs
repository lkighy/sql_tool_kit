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

#[test]
fn fields_and_select_test() {
    let select_data = vec![
        "id".to_string(),
        "user_name".to_string(),
        "email".to_string(),
        "description".to_string(),
    ];
    assert_eq!(select_data, SelectStruct::generate_select_clause());
    assert_eq!(select_data, FieldsStruct::generate_fields_clause());
}

#[cfg(test)]
#[derive(GenValues)]
#[config(database = "postgres")]
pub struct PgValuesStruct {
    pub id: i32,
    pub phone: String,
    #[value(ignore)]
    pub name: String,
    #[value(index = 2)]
    pub email: String,
    #[value(value = "'这是描述'")]
    pub description: Option<String>,
}

#[cfg(test)]
#[derive(GenValues)]
#[config(database = "postgres", index = 3)]
pub struct PgValuesStructIndex {
    pub id: i32,
    pub phone: String,
    #[value(ignore)]
    pub name: String,
    #[value(index = 2)]
    pub email: String,
    #[value(value = "'这是描述'")]
    pub description: Option<String>,
    #[value(value = "{index}::bit(4)")]
    pub user_type: i32,
}

#[cfg(test)]
#[derive(GenValues)]
#[config(database = "mysql")]
pub struct MysqlValuesStruct {
    pub id: i32,
    pub phone: String,
    #[value(ignore)]
    pub name: String,
    #[value(index = 2)]
    pub email: String,
    #[value(value = "'这是描述'")]
    pub description: Option<String>,
}

#[cfg(test)]
#[derive(GenValues)]
#[config(database = "mssql", index = 3)]
pub struct MssqlValuesStructIndex {
    pub id: i32,
    pub phone: String,
    #[value(ignore)]
    pub name: String,
    #[value(index = 2)]
    pub email: String,
    #[value(value = "'这是描述'")]
    pub description: Option<String>,
}

#[test]
fn values_test() {
    let data = vec![
        "$1".to_string(),
        "$2".to_string(),
        "$2".to_string(),
        "'这是描述'".to_string(),
    ];
    assert_eq!(data, PgValuesStruct::generate_values_clause());
    let data = vec![
        "$3".to_string(),
        "$4".to_string(),
        "$2".to_string(),
        "'这是描述'".to_string(),
        "$5::bit(4)".to_string(),
    ];
    assert_eq!(data, PgValuesStructIndex::generate_values_clause());
    let data = vec![
        "?".to_string(),
        "?".to_string(),
        "?".to_string(),
        "'这是描述'".to_string(),
    ];
    assert_eq!(data, MysqlValuesStruct::generate_values_clause());
    let data = vec![
        "@p3".to_string(),
        "@p4".to_string(),
        "@p2".to_string(),
        "'这是描述'".to_string(),
    ];
    assert_eq!(data, MssqlValuesStructIndex::generate_values_clause());
}

#[cfg(test)]
#[derive(GenWhere)]
#[config(database = "postgres")]
pub struct PgWhereStruct {
    #[r#where(condition_all = "title like {index}")]
    pub keyword: Option<String>,
    #[r#where(condition = ">=")]
    pub start_time: Option<String>,
    #[r#where(condition = "<=")]
    pub end_time: Option<String>,
    #[r#where()]
    pub ty: i32,
    #[r#where(ignore)]
    pub page_info: usize,
}

#[cfg(test)]
#[derive(GenWhere)]
#[config(database = "postgres", index = 3)]
pub struct PgWhereStructIndex {
    #[r#where(condition_all = "title like {index}")]
    pub keyword: Option<String>,
    #[r#where(condition = ">=")]
    pub start_time: Option<String>,
    #[r#where(condition = "<=")]
    pub end_time: Option<String>,
    #[r#where()]
    pub ty: i32,
    #[r#where(ignore)]
    pub page_info: usize,
}

#[cfg(test)]
#[derive(GenWhere)]
#[config(
    database = "postgres",
    ignore_none = false,
    ignore_no_macro_where = false
)]
pub struct PgWhereStructNoIgnore {
    #[r#where(condition_all = "title like {index}")]
    pub keyword: Option<String>,
    #[r#where(condition = ">=")]
    pub start_time: Option<String>,
    #[r#where(condition = "<=")]
    pub end_time: Option<String>,
    pub ty: i32,
    #[r#where(ignore)]
    pub page_info: usize,
}

#[cfg(test)]
#[derive(GenWhere)]
#[config(database = "postgres")]
pub struct PgWhereStructCondition {
    #[r#where(condition_all = "{name} like {index}")]
    pub keyword: Option<String>,
    #[r#where(condition = ">=", value = "'2024/12/12'")]
    pub start_time: Option<String>,
    #[r#where(condition = "<=", ignore_none = false)]
    pub end_time: Option<String>,
    pub ty: i32,
    #[r#where(ignore)]
    pub page_info: usize,
}

#[test]
fn where_test() {
    let value = PgWhereStruct {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty: 1,
        page_info: 0,
    };
    let data = vec![
        "title like $1".to_string(),
        "start_time >= $2".to_string(),
        "ty = $3".to_string(),
    ];
    assert_eq!(data, value.generate_where_clause());
    let value2 = PgWhereStructIndex {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty: 1,
        page_info: 0,
    };
    let data = vec![
        "title like $3".to_string(),
        "start_time >= $4".to_string(),
        "ty = $5".to_string(),
    ];
    assert_eq!(data, value2.generate_where_clause());
    assert_eq!(data, value.generate_where_clause_with_index(3));
    let value3 = PgWhereStructNoIgnore {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty: 1,
        page_info: 0,
    };
    let data = vec![
        "title like $1".to_string(),
        "start_time >= $2".to_string(),
        "end_time <= $3".to_string(),
        "ty = $4".to_string(),
    ];
    assert_eq!(data, value3.generate_where_clause());
    let value4 = PgWhereStructCondition {
        keyword: Some("这是标题".to_string()),
        start_time: Some("2024/12/12".to_string()),
        end_time: None,
        ty: 1,
        page_info: 0,
    };
    let data = vec![
        "keyword like $1".to_string(),
        "start_time >= '2024/12/12'".to_string(),
        "end_time <= $2".to_string(),
    ];
    assert_eq!(data, value4.generate_where_clause());
}

#[derive(GenSet)]
#[config(database = "postgres", index = 1, ignore_no_macro_set = false)]
pub struct PgSetAndWhereStruct {
    #[set(r#where)]
    pub id: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub description: Option<String>,
    #[set(value = "now()")]
    pub updated_at: Option<()>,
}

#[derive(GenSet)]
#[config(database = "postgres", index = 1)]
pub struct PgSetStruct {
    #[set()]
    pub id: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    #[set(rename = "url")]
    pub image_url: Option<String>,
    #[set(value = "''")]
    pub link_url: Option<String>,
    #[set(index = 4)]
    pub description: Option<String>,
    #[set(value = "now()")]
    pub updated_at: Option<()>,
}

#[test]
fn set_test() {
    let value = PgSetAndWhereStruct {
        id: 1,
        title: Some("这是标题".to_string()),
        subtitle: None,
        image_url: None,
        link_url: None,
        description: Some("这是描述".to_string()),
        updated_at: Some(()),
    };
    let set_data = vec![
        "title = $1".to_string(),
        "description = $2".to_string(),
        "updated_at = now()".to_string(),
    ];
    let where_data = vec!["id = $3".to_string()];
    let (set_value, where_value) = value.generate_set_and_where_clause();

    assert_eq!(set_data, set_value);
    assert_eq!(where_data, where_value);
    let value = PgSetStruct {
        id: 1,
        title: Some("这是标题".to_string()),
        subtitle: Some("".to_string()),
        image_url: Some("".to_string()),
        link_url: Some(String::new()),
        description: Some("这是描述".to_string()),
        updated_at: Some(()),
    };
    let set_data = vec![
        "id = $1".to_string(),
        "url = $2".to_string(),
        "link_url = ''".to_string(),
        "description = $4".to_string(),
        "updated_at = now()".to_string(),
    ];
    assert_eq!(set_data, value.generate_set_clause());
}
