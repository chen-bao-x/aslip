# aslip

## 重要提示：

> [!TIP]
> 1  
> 尽量在 `main.rs` 中使用 `#[command]`.  
> rust 编译器再宏展开时是有先后顺序的，  
> 在其他文件中使用 `#[command]` 能确保 `#[command]` 在 `aslip::run!()` 之前展开.  
> 如果要在 `main.rs` 中使用 `#[command]`,   
> 记得确保 `#[command]` 标记的函数在 `main()` 函数的前面，例如：
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
> aslip::types:: 中的每个类型都可以作为 `#[command` 标记的函数的参数。

> [!TIP]
> 4  
> 如果是用来自定义类型作病实现了 `FromArgStr`, 记得确保这个类型可以在调用 `aslip::run!()` 的地方直接使用而不用写 use path。
