// aslip_macro

use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;

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

    insert(info);

    TokenStream::from(quote!( #func))
}

fn insert(info: FnInfo) {
    let re = COMMANDS.lock();

    match re {
        Ok(mut m) => {
            let _ = m.insert(info.func_name.clone(), info.clone());
            return;
        }
        Err(_) => sleep(Duration::new(0, 1_000_000_000 / 10)),
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

        // let s = lit.clone().unwrap();
        // if s.value().is_empty() {
        //     return syn::Error::new(s.span(), "命令的名称不能为空 \"\"")
        //         .to_compile_error()
        //         .into();
        // }
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

    let A01K43QTAA353DAH630HBJRGTSY = || {
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

    A01K43QTAA353DAH630HBJRGTSY();

    };

    TokenStream::from(expanded_tokens)
}
