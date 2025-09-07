# **aslip**

> [!NOTE]
> Are you sure you want to claim this million?  
> With just a slip of your little hand,  
> you pressed the confirm button.

aslip 是一个简单的命令行参数解析工具，极其简单。

## 示例：

```rust
 // ## commands.rs

 use aslip::command;

 /// 没有参数的命令。
 #[command]
 pub fn no_arg_action() {
     println!("I'm no_arg_action, happy to see you.");
 }

 /// 只有一个参数的命令
 #[command]
 pub fn one_arg(path: String) {
     println!(
         "one_arg  I'm one_arg, happy to see you.\n your input is: {:?}",
         path
     );
 }

 /// Vec 类型只能作为最后一个参数。
 #[command]
 pub fn collection_arg(first: u8, last: Vec<u8>) {
     println!("your input is: {:?} {:?}", first, last);
 }
```

```rust
// ## main.rs
use aslip;
fn main() {
    let mut app = aslip::App::new()
        .app_name("aslip_example")
        .about("这是 aslip 示例 程序")
        .author("author: name    name@gmail.com")
        .version("0.1.0");

    aslip::run!(app);
}

```

之后再 `cargo run` 就能看到输出：

```sh

这是 aslip 示例 程序
author: name    name@gmail.com

Usage:
    aslip_example [commands]

Commands:
    no_arg_action       没有参数的命令。
    collection_arg      Vec 类型只能作为最后一个参数。
    one_arg             只有一个参数的命令


Use "aslip_example help <command>" for more information about a command.

```

> [!TIP]
> 1  
> rust 编译器在宏展开时是有先后顺序的，  
> 在其他文件中使用 `#[command]` 能确保 `#[command]` 在 `aslip::run!()` 之前展开.  
> 如果要在 `main.rs` 中使用 `#[command]`,  
> 记得确保 `#[command]` 标记的函数在 `main()` 函数的前面，例如：
>
> ```rust
>
> #[command]
> fn ls(path: PathBuf){
>
> }
>
> fn main(){
>      aslip::run!("list files.");
> }
> ```

> [!TIP]
> 2
>
> ```rust
> #[command]
> fn f(v: SomeType) {}
> ```
>
> 这里的类型 `SomeType` 必须要实现 `::aslip::from_arg_sttr::FromArgStr` trait。

> [!TIP]
> 3  
> aslip::types:: 中的每个类型都可以作为 `#[command]` 标记的函数的参数。  
> aslip::types:: 中的每个类型都已经实现了 `::aslip::from_arg_sttr::FromArgStr` trait。

> [!TIP]
> 4  
> 如果是用来自定义类型作并且实现了 `FromArgStr` trait, 记得确保这个类型可以在调用 `aslip::run!()` 的地方直接使。

> [!TIP]
> 5  
> 函数的文档注释的第一句话会用作 命令 的 **一句话说明** 打印的 cli 程序的帮助信息中。
> 函数的整个帮助文档会被作为这个 命令 的 quick help 信息 ———— `app cmd -h` 时显示的信息。
>
> ```rust
> /// 函数的文档注释的第一句话会用作 命令 的 **一句话说明** 打印的 cli 程序的帮助信息中。
> /// 整个帮助文档会被作为这个 命令 的 quick help 信息 ———— `app cmd -h` 时显示的信息。
> ///
> #[command]
> pub fn cmd_doc_example() {
>
> }
> ```

**a** **s**imple command **li**ne argument **p**arser.

常见错误及处理办法：

1:

error: Unknown color attribute:

复现方式：

```rust
#[command]
/// list files.
/// Usage:
/// aslip*example ls <PATH> [OPTIONS...]
fn ls(path: FolderPath, options: Vec<String>) { /* ... \_/ }
```

这是因为函数的文档注释会交给 color_print::cstr!() 宏去处理，
而 color_print::cstr!() 宏会使用 <red>red text</red> 这样的标签来给文字添加样式，

解决方法：
修改为 <<PATH>>,
在荡阴帮助文档的时候就会显示为 <PATH> 了。
接着重启 rust-analyzer.

同类 crate：  
 [clap](https://crates.io/crates/aslip) -- Create your command-line parser, with all of the bells and whistles, declaratively or procedurally.
