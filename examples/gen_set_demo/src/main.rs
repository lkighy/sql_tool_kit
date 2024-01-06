use sql_tool_core::SetAttributeMacro;
use sql_tool_macros::GenSet;


/// 再在 GenSet 宏中的用法
/// ignore > ignore_none > rename = value > index
///
/// 目前接受与 sqlx 支持的数据库
/// GenSet 派生宏，需要导入 SetAttributeMacro 特性来使用
/// 用于生成 update 语句中的 set 部分
/// 例如 update table_name set field1 = $1, field2 = $2 ... where ...
/// 使用方法 update_data.generate_set_clause()
/// 返回值： ["field1 = $1", "field2 = $2", ...]
///
/// #[config(...)] 用于设置一些配置
/// 参数如下
/// - `database`: 使用的数据库，根据数据库的不同生成的占位符也不同，目前至此 mysql postgres sqlite mariadb, mssql
/// - `index`: 设置索引的开始数字
/// - `ignore_none`: 是否忽略为 None 的值，默认为 `true`
/// - `ignore_no_macro_set`: 是否忽略没有标注宏的值，默认为 `true` 该值的作用是为了配合 `GenWhere` 宏使用
///
/// #[set(...)] 宏，该宏为字段宏，如果 `ignore_no_macro_set` 为 `true` 则默认忽略没有设置该值的字段
/// 参数如下
///
/// `ignore`: 忽略该字段
/// `rename`: 接受字符串类型 给该字段重新命名
/// `ignore_none`: 接受 bool 类型 当该值为 None 时，是否忽略
/// `value`: 接受字符串类型 替换占位符为自己设置的值
/// `index`: 接受 i32 类型，替换默认的占位符序号，如果当前数据库支持占位符序号

#[derive(GenSet, Debug)]
#[config(database = "postgres", index = 4)]
pub struct SetStruct {
    #[set(rename = "id")]
    pub field1: i32,
    pub field2: Option<i32>,
    #[set(ignore)]
    pub field3: String,
    #[set(ignore_none = false)]
    pub field4: Option<String>,
    #[set()]
    pub field5: i32,
    pub field6: i32,
}

fn main() {
    let data = SetStruct {
        field1: 12,
        field2: Some(12),
        field3: "".to_string(),
        field4: None,
        field5: 0,
        field6: 0,
    };
    println!("{:?}", data.generate_set_clause());
}
