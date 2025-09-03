// aslip_

use std::fmt::Debug;
use syn::Ident;
use syn::LitStr;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

extern crate proc_macro;

/// 代表一个键值对参数： a = "xxx"
#[derive(Clone)]
pub struct AttibuteArg {
    pub key: String,
    pub _eq: String,
    pub value: String,
}

impl AttibuteArg {
    pub fn suprted_arg_check(key_ident: &proc_macro2::Ident) -> syn::Result<()> {
        const SUPORTED_ARGS: &[&str] = &[
            "about",      // 命令的一句话说明。
            "quick_help", // app cmd -h 时显示的信息。
            "doc",        // 该命令的详细文档。 app help cmd 时显示的详细文档。
            "short",      // command short name.
        ];

        let key = format!("{}", key_ident);

        let err_msg: String = format!(
            "
不支持的参数：{key}

已经支持的参数有：
    about          命令的一句话说明。
    quick_help     app cmd -h 时显示的信息。
    doc            该命令的详细文档。 app help cmd 时显示的详细文档。
    short          命令的短名称，例如：`cargo b`

for more infomation: https://google.com
"
        );
        // TODO: 修改为真实的网址。

        if SUPORTED_ARGS.contains(&key.as_ref()) {
            return Ok(());
        } else {
            return Err(syn::Error::new(key_ident.span(), err_msg));
        }
    }
}

impl Parse for AttibuteArg {
    /// key = "some text."
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?; // key_name
        AttibuteArg::suprted_arg_check(&key)?;

        let _eq: Token![=] = input.parse()?; // =

        // let value: Lit = input.parse()?;
        let value: LitStr = input.parse()?; // ""

        Ok(AttibuteArg {
            key: format!("{}", key),
            _eq: "=".to_string(),
            value: value.value(),
        })
    }
}

/// 支持多个参数, 用逗号分隔
#[derive(Clone)]
pub struct AttibuteArgList {
    pub args: Vec<AttibuteArg>,
}

impl Parse for AttibuteArgList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.parse_terminated(AttibuteArg::parse, Token![,])?;
        Ok(AttibuteArgList {
            args: args.into_iter().collect(),
        })
    }
}
