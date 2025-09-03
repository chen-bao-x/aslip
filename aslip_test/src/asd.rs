use std::str::FromStr;

use aslip::{aslip_macro::command_2, command, types::NumberInRange};

// use aslip::*;

/// 测试没有参数的命令。
#[command]
pub fn no_arg_action() {
    println!("{}", module_path!());
    println!("I'm no_arg_action, happy to see you.");
}

#[command]
pub fn a2() {
    println!("a2 I'm no_arg_action, happy to see you.");
}

#[command]
pub fn a3452() {
    println!("a3452  I'm no_arg_action, happy to see you.");
}

#[command]
pub fn one_arg(s: String) {
    _ = String::from_str("s");
    // ::aslip::FromArgStr::from_arg_str(s)

    let _a = <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str("");

    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
        s
    );
}

#[command]
pub fn two_arg(a: NumberInRange<0, 88>, b: String) {
    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?} {:?}",
        a, b,
    );
}

#[command()]
pub fn arg_9(a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8, a7: u8, a8: u8, a9: Vec<u8>) {
    println!(
        "arg_9: happy to see you.\n your input is: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        a1, a2, a3, a4, a5, a6, a7, a8, a9,
    );
}

#[command]
pub fn collection_arg(first: u8, last: Vec<u8>) {
    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?} {:?}",
        first, last
    );
}

/// this is the one line descript?
#[command_2(
    about = "this is the one line descript?",
    // asdfadsf = "",
    // dsafadsfasdfdsaf = ""
)]
pub fn collectasdfion_arg(first: u8, last: Vec<u8>) {
    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?} {:?}",
        first, last
    );
}

use aslip::aslip_macro::command_3;

#[command_3("hello")]
pub fn colasdflection_arg() {}

#[command(a = "asdfasdf", b = "adfasdfsadf")]
/// 一句话说明。
pub fn test_for_macro_args() {}

// # error case: 这几个测试使用来测试 实现的 aslip_macro 是否能正常报错。

// /// 测试规则：command 函数不能有返回值。
// #[command]
// pub fn no_returns_rule_test() -> u8 {
//     return 3;
// }

// /// 测试规则：只有最后一个参数可以使用 Vec<T> 类型。
// #[command]
// pub fn only_last_arg_can_vec_rule_test(a: Vec<String>, a: u8) {}

// /// 测试 rule_3：
// #[command]
// pub fn no_arg_action() {
//     println!("{}", module_path!());
//     println!("I'm no_arg_action, happy to see you.");
// }
