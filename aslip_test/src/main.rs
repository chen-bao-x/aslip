// # aslip_test

mod cmds;
use std::collections::HashSet;

use aslip::command;
use cmds::*; // TODO: 让用户可以不输入 use cmds::* 也能正确访问到正确的函数。

#[command]
/// create a new book.
fn new(name: String) {
    println!("create a new book.")
}

#[command]
/// create a new chapter.
fn new_chapter(name: Vec<String>) {
    println!("create a new chapter.")
}

fn main() {
    // let app = aslip::app::App::new().about("description").author("author");

    // aslip::run!(app);
    aslip::run!();

    // hand_write();
}

fn hand_write() {
    let mut app = ::aslip::app::App::new();

    app._commands
        .push(::aslip::app::CmdInfo::new("name", "short_name", "about"));

    let Some(cmd_name) = &app._user_inputed_cmd_name else {
        app.print_app_help();
        return;
    };

    match cmd_name.as_str() {
        "" => {
            let __arg_4___converted: u8 = aslip::single_type_converter::<u8>(&app, "a5", 2);
            app.print_app_help();
            println!("命令的名称不能时 空字符串 \"\"");
        }

        _ => {
            println!("error: no such command: {}", cmd_name);
        }
    };
}

fn ff<T>() {}
