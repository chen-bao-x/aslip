use quote::quote;
use syn::ItemFn;

use crate::FnArgInfo;

extern crate proc_macro;

// 实例化插件注册表

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
                let f = FnArgInfo::new(arg.clone()).expect(concat!(file!(), line!()));
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

    /// 获取函数的文档注释。
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

    // "one_arg" => {
    //      ...
    // },
    pub fn gen_case_code(&self) -> String {
        // 实参。
        let mut variables: Vec<String> = vec![];

        let mut codes: Vec<String> = vec![];
        {
            for index in 0..self.func_args.len() {
                let arg = self.func_args.get(index).expect(concat!(file!(), line!()));

                // re.0 == "__arg_0___converted";
                // re.1 ==
                //      "
                //      let __arg_0__ = app._user_inputed_cmd_args.get(0).unwrap().clone();
                //      let __arg_0___converted: String =
                //          <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str(&__arg_0__)
                //              .unwrap()
                //              .clone();
                //      ";
                let re = arg.gen_type_converter_code(index);

                variables.push(re.0);
                codes.push(re.1);
            }
        }

        format!(
            r###"
    "{func_name}" => 
            {{
        {args_geter}

        {func_name}({args});
        }}  ,
        "###,
            func_name = self.func_name,
            args_geter = codes.join("\n"),
            args = variables.join(","),
        )
    }

    /// 生成 trait bound check 代码, 大概长这样。
    /// const _: () = {
    ///     ::aslip::from_arg_sttr::from_arg_str_trait_bound_check::<Haha>();
    /// };
    pub fn gen_trait_bound_check(&self) -> String {
        if self.func_args.is_empty() {
            return String::new();
        }

        let mut each_check: String = String::new();
        {
            for a in &self.func_args {
                if a.is_colection_type() {
                    continue;
                }

                each_check.push_str(&gen_check_one(&a.type_name));
                each_check.push_str("\n");
            }
        }

        let trait_bound_check_code = format!(
            r###"

    // trait bound check.
    const _: () = {{
        {each_check}
    }};
    "###,
            each_check = each_check
        );

        fn gen_check_one(type_name: &str) -> String {
            format!(
                "        ::aslip::from_arg_sttr::from_arg_str_trait_bound_check::<{}>();",
                type_name
            )
        }

        return trait_bound_check_code;
    }
}
