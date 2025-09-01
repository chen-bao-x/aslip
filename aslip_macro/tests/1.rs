use ascp_macro::command;
use ascp_macro::run;

fn main() {}

/// 能看到文档么？
/// asdfadsf
/// sadf
/// ads
/// f
/// adsf
/// sad
#[command]
fn hello_func(
    a: [u8; 3],
    adsf: &str, // adsfdasf
) {
    run!("Alice"); // 生成一个 fn generated()
}
