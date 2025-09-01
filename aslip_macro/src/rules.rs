// 1. cmd action 不能用返回值。
// 2. 只有最后一个参数可以是 数组这样的集合类型。

use std::path::PathBuf;

use syn::ReturnType;

/// rule 1 cmd action 不能用返回值。
pub fn rule_1(input: syn::ItemFn) -> Option<syn::Error> {
    // 检查返回值
    if let syn::ReturnType::Type(_, ty) = &input.sig.output {
        // 有返回值 → 报错

        return Some(syn::Error::new_spanned(
            ty,
            "This function must not have a return value",
        ));
    }
    return None;
}

/// rule 2. 只有最后一个参数可以是 数组这样的集合类型。
pub fn rule_2(input: syn::ItemFn) -> Option<syn::Error> {
    todo!()
}

// 我正在用 proc_macro2 来实现一个 命令行解析工具，用起来大概这样：

fn main() {}

fn ls(path: PathBuf) {}
