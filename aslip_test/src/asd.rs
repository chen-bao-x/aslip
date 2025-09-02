use std::str::FromStr;

use aslip::{command, from_arg_sttr, types::NumberInRange};

// use aslip::*;
/// 测试没有参数的命令。
#[command]
pub fn no_arg_action() {
    println!("{}", module_path!());
    println!("I'm no_arg_action, happy to see you.");
}

/// 测试没有参数的命令。
#[command]
pub fn a2() {
    println!("a2 I'm no_arg_action, happy to see you.");
}

/// 测试没有参数的命令。
#[command]
pub fn a3452() {
    println!("a3452  I'm no_arg_action, happy to see you.");
}

/// 测试没有参数的命令。
#[command]
pub fn one_arg(s: String) {
    _ = String::from_str("s");
    // ::aslip::FromArgStr::from_arg_str(s)

    let a = <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str("");

    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
        s
    );
}

/// 测试没有参数的命令。
#[command]
pub fn two_arg(a: NumberInRange<0, 88>, b: String) {
    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?} {:?}",
        a, b,
    );
}

// /// 测试没有参数的命令。
// #[command]
// pub fn collection_arg(a: &[String]) {
//     println!(
//         "one_arg  I'm one_arg, happy to see you.\n your input is: {:?} ",
//         a,
//     );
// }
