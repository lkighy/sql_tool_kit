use sql_tool_core::{FieldsAttributeMacro, SelectAttributeMacro, ValuesAttributeMacro, SetAttributeMacro, WhereAttributeMacro};
use sql_tool_macros::{GenFields, GenValues, GenSet, GenWhere, GenSelect};

// GenFields 宏 和 GenSelect 宏的使用
#[derive(GenFields, GenSelect, Debug)]
pub struct SelectStruct {
    pub field1: i32,
    #[field(ignore)] // 忽略该值
    #[select(ignore)]
    pub field2: i32,
    #[field(rename = "is_followed")]
    #[select(rename = "CASE WHEN f.follower_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_followed")]
    pub field3: i32,
}

#[derive(GenFields, GenValues, Debug)]
#[config(database = "postgres")]
pub struct InsertStruct {
    pub field1: i32,
    #[value(ignore)]
    #[field(ignore)] // 忽略该值
    pub field2: i32,
    #[value(rename = "user_name")]
    pub field3: i32,
}

#[derive(GenSet, GenWhere, Debug)]
#[config(database = "postgres")]
pub struct UpdateStruct {
    #[set()]
    pub field1: i32,
    #[set(rename = "user_name")]
    pub field2: String,
    #[set(rename = "email")]
    pub field3: Option<String>,
    #[r#where(rename = "id")]
    pub field4: i32,
    #[r#where(condition = ">=")]
    pub field5: i32,
    #[r#where(condition_all = "{name} is not null")]
    pub field6: i32,
}

#[derive(GenWhere, Debug)]
#[config(database = "postgres")]
pub struct DeleteStruct {
    #[r#where(rename = "id")]
    pub field1: i32,
    #[r#where(condition_all = "{name} = ANY({index}::int[])")]
    pub field2: Option<Vec<i32>>,
    #[r#where(condition_all = "!=")]
    pub field3: Option<String>,
}

fn main() {
    // 生成sql语句
    println!("select {} where table_name returning {}", SelectStruct::generate_select_clause().join(", "), SelectStruct::generate_select_clause().join(", "));
    // 输出： select field1, CASE WHEN f.follower_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_followed where table_name returning field1, CASE WHEN f.follower_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_followed
    println!("insert into table_name ({}) values ({})", InsertStruct::generate_fields_clause().join(", "), InsertStruct::generate_values_clause().join(", "));
    // 输出：insert into table_name (field1, field3) values ($1, $2)
    let update_data = UpdateStruct {
        field1: 0,
        field2: "用户名称".to_string(),
        field3: None,
        field4: 13,
        field5: 100,
        field6: 0,
    };
    println!("update table_name {} where {}", update_data.generate_set_clause().join(", "), update_data.generate_where_clause().join(" AND "));
    // 输出：update table_name field1 = $1, user_name = $2 where id = $1 AND field5 >= $2 AND field6 is not null
    let delete_data = DeleteStruct {
        field1: 1,
        field2: Some(vec![1,2,3]),
        field3: None,
    };
    println!("delete table_name where {}", delete_data.generate_where_clause().join(" AND "));
    // 输出：delete table_name where id = $1 AND field2 = ANY($2::int[])
}
