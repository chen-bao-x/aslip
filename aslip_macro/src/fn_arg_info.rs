// lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, FnArg, Ident, ItemFn, Lit, Pat, Type, meta::ParseNestedMeta, parse_macro_input,
};

extern crate proc_macro;

#[derive(Debug, PartialEq, Eq,Clone)]
pub struct FnArgInfo {
    name: String,
    _type: String,
    fn_arg_doc: Vec<String>,
}

impl FnArgInfo {
    pub fn new(arg: syn::FnArg) -> Option<FnArgInfo> {
        // 过滤掉 self 参数（仅处理普通参数）
        if let FnArg::Typed(pat_type) = arg {
            // 提取参数名称（模式中的标识符）
            let arg_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                // 处理非标识符参数（如元组、结构体等，这里简化处理）
                &Ident::new("unknown_arg", proc_macro2::Span::call_site())
            };

            // 提取参数类型
            let arg_type = &pat_type.ty;

            // 打印参数信息（编译期输出）
            println!(
                "参数名称: {}, 参数类型: {}",
                arg_name,
                quote!(#arg_type) // 将类型转换为字符串
            );

            let doc = pat_type
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path().is_ident("about") {
                        // 解析 #[about = "..."]
                        if let syn::Meta::NameValue(meta_nv) = &attr.meta {
                            if let syn::Expr::Lit(expr_lit) = &meta_nv.value {
                                if let syn::Lit::Str(litstr) = &expr_lit.lit {
                                    return Some(litstr.value());
                                }
                            }
                        }
                    }

                    None
                })
                .collect::<Vec<String>>();

            return Some(FnArgInfo {
                name: arg_name.to_string(),
                _type: quote!(#arg_type).to_string(),
                fn_arg_doc: doc,
            });
        }

        return None;
    }
}
