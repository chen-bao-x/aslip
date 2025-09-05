///! aslip_macro
///! aslip_macro
///! aslip_macro
///! aslip_macro

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

/// `#[command]` 用于申明并实现一个`命令`.
///
/// # Example:
/// ---
/// ```rust,ignore
/// // cmds.rs
/// use aslip::command;
///
/// /// 函数的文档注释的第一行会用作这个命令的一语句话说明，会显示在 cli 默认打印的 help 信息里。
/// /// 函数的完整文档会用作这个命令的 quick help，在 `app no_arg_action -h` 时显示。
/// /// Usage: app no_arg_action
/// #[command]
/// pub fn no_arg_action() {
///     println!("I'm no_arg_action, happy to see you.");
/// }
///
/// /// 只有一个一个参数 的命令。
/// /// Usage: app one_arg <PATH>
/// /// 
/// /// <green!>Args</>: 
/// ///     <PATH>:     the file path.
/// /// Options:
/// ///     -h          print this message.
/// #[command]
/// pub fn one_arg(path: String) {
///     println!(
///         "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
///         path
///     );
/// }
///
/// /// Vec<T> 类型只能作为最后一个参数。
/// #[command]
/// pub fn collection_arg(first: u8, last: Vec<u8>) {
///     println!("your input is: {:?} {:?}", first, last);
/// }
/// 
/// #[command] 也可以通过 macro args 来做一些设置。
/// #[command] macro 已经支持的参数有：
/// 
///    name        命令的实际命令名称。 如果没有设置，则默认使用 函数的名称 作为 命令名称。
///    short       命令的短名称，通常是命令名称的第一个字母。
/// 
/// #[command(name = "true_name", short = "s")]
/// fn fake_name(){
/// }
/// ```
///
///
/// 
/// 
/// ---
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

/// ```rust
/// fn run()
/// fn run(app: aslip::App)
/// ```
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
///     let mut app = aslip::app::App::new().about("description");
///
///     aslip::run!(app);
///  
/// }
/// ```
#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    crate::run_impl::run_impl(input)
}
