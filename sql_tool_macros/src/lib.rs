use proc_macro::TokenStream;

mod fields;
mod macro_utils;
mod select;
mod set;
mod values;
mod where_macro;

/// 过程宏入口点，用于处理 `#[field(...)]` 属性宏。
///
/// 此宏处理结构体定义上的 `#[field(...)]` 属性，用于生成符合 `FieldsAttributeMacro`
/// trait 的实现。这允许在结构体字段级别上定义如何处理 SQL 字段，例如忽略某些字段，
/// 或将字段重命名。
///
/// 字段宏参数：
/// - `ignore` - 忽略该字段
/// - `rename` - 字段重命名
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
/// 字段宏参数：
/// - `ignore` - 忽略该字段
/// - `rename` - 字段重命名
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
/// 宏参数：
/// - `#[config(...)]`: 设置全局配置。
///   - `database` - 指定生成的数据库类型，目前支持 `postgresql` `mysql` `mariadb` `sqlite` `mssql`
///   - `index` - 指定开始的序列，仅 `postgresql` `mssql` 上有效
///
/// `#[value(...)]` 接受的参数：
/// - `ignore` - 忽略该字段
///   - `index` - 设置当前值的index，当设置了这个参数后，全局的 index 不会加一
///   - `value` - 直接替换当前的 `${index}` ，当设置了这个参数后，全局的 index 不会加一
///       - 例如：`#[value(value = "true")]` => ["$1", "true",...]
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

/// `GenWhere` 派生宏
///
/// 用于生成 SQL `WHERE` 语句部分。此宏依赖于 `WhereAttributeMacro` trait。
/// 使用方法 `where_data.generate_where_clause()` 会返回一个字段和条件组成的字符串数组。
///
/// 宏参数：
/// - `#[config(...)]`: 设置全局配置。
///   - `database`: 指定数据库类型，影响占位符格式（支持 postgres, mysql, sqlite, mariadb, mssql）。
///   - `index`: 设置占位符的起始索引。
///   - `ignore_none`: 是否忽略 `Option::None` 值，默认为 `true`。
///   - `ignore_no_macro_where`: 是否忽略没有 `#[r#where(...)]` 宏的字段，默认值为 `true`, 为 `true` 时配合 `GenSet` 宏使用。
///
/// - `#[r#where(...)]`: 字段级别宏，用于自定义字段在 `WHERE` 语句中的表现。
///   - `ignore`: 忽略该字段。
///   - `rename`: 字段重命名，接受字符串类型。
///   - `condition`: 指定字段的比较条件，默认值为 ”=“，如果该值设置为空及 "", 会报错。
///   - `condition_all`: 应用于所有字段的通用条件，缺省值为 `"{name} {condition} {index}"`。
///     - `{name}`: 字段名称或 `rename` 指定的名称。
///     - `{condition}`: `condition` 参数指定的比较条件。如果 `condition_all`。
///     - `{index}`: `index` 参数指定的占位符索引。如果字段未设置 `index`，则使用全局 `index`。
///   - `ignore_none`: 当字段为 `Option::None` 时是否忽略，接受布尔类型。
///   - `value`: 自定义字段的值，接受字符串类型。
///   - `index`: 自定义占位符序号（如果数据库支持），接受整型。
///
/// 字段宏属性优先级：
/// `ignore` > `ignore_none` > `condition_all` > `rename` = `condition` = `value` > `index`
///
/// 示例：
/// ```rust
/// #[derive(GenWhere, Debug)]
/// #[config(database = "postgres")]
/// pub struct WhereStruct {
///     // 字段定义
///     // ...
/// }
///
/// fn main() {
///     let data = WhereStruct {
///         // 初始化字段
///         // ...
///     };
///     println!("{:?}", data.generate_where_clause());
/// }
/// ```
#[proc_macro_derive(GenWhere, attributes(r#where, config))]
pub fn where_attribute_macro(item: TokenStream) -> TokenStream {
    where_macro::gen_where_attribute_impl(item)
}

/// `GenSet` 派生宏
///
/// 用于生成 SQL `UPDATE` 语句中的 `SET` 部分。它依赖于 `SetAttributeMacro` trait。
/// 例如，`update table_name set field1 = $1, field2 = $2 ... where ...`
/// 使用方法 `update_data.generate_set_clause()` 返回值类似于 `["field1 = $1", "field2 = $2", ...]`。
///
/// 宏参数：
/// - `#[config(...)]`: 设置一些配置。
///   - `database`: 指定数据库类型，影响占位符的格式（支持 mysql, postgres, sqlite, mariadb, mssql）。
///   - `index`: 设置占位符的起始索引。
///   - `ignore_none`: 是否忽略 `Option::None` 值，默认为 `true`。
///   - `ignore_no_macro_set`: 默认忽略没有 `#[set(...)]` 宏的字段，为 `true` 时配合 `GenWhere` 宏使用。
///   - `ignore_set_and_where`: 当 `#[set(...)]`存在 `where` 参数是，会忽略 `set` 值，默认为 `false`
///
/// - `#[set(...)]`: 字段级别的宏，用于自定义字段在生成的 `SET` 语句中的表现。
///   - `ignore`: 忽略该字段。
///   - `r#where`: 将该字段设置为 where，有多种使用方式。1. `#[set(r#where)]` `#[set(r#where = "{field = {index}")]`
///   - `ignore_none`: 当字段为 `Option::None` 时是否忽略，接受布尔类型。
///   - `ignore_set`: 在 set 上忽略该字段。
///   - `rename`: 字段重命名，接受字符串类型。
///   - `condition`: 当设置 `r#where` 时生效
///   - `value`: 自定义字段的值，接受字符串类型。
///   - `index`: 自定义占位符序号（如果数据库支持），接受整型。
//
/// 宏的优先级：`ignore` > `ignore_none` > `r#where` = `ignore_set` > `rename` = `value` = `condition` > `index`
///
/// 示例：
/// #[doc = "hidden"]
/// #[cfg(test)]
/// ```rust
/// #[derive(GenSet, Debug)]
/// #[config(database = "postgres", index = 4)]
/// pub struct SetStruct {
///     #[set(rename = "id")]
///     pub field1: i32,
///     #[set()]
///     pub field2: i32,
///     #[set(rename = "email")]
///     pub field3: Option<String>,
///     #[set(value = "'用户名称'")] // 设置 field = '用户名称' 而不是 ${index}
///     pub field4: String,
///     #[set(index = 10)] // 设置当前字段的 索引
///     pub field5: String,
///     #[set()]
///     pub field6: String,
/// }
///
/// #[cfg(test)]
/// fn set_macro_test() {
///     let data = SetStruct {
///         field1: 12,
///         // 初始化其他字段...
///         field2: 0,
///         field3: None, // 为 None 值会默认被忽略
///         field4: "".to_string(),
///         field5: "".to_string(),
///         field6: "".to_string(),
///     };
///     let set_values = vec![
///         "id = $4".to_string(),
///         "email = $5".to_string(),
///         "field4 = '用户名称'".to_string(),
///         "field5 = $10".to_string(),  // 自带的 index 不会影响接下来的序列
///         "field6 = $8".to_string(), // 这里会是8是因为 field4 和 field5 会
///     ];
///     assert_eq!(set_values, data.generate_set_clause());
/// }
/// ```
#[proc_macro_derive(GenSet, attributes(set, config))]
pub fn set_attribute_macro(item: TokenStream) -> TokenStream {
    set::gen_set_attribute_impl(item)
}
