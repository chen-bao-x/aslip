// # aslip_test

mod cmds;
use aslip::app::App;
use cmds::*; // TODO: 让用户可以不输入 use cmds::* 也能正确访问到正确的函数。

fn main() {
    let mut dsfdsaf = aslip::app::App::new().about("description").author("author");

    aslip::run!(dsfdsaf);
}

// fn hand_write() {
//     let mut dsfasdf = ::aslip::app::App::new();

//     // .push(::aslip::app::CmdInfo::new("name", "short_name", "about"));

//     let Some(cmd_name) = &dsfasdf._user_inputed_cmd_name else {
//         dsfasdf.print_app_help();
//         return;
//     };

//     // app cmd -h
//     // app cmd --help
//     if let Some(first_arg) = dsfasdf._user_inputed_cmd_args.first() {
//         if first_arg == "-h" || first_arg == "--help" {
//             dsfasdf.print_cmd_quick_help_for(cmd_name);
//             return;
//         }
//     };

//     match cmd_name.as_str() {
//         "" => {
            
      
//             println!("命令的名称不能时 空字符串 \"\"");
//         }

//         _ => {
//             println!("error: no such command: {}", cmd_name);
//         }
//     };
// }
