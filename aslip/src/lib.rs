///! aslip::types:: 中的每个类型都可以作为 `#[command` 标记的函数的参数。

///! # aslip
pub use aslip_macro;
pub use aslip_macro::command;
pub use aslip_macro::run;

pub mod app;
pub mod from_arg_sttr;
pub mod types;

mod custom_types;

// TODO: 记得打开这三个选项。
// pub use from_arg_sttr::*;
// pub use types::*;
// pub use app::*;

// maybe never !
pub fn single_type_converter<T: from_arg_sttr::FromArgStr>(arg: &String) -> T {
    use owo_colors::OwoColorize;

    let asdf = <T as from_arg_sttr::FromArgStr>::from_arg_str(arg);
    match asdf {
        Ok(val) => return val,
        Err(e) => {
            eprintln!(
                "{}{}: 将 {:?} 转换为 {} 出错。",
                "error: ".red().bold(),
                std::any::type_name_of_val(&e).red(),
                arg.green(),
                std::any::type_name::<T>().cyan().bold()
            );

            std::process::exit(1);
        }
    };
}

// maybe never !
pub fn vec_type_converter<T: from_arg_sttr::FromArgStr>(args: &[String]) -> Vec<T> {
    use owo_colors::OwoColorize;
    use std::process::exit;

    let mut re: Vec<T> = vec![];
    for x in args {
        let sdaf = T::from_arg_str(x);
        match sdaf {
            Ok(val) => re.push(val),
            Err(e) => {
                eprintln!(
                    "{}{}: 将 {:?} 转换为 {} 出错。",
                    "error: ".red().bold(),
                    std::any::type_name_of_val(&e).red(),
                    x.green(),
                    std::any::type_name::<T>().cyan().bold()
                );

                exit(1);
            }
        }
    }

    return re;
}

#[test]
fn sadfadsf() {
    let _sadf: Vec<std::net::IpAddr> = vec_type_converter(&[
        format!("1"),
        format!("2"),
        format!("3"),
        format!("4"),
        format!("5"),
        format!("6"),
    ]);
}
