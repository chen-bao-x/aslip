use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, FnArg, Ident, ItemFn, Lit, Pat, Type, meta::ParseNestedMeta, parse_macro_input,
};

use crate::FnArgInfo;

extern crate proc_macro;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnInfo {
    pub func_name: String,
    pub func_args: Vec<FnArgInfo>,
    pub func_doc_comments: Vec<String>,
}

impl FnInfo {
    pub fn new(fn_item: &ItemFn) -> FnInfo {
        // 1. 获取函数名称
        let fn_name = &fn_item.sig.ident;

        // 2. 遍历参数，提取参数名称和类型
        let mut args = Vec::<FnArgInfo>::new();
        {
            for arg in &fn_item.sig.inputs {
                let f = FnArgInfo::new(arg.clone()).unwrap();
                args.push(f);
            }
        }

        // 3. 从属性中提取文档注释
        let doc_comments = FnInfo::get_doc_comments(fn_item.clone());

        FnInfo {
            func_name: quote!(#fn_name).to_string(),
            func_args: args,
            func_doc_comments: doc_comments,
        }
    }

    // fn get_doc_comments(tokens: TokenStream) -> Vec<String> {
    pub fn get_doc_comments(item_fn: ItemFn) -> Vec<String> {
        item_fn
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc") {
                    // #[doc = "xxx"]
                    match attr.meta.clone() {
                        syn::Meta::NameValue(nv) => {
                            if let syn::Expr::Lit(expr_lit) = nv.value {
                                if let syn::Lit::Str(litstr) = expr_lit.lit {
                                    return Some(litstr.value());
                                }
                            }
                            None
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn gen_case_code(&self) -> String {
        format!(
            r###"
    "{func_name}" => 
            {{

        {func_name}();
        }}  ,
        "###,
            func_name = self.func_name,
        )
    }
}
