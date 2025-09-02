// # aslip_test

mod asd;
use asd::*;

fn main() {
    aslip::run!("");

    hand_write();
}

#[test]
fn sadfadsf() {
    use std::net::IpAddr;
    let _sadf: Vec<IpAddr> = ::aslip::vec_type_converter(&[
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

        "collection_arg" => {
            let arg_0 = app._user_inputed_cmd_args.get(0).unwrap();
            let converted_0: u8 =
                <u8 as ::aslip::from_arg_sttr::FromArgStr>::from_arg_str(&arg_0).unwrap();

            // re.0 == final_value
            // re.1 ==
            let final_value: Vec<u8> = {
                let tail_args = app._user_inputed_cmd_args.get(1..).unwrap();
                let re = aslip::vec_type_converter::<u8>(tail_args);

                re
            };

            crate::collection_arg(converted_0, final_value);
        }
        _ => {
            println!("error: no such command: {}", cmd_name);
        }
    };
}
