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


             #app

             match &app._user_inputed_cmd_name {
                None => {
                     app.print_app_help();
                }
                Some(cmd_name)  => {

                        // app cmd -h
                        // app cmd --help
                        if cmd_name == "-h" || cmd_name == "--help" {
                            app.print_cmd_quick_help_for(cmd_name);

                        } else {

                            match cmd_name.as_str() {
                                "" => panic!("命令的名称不能时 空字符串 \"\"") ,

                                // TODO: 添加 help version 这两个命令的默认实现。
                                #(#arms)*


                                x => {
                                    match x {
                                        "-h" | "--help" | "help" => {
                                            app.print_app_help();
                                        }
                                        "-v" | "--version" | "verstion" => {
                                            println!("app verions: \"0.1.2\"")
                                        }

                                        v => {
                                            ::aslip::tools::color_print::cprintln!("<red,bold>error:</> no such command: <cyan>{:?}</>", v);

                                        }
                                    }



                                },

                            };
                        }

                }
             }


        };

    };

    TokenStream::from(expanded_tokens)
}

fn gen_app_var(input: TokenStream) -> proc_macro2::TokenStream {
    let lit: syn::Result<syn::Ident> = syn::parse(input.clone());

    // let sadf = gen_push_to_app_command_list();

    let store = data::COMMANDS.lock().expect(concat!(file!(), line!()));

    let mut app_cmds_init: proc_macro2::TokenStream = quote! {};

    for (_key, fn_info) in store.iter() {
        let code = fn_info.gen_push_to_app_command_list();

        app_cmds_init = quote! {
            #app_cmds_init
            #code

        };
    }

    return match lit {
        Ok(variable_ident) => {
            // 用户自己定义了一个 aslip::app::App, 则使用用户定义的 aslip::app::App.
            quote! {

            //   let mut app = #variable_ident;
                 let app: &mut App = &mut #variable_ident;
                #app_cmds_init


            }
        }
        Err(_) => {
            // 用户没有传入，则使用默认实现。
            quote! {
                let mut app = ::aslip::app::App::new();

                #app_cmds_init


            }
        }
    };
}
