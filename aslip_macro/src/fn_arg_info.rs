// lib.rs

use quote::quote;
use syn::FnArg;
use syn::Ident;
use syn::Pat;

extern crate proc_macro;

#[derive(Debug, Clone)]
pub struct FnArgInfo {
    pub arg_name: String,
    pub type_name: String,
    // pub fn_arg_doc: Vec<String>,
}

impl FnArgInfo {
    pub fn new(arg: syn::FnArg) -> Option<FnArgInfo> {
        // 过滤掉 self 参数（仅处理普通参数）
        if let FnArg::Typed(pat_type) = arg {
            // 提取参数名称（模式中的标识符）
            let arg_name = {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    &pat_ident.ident
                } else {
                    // 处理非标识符参数（如元组、结构体等，这里简化处理）
                    &Ident::new("unknown_arg", proc_macro2::Span::call_site())
                }
            };

            // 提取参数类型
            let arg_type = &pat_type.ty;

            // let doc = {
            //     pat_type
            //         .attrs
            //         .iter()
            //         .filter_map(|attr| {
            //             if attr.path().is_ident("about") {
            //                 // 解析 #[about = "..."]
            //                 if let syn::Meta::NameValue(meta_nv) = &attr.meta {
            //                     if let syn::Expr::Lit(expr_lit) = &meta_nv.value {
            //                         if let syn::Lit::Str(litstr) = &expr_lit.lit {
            //                             return Some(litstr.value());
            //                         }
            //                     }
            //                 }
            //             }

            //             None
            //         })
            //         .collect::<Vec<String>>()
            // };

            return Some(FnArgInfo {
                arg_name: arg_name.to_string() ,
                type_name: quote!(#arg_type)
                    .to_string()
                    .trim_matches(|x| x == ' ')
                    .replace(" ", "")
                    .to_string(),
                // fn_arg_doc: doc,
            });
        }

        return None;
    }

    /// 生成 cmd_arg_str -> rust_typed_value.
    ///          String -> T.
    /// returns.0 -> auto_name
    /// returns.1 -> the let exprs.
    /// returns.0 == "__arg_0___converted";
    /// returns.1 ==
    ///      "
    ///      let __arg_0__ = app._user_inputed_cmd_args.get(0).unwrap().clone();
    ///      let __arg_0___converted: String =
    ///          <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str(&__arg_0__)
    ///              .unwrap()
    ///              .clone();
    ///      ";
    pub fn gen_type_converter_code(&self, index: usize) -> (String, String) {
        if self.is_colection_type() {
            self.gen_vec_type_convet(index)
        } else {
            self.gen_single_type(index)
        }
    }

    fn auto_name(&self, index: usize) -> String {
        // format!("arg_{}_{}", index, self.type_name.to_lowercase())
        format!("__arg_{}", index,)
    }

    pub fn is_colection_type(&self) -> bool {
        let collection_types = ["Vec", "HashSet", "BTreeSet", "VecDeque"];

        let type_name: &str = &self.type_name.replace(" ", "");

        for x in collection_types {
            if type_name.starts_with(x) {
                return true;
            }
        }

        return false;
    }

    fn gen_single_type(&self, index: usize) -> (String, String) {
        // __arg_0__
        let arg_str_variable_name = self.auto_name(index);

        // __arg_0___converted
        let _converted_variable_name = arg_str_variable_name.clone() + "_converted";

        // {
        //     let ty = &self.type_name;
        //     let converted = format!(
        //         r###"
        // let {_converted_variable_name}: {ty} = {{
        //     let {arg_str_variable_name} = app._user_inputed_cmd_args.get({index}).unwrap().clone();

        //     <{ty} as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str(&{arg_str_variable_name})
        //         .unwrap()
        //         .clone()
        // }};

        // "###
        //     );

        //     return (_converted_variable_name.clone(), converted);
        // }
        {
            // let expect_msg = format!("<{}>", self.arg_name).cyan().bold().to_string();
            // let expect_msg = format!("<{}>", self.arg_name);
            let ty = &self.type_name;
            //     let converted = format!(
            //         r###"

            //     let {_converted_variable_name}: {ty} = {{
            //         let {arg_str_variable_name}: &String = app._user_inputed_cmd_args.get({index}).expect( "需要参数：{expect_msg}" );
            //         let value: {ty} = aslip::single_type_converter::<{ty}>({arg_str_variable_name});
            //         value
            //     }};
            // "###
            //     );

            let converted = format!(
                r###"
 
            let {_converted_variable_name}: {ty} = aslip::single_type_converter::<{ty}>(&app, "{arg_name}", {index});
        "###,
                arg_name = self.arg_name,
            );

            return (_converted_variable_name.clone(), converted);
        }
    }

    fn gen_vec_type_convet(&self, index: usize) -> (String, String) {
        // __arg_0__
        let arg_str_variable_name = self.auto_name(index);

        // __arg_0___converted
        let _converted_variable_name = arg_str_variable_name.clone() + "_converted";

        let final_code: String = {
            let ty = self.type_name.replace(" ", "");
            let inner_ty = get_inner_ty(&ty);

            let a = format!("[{}...]", self.arg_name);
            let expect_msg = format!("需要参数： {}", a);

            format!(
                r###"
            let {_converted_variable_name}: {ty} = {{
                let tail_args: &[String] = app._user_inputed_cmd_args.get({index}..).expect( "{expect_msg}");
                let re: {ty} = aslip::vec_type_converter::<{inner_ty}>(tail_args);

                re
            }};
                "###
            )
        };

        return (_converted_variable_name, final_code);
    }
}

/// "Vec<u8>" -> "u8"
/// 注意： 这港剧说并不支持  Vec<Option<u8>> 这样的类型嵌套，只支持 Vec<u8> 这样的， Result<O,E> 这种也是不支持的。
pub fn get_inner_ty(s: &str) -> String {
    let mut re = String::new();

    let mut inner_started = false;
    'outer: for x in s.chars() {
        match x {
            '<' => {
                if inner_started {
                    break 'outer;
                }

                inner_started = true
            }
            '>' => break 'outer,
            a => {
                if inner_started {
                    re.push(a);
                }
            }
        }
    }

    return re;
}
