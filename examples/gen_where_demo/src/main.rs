use sql_tool_core::WhereAttributeMacro;
use sql_tool_macros::GenWhere;


/// 在 #[where(...)] 字段宏上的属性先后级
/// `ignore` > `ignore_none` > `condition_all` > `rename` = `condition` = `value` > `index`
#[derive(GenWhere)]
#[config(database = "postgres", ignore_none = true)] // 这里的 `ignore_none` 表示是否忽略为 none 的值，默认为 true
pub struct PostgresStruct {
    #[r#where(ignore)]
    pub field1: Option<i32>,
    pub field2: i32,
    #[r#where(condition = ">=")]
    pub field3: Option<i32>,
    #[r#where(condition = "<", index = 1)]
    pub field4: i32,
    #[r#where(condition_all = "field5 not null")]
    pub field5: i32,
    #[r#where(rename = "rename_filed", condition = ">=")]
    pub field6: i32,
    #[r#where(condition_all = "field7 = ANY({index}::int[])")] // 这里的{index} 会被替换，如果为 postgres 则 会被替换为 `$x` mysql 则替换为 `{}`
    pub field7: i32,
    #[r#where(rename = "rename_filed2", condition_all = "{name} = ANY({index}::int[])")] // 这里的{name} 会被替换为字段名称或 rename 的名称
    pub field8: i32,
    #[r#where(ignore_none = false)] // 这样就不会为 None 的值了
    // pub field9: Option<i32>,
    // condition_all 的默认值为 "{name} {condition} {index}"
    // {name} 可以为 `rename` 如果 `rename` 不存在，则使用字段 名称
    // {condition} 可以是 `condition` 如果在 `condition_all` 中存在 "{condition}" 但并未设置 `condition` 则会报错
    // {index} 可以是 `index` 如果该字段上不存在该属性，则使用全局index
    #[r#where(condition_all = "{name} {condition} {index}", condition=">")]
    pub field10: Option<i32>,
}

///
#[derive(GenWhere, Debug)]
#[config(database = "postgres")]
pub struct WhereStruct {
    #[r#where(rename = "id")]
    pub field1: i32,
    pub field2: Option<i32>,
    #[r#where(ignore)]
    pub field3: String,
    #[r#where(ignore_none = false)]
    pub field4: Option<String>,
    #[r#where(condition_all = "{name} {condition} {index}", condition=">")]
    pub field5: i32,
    #[r#where(condition_all = "{name} {condition} {index}", value = "25")]
    pub field6: i32,
}

fn main() {
    let data = WhereStruct {
        field1: 12,
        field2: Some(12),
        field3: "".to_string(),
        field4: None,
        field5: 0,
        field6: 0,
    };
    // 输出 ["id = $1", "field2 = $2", "field4 = $3", "field5 > $4", "field6 = 25"]
    println!("{:?}", data.generate_where_clause());
    let data = PostgresStruct {
        field1: None,
        field2: 0,
        field3: None,
        field4: 0,
        field5: 0,
        field6: 0,
        field7: 0,
        field8: 0,
        field10: None,
    };
    // 输出 ["field4 < $1", "field5 not null", "rename_filed >= $3", "field7 = ANY($4::int[])", "rename_filed2 = ANY($5::int[])", "field10 = $6"]
    println!("{:?}", data.generate_where_clause());
}
