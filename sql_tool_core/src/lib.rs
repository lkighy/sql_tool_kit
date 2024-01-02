
/// `FieldsAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[fields(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[fields]` 宏指定的参数生成对应的字段列表。
pub trait FieldsAttributeMacro {
    /// 解析 `#[fields(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[fields]` 宏参数，如 `ignore` 和 `rename`，
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `RETURNING` 或 `INSERT`, 也可用于 `SELECT` 部分。
    /// 对于 `SELECT` 部分，更推荐使用 `SelectAttributeMacro`
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    fn generate_fields_clause() -> Vec<String>;
}

// 目前需要考虑到的内容有：
// 1. 生成查询字段，需要有 query_field, returning_field, ignore
// 2. 生成 update 参数 需要有 set(key = $1), where(condition [] $3), ignore
// 3. 生成 insert 语句需要的参数 fields([field1, field2, field3]), values($1, $2, $3), ignore
// 4. 生成 delete 语句需要的参数 where(condition [] $3), ignore
// 5. 生成 select 语句需要的参数
//
// 最终整理有：
// set [field1 = $1, field2 = $2, ...]
// where [field1 [condition] $1, field2 [condition] $2, ...]
// values ([$1, $2, $3, ...])
// fields ([field1, field2, field3, ...])
// select ([select1, select2, select3, ...])

/// `SelectAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[select(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[select]` 宏指定的参数生成对应的字段列表。
pub trait SelectAttributeMacro {
    /// 解析 `#[select(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[select]` 宏参数，如 `ignore` 和 `rename`，
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `SELECT` 部分, 也可用于 `RETURNING` 或 `INSERT`。
    /// 对于 `RETURNING` 或 `INSERT` 更推荐使用 `FieldsAttributeMacro`
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    fn generate_select_clause() -> Vec<String>;
}

/// `ValuesAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[values(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[values]` 宏指定的参数生成对应的字段列表。
pub  trait ValuesAttributeMacro {
    /// 解析 `#[values(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[values]` 宏参数，如 `ignore` 和  `index`
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `VALUES` 部分。
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    /// 数据库选择为 Postgres 数据应返回为 ["$1", "$2", "$3", ...]
    /// 数据库选择为 MySql 数据应返回为 ["?", "?", ...]
    fn generate_values_clause() -> Vec<String>;
    /// 根据 `generate_values_clause` 最终生成的列表，返回最后的占位符索引号
    /// 例如 Postgres 中最末尾为 `$3`, 则此处应该返回 `3`
    fn last_param_index() -> usize;
}

/// `WhereAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[where(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[where]` 宏指定的参数生成对应的字段列表。
pub trait WhereAttributeMacro {
    /// 解析 `#[where(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[where]` 宏参数，如 `ignore`, `rename`, `index`, `condition` 和 `condition_all`
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `WHERE` 部分。
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    /// 数据库选择为 Postgres 数据应返回为 ["field1 [condition] $1", "[condition_all]", "[rename] [condition] $3", "field4 [condtion] $[index]"]
    /// 数据库选择为 MySql 数据应返回为 ["field1 [condition] ?", "[condition_all]", "[rename] [condition] ?", "field4 [condtion] ?"]
    fn generate_where_clause() -> Vec<String>;
    /// 返回最终的索引号
    fn last_param_index() -> usize;
}

/// `SetAttributeMacro` trait 定义了处理字段属性宏的功能。
///
/// 这个 trait 主要用于解析和处理 `#[set(...)]` 属性宏，该宏用于
/// 定制 SQL 语句中的字段部分。通过实现这个 trait，可以根据结构体
/// 字段上的 `#[set]` 宏指定的参数生成对应的字段列表。
pub trait SetAttributeMacro {
    /// 解析 `#[set(...)]` 属性宏，并生成字段列表。
    ///
    /// 此方法会分析结构体字段上的 `#[set]` 宏参数，如 `ignore`, `rename`, `index`, 和 `where`
    /// 并据此生成对应的字段名称列表。这个列表通常用于构建 SQL 语句的 `WHERE` 部分。
    ///
    /// 返回值是一个包含字段名称的 `String` 向量。
    /// 数据应返回为 ["field1 = $1", "field1 = $[2]", "[rename] = $3", ...]
    fn generate_set_clause() -> Vec<String>;
    /// 解析 `#[set(where = "{field} > {index}")]`, 并生成相应的条件列表。
    /// 对应的使用方法有：
    /// `where = "{field} > $1"`, 不存在index 则不进行替换
    /// `where = "field4 > {index}"`, 只替换index
    /// `where = "field4 NOT NULL"`, 不进行任何替换
    ///
    /// 返回值是一个包含条件的 `String` 向量。
    /// 数据应返回为 ["field4 [where] $4", "rename [where] ${index}"]
    /// 如果想要使用好的自定义方法，应该使用 `GenWhere` 宏， 并使用 `ignore` 忽略指定值
    fn generate_where_clause() -> Vec<String>;
    /// 返回最终的索引号
    fn  last_param_index() -> usize;
}