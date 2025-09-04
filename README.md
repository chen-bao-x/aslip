# **aslip**

> [!NOTE]
> Are you sure you want to claim this million?  
> With just a slip of your little hand,  
> you pressed the confirm button.
> 
> a slip
> 
> aslip

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
> 如果是用来自定义类型作病实现了 `FromArgStr`, 记得确保这个类型可以在调用 `aslip::run!()` 的地方直接使用而不用写 use path。

> [!TIP]
> 5  
> 函数的文档注释的第一句话会用作 命令 的 **一句话说明** 打印的 cli 程序的帮助信息中。
> 整个帮助文档会被作为这个 命令 的 quick help 信息 ———— `app cmd -h` 时显示的信息。
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
