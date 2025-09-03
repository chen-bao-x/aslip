// aslip_macro

use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::ItemFn;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::{
    Ident, Lit, Token,
    parse::{Parse, ParseStream},
};

extern crate proc_macro;

mod fn_arg_info;
mod fn_info;
mod rules;

use fn_arg_info::*;
use fn_info::*;

#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入的函数
    let func = parse_macro_input!(input as ItemFn);

    {
        //  let re = crate::rules::rule_check(&func);
        if let Some(err) = crate::rules::rule_check(_args.clone(), &func) {
            return err.to_compile_error().into();
        }
    }

    let fn_info = FnInfo::new(_args, &func);

    {
        // 将 fn_info 存储到 COMMANDS 中， 留着给 aslip_macro::run!() 用。
        ::once_cell::sync::Lazy::force(&COMMANDS);
        let re = insert(fn_info.clone(), func.span());
        if let Err(e) = re {
            return e.into_compile_error().into();
        }
    }

    println!("fn_info: {:#?}", fn_info);

    let trait_bound_check_code = fn_info.gen_trait_bound_check();
    let result = syn::parse_str::<syn::ItemConst>(&trait_bound_check_code);
    match result {
        Ok(trait_bound_check_ast) => {
            return TokenStream::from(quote!(

                #trait_bound_check_ast
                #func

            ));
        }

        Err(_) => {
            // 没有需要除了的参数。直接返回原来的函数。
            return TokenStream::from(quote!(

                #func

            ));
        }
    }
}

fn insert(info: FnInfo, span: proc_macro2::Span) -> Result<(), syn::Error> {
    let re = COMMANDS.lock();

    match re {
        Ok(mut m) => {
            let old_cmd = m.insert(info.func_name.clone(), info.clone());
            if let Some(cmd) = old_cmd {
                return Err(rule_3(cmd, info, span));
                // panic!("命令的名称不能重复：{}", cmd.func_name);
            }

            return Ok(());
        }
        Err(e) => Err(syn::Error::new(span, e.to_string())),
    }
}

// let mut m = COMMANDS.lock().unwrap();
// let sadf = m.insert(String::new(), info);
// m.get(&String::new());
static COMMANDS: once_cell::sync::Lazy<Mutex<HashMap<String, FnInfo>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    let lit: Result<syn::LitStr, syn::Error> = syn::parse(input.clone());
    {
        // error handle.
        if let Err(err) = lit {
            // 把 syn::Error 转成编译错误
            return syn::Error::new(err.span(), "tips: aslip::run!(\"此程序的一句话说明.\");")
                .to_compile_error()
                .into();
        }
    }

    // 把 TokenStream 转成字符串（可解析成 AST）
    let app_descript_litstr = lit.unwrap();

    let store = COMMANDS.lock().expect(concat!(file!(), line!()));

    let arms: Vec<syn::Arm> = {
        let mut re: Vec<syn::Arm> = vec![];
        for (_key, fn_info) in store.iter() {
            let code = fn_info.gen_case_code();

            let a = syn::parse_str::<syn::Arm>(&code).expect(&(code + concat!(file!(), line!())));
            re.push(a);
        }
        re
    };

    // 生成新的代码
    let expanded_tokens = quote! {


    {
         use aslip::types::*;
         use ::aslip::from_arg_sttr::FromArgStr;

         let app = ::aslip::app::App::new(#app_descript_litstr);
         let Some(cmd_name) = &app._user_inputed_cmd_name else {
             app.print_app_help();
             return;
         };


         match cmd_name.as_str() {
             "" => panic!("命令的名称不能时 空字符串 \"\"") ,


              #(#arms)*


             _ => println!("error: no such command: {:?}", cmd_name),

         };

     };



     };

    TokenStream::from(expanded_tokens)
}

mod attribute_args;
use attribute_args::*;

use crate::rules::rule_3;
#[proc_macro_attribute]
pub fn command_2(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析参数
    let args = syn::parse_macro_input!(attr as AttibuteArgList);
    for arg in &args.args {
        println!("key = {}, value = {:?}", arg.key, arg.value,);
    }

    if args.args.is_empty() {
        println!("args is empty.")
    }

    // 原函数不变
    let item = proc_macro2::TokenStream::from(item);

    quote!(#item).into()
}

#[proc_macro_attribute]
pub fn command_3(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析参数
    // let args = syn::parse_macro_input!(attr as syn::LitStr);

    let re: syn::Result<syn::LitStr> = syn::parse(attr);
    match re {
        Ok(about) => {
            println!("inputed arg is: {:?}", about.value());
        }
        Err(e) => {
            println!("没有出入。",);
        }
    }

    // 原函数不变
    let item = proc_macro2::TokenStream::from(item);
    quote!(#item).into()
}
