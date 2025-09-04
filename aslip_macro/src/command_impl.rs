use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;
use syn::spanned::Spanned;

use crate::FnInfo;

extern crate proc_macro;

pub fn command_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入的函数
    let func = parse_macro_input!(input as ItemFn);

    {
        // error check。
        match crate::rules::rule_check(_args.clone(), &func) {
            Ok(_) => {}
            Err(e) => return e.to_compile_error().into(),
        };
    }

    let fn_info = match FnInfo::new(_args, &func) {
        Ok(info) => info,
        Err(e) => return e.into_compile_error().into(),
    };

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
