use std::net::IpAddr;

///! aslip::types:: 中的每个类型都可以作为 `#[command` 标记的函数的参数。

///! # aslip
pub use aslip_macro;
pub use aslip_macro::command;
pub use aslip_macro::run;

use crate::from_arg_sttr::FromArgStr;

pub mod app;
pub mod from_arg_sttr;
pub mod types;

// TODO: 记得打开这三个选项。
// pub use from_arg_sttr::*;
// pub use types::*;
// pub use app::*;

// maybe never !
pub fn tail_colection_type_convert<T: from_arg_sttr::FromArgStr>(args: &[String]) -> Vec<T> {
    use owo_colors::OwoColorize;
    use std::process::exit;
    use std::thread::sleep;
    use std::time::Duration;

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
                sleep(Duration::new(1, 0));
                exit(1);
            }
        }
    }

    return re;
}

#[test]
fn sadfadsf() {
    let sadf: Vec<IpAddr> = tail_colection_type_convert(&[
        format!("1"),
        format!("2"),
        format!("3"),
        format!("4"),
        format!("5"),
        format!("6"),
    ]);
}
