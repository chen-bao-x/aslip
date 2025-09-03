// aslip_macro
use crate::rules::*;
use proc_macro::TokenStream;
extern crate proc_macro;
use fn_arg_info::*;
use fn_info::*;
mod attribute_args;
use attribute_args::*;

mod command_impl;
mod data;
mod fn_arg_info;
mod fn_info;
mod rules;
mod run_impl;

#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    crate::command_impl::command_impl(_args, input)
}

#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    crate::run_impl::run_impl(input)
}
