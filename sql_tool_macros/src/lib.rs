
use proc_macro::TokenStream;

mod fields;
mod select;

/// 过程宏入口点，用于处理 `#[fields(...)]` 属性宏。
///
/// 此宏处理结构体定义上的 `#[fields(...)]` 属性，用于生成符合 `FieldsAttributeMacro`
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
///     #[fields(ignore)]
///     field2: i32,
///     #[fields(rename = "rename_field")]
///     field3: i32,
/// }
///
/// MyStruct::generate_fields_clause(); // 输出：["field1", "rename_field"]
/// ```
#[proc_macro_derive(GenFields, attributes(fields))]
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
