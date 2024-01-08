use sql_tool_kit::{ValuesAttributeMacro, GenValues};

#[derive(GenValues)]
#[config(database = "postgres")]
struct PostgresStruct {
    field1: i32,
    #[value(ignore)]
    field2: i32,
    #[value(index = 4)]
    field3: i32,
}

#[derive(GenValues)]
#[config(database = "mysql")]
struct MysqlStruct {
    field1: i32,
    #[value(ignore)]
    field2: i32,
    #[value(index = 4)] // mysql 并不需要占位符，所以不需要 `index`
    field3: i32,
}
// 设置开始的索引
#[derive(GenValues)]
#[config(database = "postgres", index = 5)]
struct PostgresSetIndexStruct {
    field1: i32,
    field2: i32,
    field3: i32,
}


fn main() {
    println!("MysqlStruct: {:?}", MysqlStruct::generate_values_clause()); // 输出：["?", "?"]
    println!("{:?}", PostgresStruct::generate_values_clause()); // 输出：["$1", "$4"]
    println!("{:?}", PostgresSetIndexStruct::generate_values_clause()); // 输出：["$5", "$6"]
}
