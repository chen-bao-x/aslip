// aslip_macro

use crate::data;
use proc_macro::TokenStream;
use quote::quote;
extern crate proc_macro;

pub fn run_impl(input: TokenStream) -> TokenStream {
    let arms: Vec<syn::Arm> = {
        let store = data::COMMANDS.lock().expect(concat!(file!(), ":", line!()));

        let mut re: Vec<syn::Arm> = vec![];

        for (_key, fn_info) in store.iter() {
            {
                let code = fn_info.gen_case_code();

                let a = syn::parse_str::<syn::Arm>(&code).expect(
                    &("解析 syn::Arm 时出错：".to_string() + concat!(":", file!(), ":", line!())),
                );
                re.push(a);
            }
            {
                let code = fn_info.gen_short_name_case_code();

                if !code.is_empty() {
                    let a = syn::parse_str::<syn::Arm>(&code).expect(
                        &("解析 syn::Arm 时出错：".to_string()
                            + &code
                            + concat!(":", file!(), ":", line!())),
                    );
                    re.push(a);
                }
            }
        }

        re
    };

    let app = gen_app_var(input);

    // 生成新的代码
    let expanded_tokens = quote! {


        'block:  {
            use aslip::types::*;
            use aslip::app::App;

            #app

            match &app._user_inputed_cmd_name {
                None => {
                     app.print_app_help();
                }
                Some(cmd_name)  => {

                        // 查看 命令 的 quick help。
                        // app cmd -h
                        // app cmd --help
                        if let Some(first_arg) = app._user_inputed_cmd_args.first() {
                            if first_arg == "-h" || first_arg == "--help" {
                                app.print_cmd_quick_help_for(cmd_name);
                                break 'block;
                            }
                        };

                            match cmd_name.as_str() {

                                "" =>    {app.print_app_help();},

                                #(#arms)*

                                x => {
                                    match x {

                                        "help" => {
                                            if let Some(first_arg) = app._user_inputed_cmd_args.first() {
                                                    app.print_cmd_quick_help_for(first_arg);
                                                    break 'block;
                                            }else {
                                                app.print_app_help();
                                            };

                                        }
                                        "-h" | "--help"  => {
                                            app.print_app_help();

                                        }
                                        "-v" | "--version" | "version" => {
                                             app.print_app_version();
                                        }

                                        v => {
                                            ::aslip::tools::color_print::cprintln!("<red,bold>error:</> no such command: <cyan>{:?}</>", v);

                                        }
                                    }

                                },

                            };


                    }
             }


        };

    };

    TokenStream::from(expanded_tokens)
}

/// &mut app
#[derive(Clone)]
struct BarrowMutIdent {
    app_iden: syn::Ident,
}
impl syn::parse::Parse for BarrowMutIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Token![&] = input.parse()?;
        let _: syn::Token![mut] = input.parse()?;
        let app_iden: syn::Ident = input.parse()?;
        Ok(Self { app_iden: app_iden })
    }
}

/// ()
#[derive(Clone)]
struct EmptyIden {}
impl syn::parse::Parse for EmptyIden {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input;

        Ok(Self {})
    }
}
fn gen_app_var(input: TokenStream) -> proc_macro2::TokenStream {
    let barrow_mut_ident_result: syn::Result<BarrowMutIdent> =
        syn::parse::<BarrowMutIdent>(input.clone());
    if let Err(ref _e) = barrow_mut_ident_result {
        // 有求 要么 &mut app, 要么 ()
        let _ = syn::parse::<EmptyIden>(input).expect("提示：&mut app");
    }

    let store = data::COMMANDS.lock().expect(concat!(file!(), line!()));

    let mut app_cmds_init: proc_macro2::TokenStream = quote! {};

    for (_key, fn_info) in store.iter() {
        let code = fn_info.gen_push_to_app_command_list();

        app_cmds_init = quote! {
            #app_cmds_init
            #code

        };
    }

    return match barrow_mut_ident_result {
        Ok(app_ident) => {
            let ident = app_ident.app_iden;
            // 用户自己定义了一个 aslip::app::App, 则使用用户定义的 aslip::app::App.
            quote! {
                let app: &mut App = &mut #ident;
                #app_cmds_init
            }
        }
        Err(_e) => {
            // 用户没有传入，则使用默认实现。
            quote! {
                let mut app = ::aslip::app::App::new();
                #app_cmds_init
            }
        }
    };
}
