use proc_macro::TokenStream;

mod fields;
mod select;
mod values;

/// 过程宏入口点，用于处理 `#[field(...)]` 属性宏。
///
/// 此宏处理结构体定义上的 `#[field(...)]` 属性，用于生成符合 `FieldsAttributeMacro`
/// trait 的实现。这允许在结构体字段级别上定义如何处理 SQL 字段，例如忽略某些字段，
/// 或将字段重命名。
///
/// # 参数
/// * `item`: TokenStream，表示传入的 Rust 代码项，通常是结构体定义。
///
/// # 返回值
/// 返回 TokenStream，包含了生成的 trait 实现代码。
///
/// # 使用示例
/// ```ignore
/// use sql_tool_core::FieldsAttributeMacro;
///
/// #[derive(GenFields, Debug)]
/// struct MyStruct {
///     field1: i32,
///     #[field(ignore)]
///     field2: i32,
///     #[field(rename = "rename_field")]
///     field3: i32,
/// }
///
/// MyStruct::generate_fields_clause(); // 输出：["field1", "rename_field"]
/// ```
#[proc_macro_derive(GenFields, attributes(field))]
pub fn fields_attribute_macro(item: TokenStream) -> TokenStream {
    fields::gen_fields_attribute_impl(item)
}

/// 过程宏入口点，用于处理 `#[select(...)]` 属性宏。
///
/// 此宏处理结构体定义上的 `#[select(...)]` 属性，用于生成符合 `SelectAttributeMacro`
/// trait 的实现。这允许在结构体字段级别上定义如何处理 SQL 字段，例如忽略某些字段，
/// 或将字段重命名。
///
/// # 参数
/// * `item`: TokenStream，表示传入的 Rust 代码项，通常是结构体定义。
///
/// # 返回值
/// 返回 TokenStream，包含了生成的 trait 实现代码。
///
/// # 使用示例
/// ```ignore
/// use sql_tool_core::SelectAttributeMacro;
///
/// #[derive(GenSelect, Debug)]
/// struct MyStruct {
///     field1: i32,
///     #[select(ignore)]
///     field2: i32,
///     #[select(rename = "NULL::varchar as city_name")]
///     field3: i32,
///     #[select(rename = "CASE WHEN f.follower_id IS NOT NULL THEN TRUE ELSE FALSE END AS is_followed")]
///     field4: bool,
/// }
///
/// MyStruct::generate_fields_clause(); // 输出：["field1", "NULL::varchar as city_name", "CASE WHE..."]
/// ```
#[proc_macro_derive(GenSelect, attributes(select))]
pub fn select_attribute_macro(item: TokenStream) -> TokenStream {
    select::gen_select_attribute_impl(item)
}

/// 生成针对特定结构体的 `ValuesAttributeMacro` 实现。
///
/// 此函数解析标记在结构体字段上的 `#[derive(GenValues)]` 属性宏，
/// 并生成一个实现 `ValuesAttributeMacro` trait 的代码块。
/// 它处理每个字段的 `ignore` 和 `index` 指令，以生成相应的字段列表。
/// 结构上的 `#[config(database = "postgres", index = 1)]` 用于设置使用的数据库类型，对于部分有索引的数据库如 `postgres` 可以使用index设置初始值
///
/// 目前支持的数据库有："postgres", "mariadb", "mysql", "sqlite", "mssql"
///
/// # 参数
/// * `item`: TokenStream，表示要处理的 Rust 代码项（一般是结构体定义）。
///
/// # 返回值
/// 返回一个 `TokenStream`，它包含了生成的 `FieldsAttributeMacro` trait 实现。
///
/// # 示例
/// ```ignore
/// use sql_tool_core::ValuesAttributeMacro;
///
/// #[derive(GenValues)]
/// #[config(database = "postgres")]
/// struct PostgresStruct {
///     field1: i32,
///     #[value(ignore)]
///     field2: i32,
///     #[value(index = 4)]
///     field3: i32,
/// }
/// PostgresStruct::generate_values_clause(); // 输出：["$1", "$4"]
/// MysqlStruct::last_param_index(); // 2
///
/// #[derive(GenValues)]
/// #[config(database = "mysql")]
/// struct MysqlStruct {
///     field1: i32,
///     #[value(ignore)]
///     field2: i32,
///     #[value(index = 4)] // mysql 并不需要占位符，所以不需要 `index`
///     field3: i32,
/// }
/// MysqlStruct::generate_values_clause(); // 输出：["?", "?"]
/// PostgresStruct::last_param_index(); // 2
///
/// // 设置开始的索引
/// #[derive(GenValues)]
/// #[config(database = "postgres", index = 5)]
/// struct PostgresSetIndexStruct {
///     field1: i32,
///     field2: i32,
///     field3: i32,
/// }
/// PostgresSetIndexStruct::generate_values_clause(); // 输出["$5", "$6", "$7"]
/// PostgresSetIndexStruct::last_param_index(); // 7
/// ```
/// 此函数将为 `MyStruct` 生成相应的 `FieldsAttributeMacro` 实现。
#[proc_macro_derive(GenValues, attributes(value, config))]
pub fn values_attribute_macro(item: TokenStream) -> TokenStream {
    values::gen_values_attribute_impl(item)
}
