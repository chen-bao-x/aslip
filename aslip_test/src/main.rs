// # aslip_test

use std::fmt::Debug;
use std::net::IpAddr;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, path::Path};

use aslip::command;

use aslip::*;
mod asd;
use asd::*;
#[derive(Debug)]
struct Haha {}

fn main() {
    aslip::run!("");
    // let sadf = <String as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str("");
    // let sadf = <Box<String> as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str("hello");

    // hand_write();
}

// #[test]
fn sadfadsf() {
    let sadf: Vec<IpAddr> = ::aslip::tail_colection_type_convert(&[
        format!("1"),
        format!("2"),
        format!("3"),
        format!("4"),
        format!("5"),
        format!("6"),
    ]);
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

// fn adfadsf() {
//     // trait bound check.
//     const _: () = {
//         ::aslip::from_arg_sttr::from_arg_str_trait_bound_check::<Haha>();
//     };
// }


fn asddsaf(){

    let f = ||{};
}