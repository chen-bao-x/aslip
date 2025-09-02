///! # aslip
pub use aslip_macro;
pub use aslip_macro::command;
pub use aslip_macro::run;

pub mod app;
pub mod from_arg_sttr;
pub mod types;

// pub use from_arg_sttr::*;

// use crate::app::App;

// pub fn run() {
//     let asdf = String::from_str("s");
//     let asdf = u8::from_str("4");
//     let asdf = <String as ::core::str::FromStr>::from_str("4");
//     let asdf = <String as ::core::str::FromStr>::from_str("4");
//     let asdf = <u8 as ::core::str::FromStr>::from_str("4");
// }

// fn type_convert(ty_name: &str, cmd_arg_str: &str) -> String {
//     format!(
//         "<{} as ::core::str::FromStr>::from_str(\"{}\")",
//         ty_name, cmd_arg_str
//     )
// }

// #[test]
// fn adfs() {
//     println!("{}", type_convert("String", "123"));
// }

// fn asdf<T: ::core::str::FromStr>() {
//     let a = T::from_str("s");
//     // a.map_err(|e|{
//     //     format!("{}",e);
//     // });

//     const ADSF: std::ops::Range<i32> = 1..9;
//     std::ops::Range::<i32>::from(2..9);
// }
