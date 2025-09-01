# aslip

## 重要提示：
不要在 main.rs 中使用 `#[command]`.
rust 编译器再宏展开时是有先后顺序的，在其他文件中使用 `#[command]` 能确保 `#[command]` 在 `aslip::run!()` 之前展开.

#[command]
fn f<T>(v:T) {}
这里的类型 T 必须要实现 `::core::str::FromStr` trait。
