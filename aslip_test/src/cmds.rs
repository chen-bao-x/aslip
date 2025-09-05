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
    println!("a2 I'm a2, happy to see you.");
}

#[command]
pub fn a3452() {
    println!("I'm a3452, happy to see you.");
}

/// 只有一个参数 的命令。
///
/// <green,bold>Usage</>: <cyan,bold>app</> <cyan,bold>one_arg</> <green,bold><<PATH>></>
///
/// <green!>Args</>:
///     <<PATH>>:     the file path.
/// Options:
///     -h          print this message.
#[command]
pub fn one_arg(path: usize) {
    println!(
        "I'm one_arg(path: String), happy to see you.\n your input is: {:?}",
        path
    );
}

#[command]
/// 一句话说明。
pub fn two_arg(a: NumberInRange<0, 88>, b: String) {
    println!(
        "I'm two_arg(a: NumberInRange<0, 88>, b: String), happy to see you.\n your input is: {:?} {:?}",
        a, b,
    );
}

#[command()]
/// arg 9 about.
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

#[command(name = "renamed", short = "c")]
/// 一句话说明。
pub fn test_for_macro_args() {}

/// 函数的文档注释的第一句话会用作 命令 的 <bold>一句话说明</bold> 打印到 cli 程序的帮助信息中。
/// 整个帮助文档会被作为这个 命令 的 quick help 信息 ———— `app cmd -h` 时显示的信息。
/// 还支持一些彩色标记语法哦： <red>red</red>  <bg:green>asdfadsf</>
///
#[command]
pub fn cmd_doc_example() {}

#[command(name = "-h")]
pub fn asdfadsfcmd_doc_example() {
    println!("this is my own help command.")
}

// #[command]
// pub fn option_arg(o: Option<String>) {
//     println!("a2 I'm a2, happy to see you.");
// }

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
// pub fn no_arg_adction() {
//     println!("{}", module_path!());
//     println!("I'm no_arg_action, happy to see you.");
// }
