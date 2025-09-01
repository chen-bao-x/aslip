// aslip_macro

use std::{collections::HashMap, process::Command, sync::Mutex, thread::sleep, time::Duration};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, FnArg, Ident, ItemFn, Lit, Pat, Type, meta::ParseNestedMeta, parse_macro_input,
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

    let info = FnInfo::new(&func);
    // panic!("asdfasdfdsaf {:?}", info);

    // 注册命令到全局表
    ::once_cell::sync::Lazy::force(&COMMANDS);
    adadsf(info);
    TokenStream::from(quote!(        #func))
}

fn adadsf(info: FnInfo) {
    for x in 0..100 {
        let re = COMMANDS.lock();

        match re {
            Ok(mut m) => {
                let sadf = m.insert(info.func_name.clone(), info.clone());
                return;
            }
            Err(_) => sleep(Duration::new(0, 1_000_000_000 / 10)),
        }
    }
}

// let mut m = COMMANDS.lock().unwrap();
// let sadf = m.insert(String::new(), info);
// m.get(&String::new());
static COMMANDS: once_cell::sync::Lazy<Mutex<HashMap<String, FnInfo>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

// #[proc_macro]
// pub fn run(input: TokenStream) -> TokenStream {
//     // 把 TokenStream 转成字符串（可解析成 AST）
//     let input = parse_macro_input!(input as syn::LitStr);

//     let m = COMMANDS.lock().unwrap();

//     // 把字符串解析成 Arm（match 分支）
//     let arms: &Vec<syn::Arm> = &m
//         .values()
//         .map(|x| x.gen_case_code())
//         .into_iter()
//         .map(|s| syn::parse_str::<syn::Arm>(&s).unwrap())
//         .collect();

//     // 生成新的代码
//     let expanded_tokens = quote! {

//     let app = ::aslip::app::App::new(#input);

//     let Some(cmd_name) = &app._user_inputed_cmd_name else {
//         app.print_app_help();
//         return;
//     };

//     match cmd_name.as_str() {
//         "" => {
//             panic!("命令的名称不能时 空字符串 \"\"");
//         },
//          #(#arms),*
//         _ => {

//             println!("error: no such command: {}", cmd_name);
//         }
//     };

//     };

//     TokenStream::from(expanded_tokens)
// }

#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    // 把 TokenStream 转成字符串（可解析成 AST）
    let input = parse_macro_input!(input as syn::LitStr);

    let m = COMMANDS.lock().unwrap();

    // 把字符串解析成 Arm（match 分支）
    // let arms: &Vec<syn::Arm> = &m
    //     .values()
    //     .map(|x| x.gen_case_code())
    //     .into_iter()
    //     // .map(|s| syn::parse_str::<syn::Arm>(&s).unwrap())
    //     .map(|s| syn::parse_str::<syn::Arm>(&s).unwrap())
    //     .collect();

    let mut arms: Vec<syn::Arm> = vec![];
    for (_key, b) in m.iter() {
        let code = b.gen_case_code();

        let a = syn::parse_str::<syn::Arm>(&code).unwrap();
        arms.push(a);
    }

    // panic!("m.len() {}", m.len());
    // panic!("m.iter().len() {}", m.iter().len());

    // 生成新的代码
    let expanded_tokens = quote! {
    let adsfasdfadsfadsf = || {

        let app = ::aslip::app::App::new(#input);

        let Some(cmd_name) = &app._user_inputed_cmd_name else {
            app.print_app_help();
            return;
        };


        match cmd_name.as_str() {
            "" => {
                panic!("命令的名称不能时 空字符串 \"\"");
            },


             #(#arms)*


            _ => {

                println!("error: no such command: {:?}", cmd_name);
            }
        };

    };

        adsfasdfadsfadsf();

        };

    TokenStream::from(expanded_tokens)
}
