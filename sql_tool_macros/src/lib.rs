
use proc_macro::TokenStream;

mod fields;

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
/// #[derive(GenFields, Debug)]
/// struct MyStruct {
///     #[fields(ignore)] // 这段将被忽略
///     field1: i32,
///     #[fields(rename = "rename_field")]
///     field2: i32,
///     field3: i32,
///     // ...
/// }
///
/// MyStruct::generate_fields_clause() // 输出： ["rename_field"]
/// ```
///
///  # 输出
///
///
#[proc_macro_derive(GenFields, attributes(fields))]
pub fn fields_attribute_macro(item: TokenStream) -> TokenStream {
    fields::gen_fields_attribute_impl(item)
}
