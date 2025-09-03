use aslip::command;
use aslip::types::NumberInRange;


// use aslip::*;

#[command]
/// 测试没有参数的命令。
pub fn no_arg_action() {
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
pub fn one_arg(path: String) {
    println!(
        "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
        path
    );
}

#[command]
/// 一句话说明
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
    println!("your input is: {:?} {:?}", first, last);
}

#[command(name = "renamed", short = "c", about="这是覆盖后的 about。")]
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
