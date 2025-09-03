// # aslip_test

mod asd;
use asd::*;
use aslip::command;

#[command]
fn new(name: String) {
    println!("your input is: {}", name);
}

fn main() {
    aslip::run!("");

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

        _ => {
            println!("error: no such command: {}", cmd_name);
        }
    };
}
