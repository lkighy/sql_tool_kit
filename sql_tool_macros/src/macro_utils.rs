use syn::{Expr, Lit, MetaNameValue};

/// 生成数据库特定的查询参数占位符模板。
///
/// 此函数根据指定的数据库类型生成一个占位符模板字符串。返回的占位符模板
/// 包含 `{index}`，这应当在实际使用中被替换为具体的参数索引值。
///
/// # 参数
/// - `database`: 要生成占位符的数据库类型，如 "postgres"、"mysql"、"sqlite" 或 "mssql"。
///
/// # 返回值
/// 返回含有 `{index}` 的数据库特定的占位符模板字符串。
///
/// # Panics
/// 如果提供了不支持的数据库类型，则函数将 panic。
///
/// # 示例
/// ```
/// let placeholder_template = generate_placeholder("postgres");
/// let placeholder = placeholder_template.replace("{index}", "1");
/// assert_eq!(placeholder, "$1");
/// ```
pub fn generate_placeholder(database: &str) -> String {
    match database {
        "postgres" => "${index}".to_string(),
        "mysql" | "mariadb" => "?".to_string(),
        "sqlite" => "?".to_string(),
        "mssql" => "@p{index}".to_string(),
        _ => panic!("未支持的数据库类型"),
    }
}

pub fn name_value_to_string(name_value: &MetaNameValue) -> Option<String> {
    if let Expr::Lit(value) = &name_value.value {
        if let Lit::Str(val) = &value.lit {
            return Some(val.value());
        }
    }
    None
}

pub fn name_value_to_bool(name_value: &MetaNameValue) -> Option<bool> {
    if let Expr::Lit(value) = &name_value.value {
        if let Lit::Bool(val) = &value.lit {
            return Some(val.value());
        }
    }
    None
}

pub fn from_name_value<T>(name_value: &MetaNameValue) -> Option<T>
where
    T: From<Lit>,
{
    if let Expr::Lit(value) = &name_value.value {
        return Some(T::from(value.lit.clone()));
    }
    None
}
