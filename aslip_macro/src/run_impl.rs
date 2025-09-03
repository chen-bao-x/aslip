// aslip_macro

use crate::data;
use proc_macro::TokenStream;
use quote::quote;
extern crate proc_macro;

pub fn run_impl(input: TokenStream) -> TokenStream {
    let arms: Vec<syn::Arm> = {
        let store = data::COMMANDS.lock().expect(concat!(file!(), line!()));

        let mut re: Vec<syn::Arm> = vec![];

        for (_key, fn_info) in store.iter() {
            let code = fn_info.gen_case_code();

            let a = syn::parse_str::<syn::Arm>(&code).expect(&(code + concat!(file!(), line!())));
            re.push(a);
        }

        re
    };

    let app = gen_app_var(input);

    // 生成新的代码
    let expanded_tokens = quote! {


        {
             use aslip::types::*;
             use ::aslip::from_arg_sttr::FromArgStr;

             #app

             let Some(cmd_name) = &app._user_inputed_cmd_name else {
                 app.print_app_help();
                 return;
             };


             match cmd_name.as_str() {
                 "" => panic!("命令的名称不能时 空字符串 \"\"") ,

    // TODO: 添加 help version 这两个命令的默认实现。
                  #(#arms)*


                 _ => println!("error: no such command: {:?}", cmd_name),

             };

         };



         };

    TokenStream::from(expanded_tokens)
}

fn gen_app_var(input: TokenStream) -> proc_macro2::TokenStream {
    let lit: syn::Result<syn::Ident> = syn::parse(input.clone());

    // let sadf = gen_push_to_app_command_list();

    let store = data::COMMANDS.lock().expect(concat!(file!(), line!()));

    let mut re: proc_macro2::TokenStream = quote! {};

    for (_key, fn_info) in store.iter() {
        let code = fn_info.gen_push_to_app_command_list();

        re = quote! {
            #re
            #code

        };
    }

    return match lit {
        Ok(variable_ident) => {
            // 用户自己定义了一个 aslip::app::App, 则使用用户定义的 aslip::app::App.
            quote! {

                let mut app = #variable_ident;
                #re
              }
        }
        Err(_) => {
            // 用户没有传入，则使用默认实现。
            quote! {
                let mut app = ::aslip::app::App::new();

                #re
            }
        }
    };
}
