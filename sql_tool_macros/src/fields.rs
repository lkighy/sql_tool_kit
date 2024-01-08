extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Meta, Token};

use syn::punctuated::Punctuated;

/// 生成针对特定结构体的 `FieldsAttributeMacro` 实现。
///
/// 此函数解析标记在结构体字段上的 `#[fields(...)]` 属性宏，
/// 并生成一个实现 `FieldsAttributeMacro` trait 的代码块。
/// 它处理每个字段的 `ignore` 和 `rename` 指令，以生成相应的字段列表。
///
/// # 参数
/// * `item`: TokenStream，表示要处理的 Rust 代码项（一般是结构体定义）。
///
/// # 返回值
/// 返回一个 `TokenStream`，它包含了生成的 `FieldsAttributeMacro` trait 实现。
///
/// 此函数将为 `MyStruct` 生成相应的 `FieldsAttributeMacro` 实现。
pub fn gen_fields_attribute_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let fields = if let Data::Struct(data_struct) = &input.data {
        let mut fields = Vec::new();
        for field in &data_struct.fields {
            let mut field_name = Some(field.ident.as_ref().unwrap().to_string());
            let attrs = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("field"));

            if let Some(attr) = attrs {
                let nested = attr
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .map_err(|e| {
                        println!("分析 'field' 属性时出错");
                        e
                    })
                    .unwrap();

                for meta in nested {
                    match meta {
                        Meta::Path(_) if meta.path().is_ident("ignore") => {
                            field_name = None;
                            break;
                        }
                        Meta::NameValue(name_value) if meta.path().is_ident("rename") => {
                            if let Expr::Lit(value) = &name_value.value {
                                if let Lit::Str(val) = &value.lit {
                                    field_name = Some(val.value());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(value) = field_name {
                fields.push(value);
            }
        }

        fields
    } else {
        Vec::new()
    };

    let expanded = quote! {
        impl FieldsAttributeMacro for #name {
            fn generate_fields_clause() -> Vec<String> {
                vec![#(#fields.to_string()),*]
            }
        }
    };

    TokenStream::from(expanded)
}
