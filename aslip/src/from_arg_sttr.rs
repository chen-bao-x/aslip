/// 用来检查某个类型是否实现了 `FromArgStr` trait.
/// 主要是给 aslip_macro::command 用的。
/// # Example:
/// ```rust,ignore
/// use aslip::from_arg_sttr::from_arg_str_trait_bound_check;
/// const _: () = { from_arg_str_trait_bound_check::<u8>() }; // 生成一个空的 const 用来触发编译时检查
///
/// struct CustomType {}
/// const _: () = { from_arg_str_trait_bound_check::<CustomType>() };
/// // the trait bound `CustomType: FromArgStr` is not satisfied the following other types implement trait `FromArgStr`
/// ```
///
pub const fn from_arg_str_trait_bound_check<T: FromArgStr>() {}

/// 所有使用 `#[command]` 标记的函数的参数类型都需要实现了 `::aslip::from_arg_sttr::FromArgStr` trait.
/// # Example:
/// ```rust
/// use aslip::FromArgStr;
/// struct CustomType {}
/// impl FromArgStr for CustomType {
///     type Err = std::fmt::Error;
///     fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
///          Ok(Self{})
///     }
/// }
/// ```
///
pub trait FromArgStr: Sized {
    type Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err>;
}

// single types

// 代码生成。
// #[test]
// fn starter() {
//     let arr = [
//         "String",
//         "bool",
//         "char",
//         "i8",
//         "u8",
//         "i16",
//         "u16",
//         "i32",
//         "u32",
//         "f32",
//         "i64",
//         "u64",
//         "f64",
//         "i128",
//         "u128",
//         "isize",
//         "usize",
//         "std::ffi::CString",
//         "std::ffi::OsString",
//         "std::net::IpAddr",
//         "std::net::Ipv4Addr",
//         "std::net::Ipv6Addr",
//         "std::net::SocketAddr",
//         "std::net::SocketAddrV4",
//         "std::net::SocketAddrV6",
//         "std::path::PathBuf",
//     ];
//
//     let new_arr = arr.map(code_gen);
// }
//
// fn code_gen(ty_name: &str) {
//     println!(
//         r###"
// impl FromArgStr for {} {{
//     type Err = <{} as ::std::str::FromStr>::Err;
//     fn from_arg_str(s: &str) -> Result<Self, Self::Err> {{
//         {}::from_str(s)
//     }}
// }}
//
// "###,
//         ty_name, ty_name, ty_name
//     )
// }

impl FromArgStr for String {
    type Err = <String as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        String::from_str(s)
    }
}
impl FromArgStr for bool {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        bool::from_str(s)
    }
}
impl FromArgStr for char {
    type Err = <char as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        char::from_str(s)
    }
}
impl FromArgStr for i8 {
    type Err = <i8 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        i8::from_str(s)
    }
}
impl FromArgStr for u8 {
    type Err = <u8 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str(s)
    }
}
impl FromArgStr for i16 {
    type Err = <i16 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        i16::from_str(s)
    }
}
impl FromArgStr for u16 {
    type Err = <u16 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str(s)
    }
}
impl FromArgStr for i32 {
    type Err = <i32 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s)
    }
}
impl FromArgStr for u32 {
    type Err = <u32 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s)
    }
}
impl FromArgStr for f32 {
    type Err = <f32 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        f32::from_str(s)
    }
}
impl FromArgStr for i64 {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        i64::from_str(s)
    }
}
impl FromArgStr for u64 {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        u64::from_str(s)
    }
}
impl FromArgStr for f64 {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        f64::from_str(s)
    }
}
impl FromArgStr for i128 {
    type Err = <i128 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        i128::from_str(s)
    }
}
impl FromArgStr for u128 {
    type Err = <u128 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        u128::from_str(s)
    }
}
impl FromArgStr for isize {
    type Err = <isize as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        isize::from_str(s)
    }
}
impl FromArgStr for usize {
    type Err = <usize as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s)
    }
}
impl FromArgStr for std::ffi::CString {
    type Err = <std::ffi::CString as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::ffi::CString::from_str(s)
    }
}
impl FromArgStr for std::ffi::OsString {
    type Err = <std::ffi::OsString as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::ffi::OsString::from_str(s)
    }
}
impl FromArgStr for std::net::IpAddr {
    type Err = <std::net::IpAddr as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::IpAddr::from_str(s)
    }
}
impl FromArgStr for std::net::Ipv4Addr {
    type Err = <std::net::Ipv4Addr as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::Ipv4Addr::from_str(s)
    }
}
impl FromArgStr for std::net::Ipv6Addr {
    type Err = <std::net::Ipv6Addr as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::Ipv6Addr::from_str(s)
    }
}
impl FromArgStr for std::net::SocketAddr {
    type Err = <std::net::SocketAddr as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::SocketAddr::from_str(s)
    }
}
impl FromArgStr for std::net::SocketAddrV4 {
    type Err = <std::net::SocketAddrV4 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::SocketAddrV4::from_str(s)
    }
}
impl FromArgStr for std::net::SocketAddrV6 {
    type Err = <std::net::SocketAddrV6 as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::net::SocketAddrV6::from_str(s)
    }
}
impl FromArgStr for std::path::PathBuf {
    type Err = <std::path::PathBuf as ::std::str::FromStr>::Err;
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        std::path::PathBuf::from_str(s)
    }
}

// 包装类型

impl<T: FromArgStr> FromArgStr for Box<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| Box::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for ::std::rc::Rc<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| ::std::rc::Rc::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for std::sync::Arc<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| std::sync::Arc::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for std::cell::RefCell<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| std::cell::RefCell::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for std::sync::Mutex<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| std::sync::Mutex::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for std::sync::RwLock<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| std::sync::RwLock::new(x))
    }
}
impl<T: FromArgStr> FromArgStr for std::cell::Cell<T> {
    type Err = <T as FromArgStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        T::from_arg_str(s).map(|x| std::cell::Cell::new(x))
    }
}

use std::str::FromStr;

use color_print::cformat;

// -=-----------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ParseError {
    /// 提示为什么出错。
    err_msg: String,

    /// 提示如何填写正确的参数。
    tips: String,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.err_msg, self.tips)
    }
}

impl ParseError {
    pub const fn empty() -> Self {
        Self {
            err_msg: String::new(),
            tips: String::new(),
        }
    }
}

pub trait FromArgStr_v3: Sized {
    fn from_arg_str(s: &str) -> Result<Self, ParseError>;
}

impl FromArgStr_v3 for String {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        String::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 是出错。",
                s,
                "String"
            ),

            tips: "".to_string(),
        })
    }
}

// 代码生成。
#[test]
fn starter() {
    let arr = [
        "String",
        "bool",
        "char",
        "i8",
        "u8",
        "i16",
        "u16",
        "i32",
        "u32",
        "f32",
        "i64",
        "u64",
        "f64",
        "i128",
        "u128",
        "isize",
        "usize",
        "std::ffi::CString",
        "std::ffi::OsString",
        "std::net::IpAddr",
        "std::net::Ipv4Addr",
        "std::net::Ipv6Addr",
        "std::net::SocketAddr",
        "std::net::SocketAddrV4",
        "std::net::SocketAddrV6",
        "std::path::PathBuf",
    ];

    let new_arr = arr.map(code_gen);
}

fn code_gen(ty_name: &str) {
    println!(
        r###"

impl FromArgStr_v3 for {ty_name} {{
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {{
        {ty_name}::from_str(s).map_err(|e| ParseError {{
            err_msg: cformat!(
                "将 <green,bold>{{}}</> 转换为 <cyan,bold>{{}}</> 时出错：{{}}",
                s,
                "{ty_name}",std::any::type_name_of_val(&e)
            ),

            tips: "".to_string(),
        }})
    }}
}}

"###,
    )
}
