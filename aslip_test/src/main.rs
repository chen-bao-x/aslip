// # aslip_test

use std::{collections::HashMap, path::Path};

use aslip::command;

use aslip::*;
mod asd;
use asd::*;

fn main() {
    aslip::run!("");
    let sadf = <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str("");

    // hand_write();
}

fn hand_write() {
    let app = ::aslip::app::App::new("");

    let Some(cmd_name) = &app._user_inputed_cmd_name else {
        app.print_app_help();
        return;
    };

    match cmd_name.as_str() {
        "" => {
            panic!("命令的名称不能时 空字符串 \"\"");
        }
        "no_arg_action" => {
            crate::no_arg_action();
        }
        _ => {
            println!("error: no such command: {}", cmd_name);
        }
    };
}
