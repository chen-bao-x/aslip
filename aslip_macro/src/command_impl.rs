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

use crate::FnInfo;

extern crate proc_macro;

pub fn command_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
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
        ::once_cell::sync::Lazy::force(&crate::data::COMMANDS);
        let re = crate::data::insert(fn_info.clone(), func.span());
        if let Err(e) = re {
            return e.into_compile_error().into();
        }
    }

    // println!("fn_info: {:#?}", fn_info);

    let trait_bound_check_code = fn_info.gen_trait_bound_check();
    let result = syn::parse_str::<syn::ItemConst>(&trait_bound_check_code);
    match result {
        Ok(trait_bound_check_ast) => {
            return TokenStream::from(quote!(

                #trait_bound_check_ast
                #func

                // -h               quick help
                // app help cmd     command document
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


fn adsfdasf_quick_help(){

}

fn adsfdasf_document(){
    
}