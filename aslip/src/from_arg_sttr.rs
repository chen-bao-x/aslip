use color_print::{cformat, cstr};
use owo_colors::OwoColorize;
use std::str::FromStr;

// -=-----------------------------------------------------------------------------------

/// ## 所有使用 `#[command]` 标记的函数的参数类型都需要实现 `FromArgStr` trait.
/// #### Example:
/// ```rust
/// use aslip::FromArgStr;
/// use aslip::from_arg_sttr::ParseError;
/// 
/// struct CustomType {}
/// impl FromArgStr for CustomType {
///     fn from_arg_str(s: &str) -> Result<Self, ParseError> {
///          Ok(Self{})
///     }
/// }
/// ```
///
pub trait FromArgStr: Sized {
    fn from_arg_str(s: &str) -> Result<Self, ParseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// 提示为什么出错。
    pub err_msg: String,

    /// 提示如何填写正确的参数。
    pub tips: String,
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

impl FromArgStr for String {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        String::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "String",
                std::any::type_name_of_val(&e)
            ),

            tips: "这种错误理论上可能出现，希望你能将如何触发这个错误的方法告诉这个程序的开发者。"
                .bold()
                .to_string(),
        })
    }
}
impl FromArgStr for bool {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        bool::from_str(s).map_err(|_| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "bool",
            ),

            tips: cstr!("<cyan!>bool</> 的值只有 <green,s>true</> 和 <green,s>false</>").into(),
        })
    }
}
impl FromArgStr for char {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        char::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "char",
                std::any::type_name_of_val(&e)
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = "'a'".green().bold(),
                custom = "'c'".green().bold(),
                max = "'Z'".green().bold()
            ),
        })
    }
}
impl FromArgStr for i8 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        i8::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "i8",
                std::any::type_name_of_val(&e)
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = i8::MIN.green().bold(),
                custom = 123.green().bold().green().bold(),
                max = i8::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for u8 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        u8::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "u8",
                std::any::type_name_of_val(&e)
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = u8::MIN.green().bold(),
                custom = 123.green().bold(),
                max = u8::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for i16 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        i16::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "i16",
                std::any::type_name_of_val(&e)
            ),
            tips: format!(
                "示例: {min} {custom} {max}",
                min = i16::MIN.green().bold(),
                custom = 123.green().bold(),
                max = i16::MAX.green().bold(),
            ),
        })
    }
}
impl FromArgStr for u16 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        u16::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "u16",
                std::any::type_name_of_val(&e)
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = u16::MIN.green().bold(),
                custom = 123.green().bold(),
                max = u16::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for i32 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        i32::from_str(s).map_err(|e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错：{}",
                s,
                "i32",
                std::any::type_name_of_val(&e)
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = i32::MIN.green().bold(),
                custom = 123.green().bold(),
                max = i32::MAX
            ),
        })
    }
}
impl FromArgStr for u32 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        u32::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "u32",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = u32::MIN.green().bold(),
                custom = 123.green().bold(),
                max = u32::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for f32 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        f32::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "f32",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = f32::MIN.green().bold(),
                custom = 123.green().bold(),
                max = f32::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for i64 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        i64::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "i64",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = i64::MIN.green().bold(),
                custom = 123.green().bold(),
                max = i64::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for u64 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        u64::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "u64",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = u64::MIN.green().bold(),
                custom = 123.green().bold(),
                max = u64::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for f64 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        f64::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "f64",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = f64::MIN.green().bold(),
                custom = 123.green().bold(),
                max = f64::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for i128 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        i128::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "i128",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = i128::MIN.green().bold(),
                custom = 123,
                max = i128::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for u128 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        u128::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "u128",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = u128::MIN.green().bold(),
                custom = 123.green().bold(),
                max = u128::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for isize {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        isize::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "isize",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = isize::MIN.green().bold(),
                custom = 123.green().bold(),
                max = isize::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for usize {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        usize::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "usize",
            ),

            tips: format!(
                "示例: {min} {custom} {max}",
                min = usize::MIN.green().bold(),
                custom = 123.green().bold(),
                max = usize::MAX.green().bold()
            ),
        })
    }
}
impl FromArgStr for std::ffi::CString {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::ffi::CString::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::ffi::CString",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::ffi::OsString {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::ffi::OsString::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::ffi::OsString",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::IpAddr {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::IpAddr::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::IpAddr",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::Ipv4Addr {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::Ipv4Addr::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::Ipv4Addr",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::Ipv6Addr {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::Ipv6Addr::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::Ipv6Addr",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::SocketAddr {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::SocketAddr::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::SocketAddr",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::SocketAddrV4 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::SocketAddrV4::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::SocketAddrV4",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::net::SocketAddrV6 {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::net::SocketAddrV6::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::net::SocketAddrV6",
            ),

            tips: "".to_string(),
        })
    }
}
impl FromArgStr for std::path::PathBuf {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::path::PathBuf::from_str(s).map_err(|_e| ParseError {
            err_msg: cformat!(
                "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                s,
                "std::path::PathBuf",
            ),

            tips: "".to_string(),
        })
    }
}

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

pub fn single_type_converter<T: FromArgStr>(
    app: &crate::App,
    arg_name: &str,
    arg_index: usize,
) -> T {
    let arg: &String = app
        ._user_inputed_cmd_args
        .get(arg_index)
        .expect(&format!("需要参数：<{arg_name}>"));

    use owo_colors::OwoColorize;

    let asdf = <T as FromArgStr>::from_arg_str(arg);

    match asdf {
        Ok(val) => return val,
        Err(e) => {
            eprintln!("{err_marker}: {e}", err_marker = "error: ".red().bold(),);

            std::process::exit(1);
        }
    };
}

pub fn vec_type_converter<T: FromArgStr>(args: &[String]) -> Vec<T> {
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
