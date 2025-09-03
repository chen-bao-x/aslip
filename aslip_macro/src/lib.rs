// aslip_macro

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

/// # Exampel:
/// ```rust,ignore
///
/// // cmds.rs
/// use aslip::command;
///
/// #[command]
/// pub fn no_arg_action() {
///     println!("I'm no_arg_action, happy to see you.");
/// }
///
/// #[command]
/// pub fn one_arg(path: String) {
///     println!(
///         "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
///         path
///     );
/// }
///
/// #[command]
/// /// Vec<T> 类型只能作为最后一个参数。
/// pub fn collection_arg(first: u8, last: Vec<u8>) {
///     println!("your input is: {:?} {:?}", first, last);
/// }
/// ```
///
///
/// ```rust,ignore
/// // main.rs
/// use aslip;
/// fn main() {
///     let app = aslip::app::App::new().about("description");
///
///     aslip::run!(app);
///  
/// }
/// ```
#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    crate::command_impl::command_impl(_args, input)
}

/// # Exampel:
/// ```rust,ignore
///
/// // cmds.rs
/// use aslip::command;
///
/// #[command]
/// pub fn no_arg_action() {
///     println!("I'm no_arg_action, happy to see you.");
/// }
///
/// #[command]
/// pub fn one_arg(path: String) {
///     println!(
///         "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
///         path
///     );
/// }
///
/// #[command]
/// /// Vec<T> 类型只能作为最后一个参数。
/// pub fn collection_arg(first: u8, last: Vec<u8>) {
///     println!("your input is: {:?} {:?}", first, last);
/// }
/// ```
///
///
/// ```rust,ignore
/// // main.rs
/// use aslip;
/// fn main() {
///     let app = aslip::app::App::new().about("description");
///
///     aslip::run!(app);
///  
/// }
/// ```
#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    crate::run_impl::run_impl(input)
}
