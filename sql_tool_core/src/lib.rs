
/// `FieldsAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[fields(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[fields]` 宏指定的参数生成对应的字段列表。
pub trait FieldsAttributeMacro {
    /// 解析 `#[fields(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[fields]` 宏参数，如 `ignore` 和 `rename`，
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `SELECT` 或 `INSERT` 部分。
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    fn generate_fields_clause() -> Vec<String>;
}