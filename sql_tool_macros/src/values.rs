extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Meta, Token};

use crate::macro_utils::from_name_value;
use syn::punctuated::Punctuated;

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
pub fn gen_values_attribute_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let mut index = 1;
    let mut placeholder = "";

    let attrs = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("config"))
        .expect("必须在结构上设置 `#[config(database = \"/*数据库类型*/\")]` 宏");

    let nested = attrs
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .map_err(|e| {
            println!("分析字段属性时出错");
            e
        })
        .unwrap();

    for meta in nested {
        match meta {
            Meta::NameValue(name_value) if meta.path().is_ident("database") => {
                if let Expr::Lit(value) = &name_value.value {
                    if let Lit::Str(value) = &value.lit {
                        placeholder = match value.value().as_str() {
                            "postgres" => "${index}",   // PostgreSQL 使用 $1, $2, ...
                            "mysql" | "mariadb" => "?", // MySQL 和 MariaDB 使用 ?
                            "sqlite" => "?",            // SQLite 也是使用 ?
                            "mssql" => "@p{index}",     // Microsoft SQL Server 使用 @p1, @p2, ...
                            _ => panic!("未支持的数据库类型"),
                        };
                        continue;
                    }
                }
                panic!("database 值转换失败")
            }
            Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                if let Some(Lit::Int(value)) = &from_name_value(&name_value) {
                    index = value.base10_parse::<usize>().unwrap();
                }
            }
            _ => {}
        }
    }

    if placeholder.is_empty() {
        panic!("database 值必须设置");
    }

    let values = if let Data::Struct(data_struct) = &input.data {
        let mut fields = Vec::new();
        for field in &data_struct.fields {
            let mut field_name = Some(placeholder.replace("{index}", &index.to_string()));
            let mut add_index = 1;
            let attrs = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("value"));

            if let Some(attr) = attrs {
                let nested = attr
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .map_err(|e| {
                        println!("分析 'value' 属性时出错");
                        e
                    })
                    .unwrap();

                for meta in nested {
                    match meta {
                        Meta::Path(_) if meta.path().is_ident("ignore") => {
                            field_name = None;
                            break;
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                            if let Some(Lit::Int(value)) = &from_name_value(&name_value) {
                                field_name =
                                    Some(placeholder.replace("{index}", &value.to_string()));
                                add_index = 0;
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(value) = field_name {
                fields.push(value);
                index += 1;
            }
        }

        fields
    } else {
        Vec::new()
    };

    let expanbded = quote! {
        impl ValuesAttributeMacro for #name {
            fn generate_values_clause() -> Vec<String> {
                vec![#(#values.to_string()),*]
            }
            // fn last_param_index() -> usize {
            //     #index
            // }
        }
    };

    TokenStream::from(expanbded)
}
