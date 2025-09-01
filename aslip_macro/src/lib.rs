// lib.rs

use std::{collections::HashMap, process::Command, sync::Mutex};

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

    TokenStream::from(quote!(#func))
}

static HAHA: &str = "1234";
#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    // 把 TokenStream 转成字符串（可解析成 AST）
    let input = parse_macro_input!(input as syn::LitStr);
    let name = input.value();

    // 生成新的代码
    let expanded = quote! {
      ::aslip::app::App::new("").run();
    };

    TokenStream::from(expanded)
}

// let mut m = COMMANDS.lock().unwrap();
// let sadf = m.insert(String::new(), info);
// m.get(&String::new());
static COMMANDS: once_cell::sync::Lazy<Mutex<HashMap<String, FnInfo>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
