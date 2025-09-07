// # aslip_test

mod aa;
mod cmds;
use aa::*;

/// 如果把 #[command] 标记的函数都存起来，
/// 再写到 match case 里面，就可以不用 use cmds::*; 了。
use cmds::ok_case::*;

#[aslip::command]
fn aasdfasdf2() {
    println!("ok")
}

fn main() {
    let mut app = aslip::app::App::new().about("description").author("author");

    aslip::run!(&mut app);
}
