// # aslip_test

mod cmds;
use cmds::*; // TODO: 让用户可以不输入 use cmds::* 也能正确访问到正确的函数。

fn main() {
    let app = aslip::app::App::new()
        .app_name("app name")
        .description("description");

    aslip::run!(app);

    // hand_write();
}

fn hand_write() {
    let app = ::aslip::app::App::new();

    let Some(cmd_name) = &app._user_inputed_cmd_name else {
        app.print_app_help();
        return;
    };

    match cmd_name.as_str() {
        "" => {
            println!("命令的名称不能时 空字符串 \"\"");
        }

        _ => {
            println!("error: no such command: {}", cmd_name);
        }
    };
}
