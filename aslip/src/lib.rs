///! # aslip
///! aslip::types:: 中的每个类型都可以作为 `#[command]` 标记的函数的参数。
mod custom_types;

/// aslip 用到的一些 crate。
pub mod tools {
    pub use color_print; // https://crates.io/crates/color-print
    pub use owo_colors; // https://crates.io/crates/owo-colors
    pub use prettytable; // https://crates.io/crates/prettytable
}

pub mod app;
/// 如果希望自定义的类型也口译用作 `#[command]` 标记的函数的参数，实现 FromArgStr trait 就行。
pub mod from_arg_sttr;
/// ### 这里的类型都可以用作 `#[command]` 标记的函数的参数.  
/// ### 这里面的所有类型都实现了 `FromArgStr` trait.
pub mod types;

pub use app::App;

pub use ::aslip_macro::command;

pub use ::aslip_macro::run;
pub use custom_types::Have;
pub use from_arg_sttr::FromArgStr;
// pub use types::*;
