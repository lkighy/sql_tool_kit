extern crate proc_macro;
use proc_macro::TokenStream;
// use std::any::Any;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Meta, Token, Type};

use syn::punctuated::Punctuated;
use crate::macro_utils::generate_placeholder;

///
pub fn gen_where_attribute_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let mut index = 1;
    let mut placeholder = String::new();
    let mut ignore_none = true;

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
                        placeholder = generate_placeholder(value.value().as_str());
                        continue;
                    }
                }
                panic!("database 值转换失败")
            }
            Meta::NameValue(name_value) if meta.path().is_ident("ignore_none") => {
                if let Expr::Lit(value) = &name_value.value {
                    if let Lit::Bool(value) = &value.lit {
                        ignore_none = value.value;
                    }
                }
            }
            Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                if let Expr::Lit(value) = &name_value.value {
                    if let Lit::Int(value) = &value.lit {
                        index = value.base10_parse::<usize>().unwrap();
                    }
                }
            }
            _ => {}
        }
    }

    let values = if let Data::Struct(data_struct) = &input.data {
        let mut fields = Vec::new();
        for field in &data_struct.fields {
            let mut where_value = Some("{name} {condition} {index}".to_string());
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_value = field.ident.clone().unwrap();

            let attrs = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("r#where"));

            let mut ignore_none = ignore_none;
            let mut condition_all = String::new();
            let mut condition = "=".to_string() ;
            let mut rename = String::new();
            let mut value_placeholder = String::new();
            let mut field_index = -1;

            if let Some(attr) = attrs {
                let nested = attr
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .map_err(|e| {
                        println!("分析 'value' 属性时出错");
                        e
                    }).unwrap();


                for meta in nested {
                    match meta {
                        Meta::Path(_) if meta.path().is_ident("ignore") => {
                            where_value = None;
                            break;
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("ignore_none") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Bool(val) = &value.lit {
                                    ignore_none = val.value();
                                }
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("condition_all") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Str(val) = &value.lit {
                                    condition_all = val.value();
                                }
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("rename") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Str(val) = &value.lit {
                                    rename = val.value();
                                }
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("condition") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Str(val) = &value.lit {
                                    condition = val.value();
                                }
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("value") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Str(val) = &value.lit {
                                    value_placeholder = val.value();
                                }
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Int(val) = &value.lit {
                                    field_index = val.base10_parse::<i32>().unwrap();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(mut field) = where_value {
                    // 使用 condition_all 或原始 field
                    field = if !condition_all.is_empty() { condition_all } else { field };

                    // 替换 {name}
                    let name_to_use = if rename.is_empty() { &field_name } else { &rename };
                    field = field.replace("{name}", name_to_use);

                    // 检查并替换 {condition}
                    if condition.is_empty() && field.contains("{condition}") {
                        panic!("存在 {{condition}} 但是在字段 {} 的属性宏上没有设置 condition 属性", field_name);
                    }
                    field = field.replace("{condition}", &condition);

                    // 替换 {index} 和 {value}
                    let index_str = if field_index != -1 { field_index.to_string() } else { "{index}".to_string() };
                    let placeholder_str = if value_placeholder.is_empty() { placeholder.replace("{index}", &index_str) } else { value_placeholder.clone() };
                    field = field.replace("{index}", &placeholder_str);

                    where_value = Some(field);
                }

            } else {
                if let Some(mut value) = where_value {
                    value = value.replace("{name}", &field_name);
                    value = value.replace("{condition}", &condition);
                    value = value.replace("{index}", &placeholder.replace("{index}", "{index}"));
                    where_value = Some(value);
                }
            }
            if let Some(value) = where_value {
                let get_data = if let Type::Path(type_path) = &field.ty  {
                    if ignore_none && type_path.path.segments.first().map_or(false, |segment| segment.ident == "Option") {
                        quote!{
                            if self.#field_value.is_some() {
                                Some(#value)
                            } else {
                                None
                            }
                        }
                    } else {
                        quote_spanned!{field.span() => Some(#value)}
                    }
                } else {
                    quote_spanned!{field.span() => Some(#value)}
                };
                // let get_data = quote_spanned!{field.span() => Some(#value)};
                fields.push(get_data);
            }
        }
        fields
    } else {
        Vec::new()
    };

    let expanded = quote! {
        impl WhereAttributeMacro for #name {
            fn generate_where_clause(&self) -> Vec<String> {
                let mut fields = Vec::new();
                let mut index = #index;
                #(
                    // 使用 values 中的每个 TokenStream
                    if let Some(value) = #values {
                        let value = value.replace("{index}", &index.to_string());
                        index += 1;
                        fields.push(value);
                    }
                )*
                fields
            }

            fn last_param_index(&self) -> usize {
                #index
            }
        }
    };

    TokenStream::from(expanded)
}


// {
// 当该段落的属性都遍历完成是，进行数据通过
// if let Some(field) = where_value {
//     let mut field = field;
//     if !condition_all.is_empty() {
//         field = condition_all;
//     }
//     if rename.is_empty() {
//         field = field.replace("{name}", field_name);
//     } else {
//         field = field.replace("{name}", rename);
//     }
//     if condition.is_empty() {
//         if condition_all.contains("{condition}") {
//             panic!("存在 {{condition}} 但是在字段 {} 的属性宏上没有设置 condition 属性", field_name);
//         }
//     } else {
//         field = field.replace("{condition}", condition);
//     }
//     if !value_placeholder.is_empty() {
//         // 不进行任何操作
//     } else if field_index != -1 {
//         value_placeholder = placeholder.replace("{index}", field_index.to_string());
//     } else {
//         value_placeholder = placeholder.replace("{index}", index.to_string());
//     }
//     field = field.replace("{value}", value_placeholder);
// }
// }
