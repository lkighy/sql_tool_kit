use crate::macro_utils::{
    from_name_value, generate_placeholder, name_value_to_bool, name_value_to_string,
};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, Token, Type};

pub fn gen_set_attribute_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let mut index = 1;
    let mut placeholder = String::new();
    let mut ignore_none = true;
    let mut ignore_no_macro_set = true;
    let mut ignore_set_and_where = false;

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
                if let Some(value) = name_value_to_string(&name_value) {
                    placeholder = generate_placeholder(value.as_str());
                    continue;
                }
                panic!("database 值转换失败")
            }
            Meta::NameValue(name_value) if meta.path().is_ident("ignore_none") => {
                if let Some(value) = name_value_to_bool(&name_value) {
                    ignore_none = value;
                }
            }
            Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                if let Some(Lit::Int(value)) = &from_name_value(&name_value) {
                    index = value.base10_parse::<usize>().unwrap();
                }
            }
            Meta::NameValue(name_value) if meta.path().is_ident("ignore_no_macro_set") => {
                if let Some(value) = name_value_to_bool(&name_value) {
                    ignore_no_macro_set = value;
                }
            }
            Meta::NameValue(name_value) if meta.path().is_ident("ignore_set_and_where") => {
                if let Some(value) = name_value_to_bool(&name_value) {
                    ignore_set_and_where = value
                }
            }
            _ => {}
        }
    }

    if placeholder.is_empty() {
        panic!("database 值必须设置");
    }

    let (set_values, where_values) = if let Data::Struct(data_struct) = &input.data {
        let mut sets = Vec::new();
        let mut wheres = Vec::new();

        for field in &data_struct.fields {
            let mut set_value = Some("{name} = {index}".to_string());
            let mut where_value = None;
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_value = field.ident.clone().unwrap();
            let mut ignore_none = ignore_none;
            let mut rename = String::new();
            let mut condition = "=".to_string();
            let mut value_placeholder = String::new();
            let mut field_index = -1;
            let mut add_index: usize = 0;

            let attrs = field.attrs.iter().find(|attr| attr.path().is_ident("set"));

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
                            set_value = None;
                            where_value = None;
                            break;
                        }
                        Meta::Path(_) if meta.path().is_ident("ignore_set") => {
                            set_value = None;
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("r#where") => {
                            if let Some(value) = name_value_to_string(&name_value) {
                                where_value = Some(value);
                                if ignore_set_and_where {
                                    set_value = None;
                                }
                            }
                        }
                        Meta::Path(_) if meta.path().is_ident("r#where") => {
                            where_value = Some("{name} {condition} {index}".to_string());
                            set_value = None;
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("ignore_none") => {
                            if let Some(value) = name_value_to_bool(&name_value) {
                                ignore_none = value;
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("rename") => {
                            if let Some(value) = name_value_to_string(&name_value) {
                                rename = value;
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("condition") => {
                            if let Some(value) = name_value_to_string(&name_value) {
                                condition = value;
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("value") => {
                            if let Some(value) = name_value_to_string(&name_value) {
                                value_placeholder = value;
                            }
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("index") => {
                            if let Some(Lit::Int(value)) = &from_name_value(&name_value) {
                                field_index = value.base10_parse::<i32>().unwrap();
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(mut field) = set_value {
                    let name_to_use = if rename.is_empty() {
                        &field_name
                    } else {
                        &rename
                    };
                    field = field.replace("{name}", name_to_use);

                    // 替换 {index} 和 {value}
                    let index_str = if field_index != -1 {
                        field_index.to_string()
                    } else {
                        "{index}".to_string()
                    };
                    let placeholder_str = if value_placeholder.is_empty() {
                        placeholder.replace("{index}", &index_str)
                    } else {
                        value_placeholder.clone()
                    };
                    field = field.replace("{index}", &placeholder_str);

                    set_value = Some(field);
                }
                if let Some(mut field) = where_value {
                    let name_to_use = if rename.is_empty() {
                        &field_name
                    } else {
                        &rename
                    };
                    field = field.replace("{name}", name_to_use);

                    field = field.replace("{condition}", &condition);

                    // 替换 {index} 和 {value}
                    let index_str = if field_index != -1 {
                        field_index.to_string()
                    } else {
                        "{index}".to_string()
                    };

                    let placeholder_str = if value_placeholder.is_empty() {
                        placeholder.replace("{index}", &index_str)
                    } else {
                        value_placeholder.clone()
                    };
                    field = field.replace("{index}", &placeholder_str);

                    where_value = Some(field);
                }
            } else {
                if let (false, Some(mut value)) = (ignore_no_macro_set, set_value.clone()) {
                    value = value.replace("{name}", &field_name);
                    value = value.replace("{index}", &placeholder.replace("{index}", "{index}"));
                    set_value = Some(value);
                } else {
                    set_value = None;
                }
            }

            if field_index == -1 && value_placeholder.is_empty() {
                add_index = 1;
            }

            if let Some(value) = set_value {
                let get_data = if let Type::Path(type_path) = &field.ty {
                    if ignore_none
                        && type_path
                            .path
                            .segments
                            .first()
                            .map_or(false, |segment| segment.ident == "Option")
                    {
                        quote! {
                            if self.#field_value.is_some() {
                                (Some(#value), #add_index)
                            } else {
                                (None, #add_index)
                            }
                        }
                    } else {
                        quote_spanned! {field.span() => (Some(#value), #add_index)}
                    }
                } else {
                    quote_spanned! {field.span() => (Some(#value), #add_index)}
                };
                sets.push(get_data);
            }

            if let Some(value) = where_value {
                if field_index == -1 && value_placeholder.is_empty() && value.contains("{index}") {
                    add_index = 1;
                } else {
                    add_index = 0;
                }
                let get_data = if let Type::Path(type_path) = &field.ty {
                    if ignore_none
                        && type_path
                            .path
                            .segments
                            .first()
                            .map_or(false, |segment| segment.ident == "Option")
                    {
                        quote!(
                            if set.#field_value.is_some() {
                                (Some(#value), #add_index)
                            } else {
                                (None, #add_index)
                            }
                        )
                    } else {
                        quote_spanned! {field.span() => (Some(#value), #add_index)}
                    }
                } else {
                    quote_spanned! {field.span() => (Some(#value), #add_index)}
                };
                wheres.push(get_data);
            }
        }
        (sets, wheres)
    } else {
        (Vec::new(), Vec::new())
    };

    let expanded = quote! {
        impl SetAttributeMacro  for #name {
            fn generate_set_clause(&self) -> Vec<String> {
                let mut fields = Vec::new();
                let mut index = #index;
                #(
                    // 使用 values 中的每个 TokenStream
                    if let (Some(value), add_index) = #set_values {
                        let value = value.replace("{index}", &index.to_string());
                        index = index + add_index;
                        fields.push(value);
                    }
                )*
                fields
            }
            fn generate_set_and_where_clause(&self) -> (Vec<String>, Vec<String>) {
                let mut set_data = Vec::new();
                let mut where_data = Vec::new();
                let mut index = #index;
                #(
                    if let (Some(value), add_index) = #set_values {
                        let value = value.replace("{index}", &index.to_string());
                        index = index + add_index;
                        set_data.push(value);
                    }
                )*
                #(
                    if let (Some(value), add_index) = #where_values {
                        let value = value.replace("{index}", &index.to_string());
                        index = index + add_index;
                        where_data.push(value);
                    }
                )*
                (set_data, where_data)
            }
        }
    };

    TokenStream::from(expanded)
}
