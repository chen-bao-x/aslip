// lib.rs

use quote::quote;
use syn::FnArg;
use syn::Ident;
use syn::Pat;

extern crate proc_macro;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnArgInfo {
    arg_name: String,
    type_name: String,
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
            // panic!("arg_type: {}",  quote!(#arg_type));

            // 打印参数信息（编译期输出）
            // println!(
            //     "参数名称: {}, 参数类型: {}",
            //     arg_name,
            //     quote!(#arg_type) // 将类型转换为字符串
            // );

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
                arg_name: arg_name.to_string(),
                type_name: quote!(#arg_type)
                    .to_string()
                    .trim_matches(|x| x == ' ')
                    .to_string(),
                fn_arg_doc: doc,
            });
        }

        return None;
    }

    /// returns.0 -> auto_name
    /// returns.1 -> the let expr.
    pub fn gen_code(&self, index: usize) -> (String, String) {
        let auto_name = self.auto_name(index);
        let _converted = auto_name.clone() + "_converted";

        let get_arg_code = format!(
            "let {} = app._user_inputed_cmd_args.get({}).unwrap().clone();",
            auto_name, index,
        );

        let convert_right_sede_code = self.type_convert(&self.type_name, &auto_name);

        let type_convert_code = format!(
            "let {}: {} = {};",
            _converted, self.type_name, convert_right_sede_code,
        );

        return (_converted.clone(), get_arg_code + "\n" + &type_convert_code);
    }

    fn type_convert(&self, ty_name: &str, cmd_arg_variable_name: &str) -> String {
        format!(
            "<{} as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str(&{}).unwrap().clone()",
            ty_name, cmd_arg_variable_name
        )
    }

    fn auto_name(&self, index: usize) -> String {
        // format!("arg_{}_{}", index, self.type_name.to_lowercase())
        format!("__arg_{}__", index,)
    }
}
