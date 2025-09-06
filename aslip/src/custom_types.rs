use std::str::FromStr;

use crate::from_arg_sttr::FromArgStr;
use crate::from_arg_sttr::ParseError;
use color_print::cformat;
use owo_colors::OwoColorize;

/// 一个数字。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NumberInRange<const MIN: isize, const MAX: isize> {
    pub value: f64,
}

impl<const MIN: isize, const MAX: isize> FromArgStr for NumberInRange<MIN, MAX> {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        let re = <f64 as ::core::str::FromStr>::from_str(s).map_err(|_e| {
            crate::from_arg_sttr::ParseError {
                err_msg: format!(
                    "参数错误。{val} 不是类型 {ty} 的值。",
                    val = s,
                    ty = "NumberInRange"
                ),
                tips: format!(
                    "示例: {min} {max}",
                    min = MIN.green().bold(),
                    max = MAX.green().bold()
                ),
            }
        })?;

        let min = MIN as f64;
        let max = MAX as f64;

        if re >= min && re <= max {
            // ok

            return Ok(Self { value: re });
        } else {
            // err
            let err_msg = {
                if re < min {
                    format!(
                        "{re} 不能小于 {min}",
                        re = re.to_string().green().bold(),
                        min = min.to_string().green().bold()
                    )
                } else {
                    format!(
                        "{a} 不能大于 {m}",
                        a = re.to_string().green().bold(),
                        m = min.to_string().green().bold()
                    )
                }
            };
            return Err(ParseError {
                err_msg: err_msg,
                tips: "".into(),
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OnOff {
    On,
    Off,
}
impl crate::from_arg_sttr::FromArgStr for OnOff {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        return match s {
            "on" => Ok(OnOff::On),
            "off" => Ok(OnOff::Off),

            x => Err(ParseError {
                err_msg: format!("参数错误。{val} 不是类型 ON_OFF 的值。", val = x,),
                tips: format!(
                    "提示：{} 类型的值只有 {on} 和 {off}",
                    "ON_OFF".cyan().bold(),
                    on = "\"on\"".green().bold(),
                    off = "\"off\"".green().bold(),
                ),
            }),
        };
    }
}

/// file path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilePath {
    pub path: std::path::PathBuf,
}
impl FromArgStr for FilePath {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::path::PathBuf::from_str(s)
            .map_err(|_e| ParseError {
                err_msg: cformat!(
                    "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                    s,
                    "FilePath",
                ),

                tips: "".to_string(),
            })
            .and_then(|x| {
                if x.is_file() {
                    Ok(Self { path: x })
                } else {
                    Err(ParseError {
                        err_msg: cformat!("{} 不是文件。", x.to_str().unwrap_or_default(),),

                        tips: "示例: /path/to/file.txt".to_string(),
                    })
                }
            })
    }
}

/// folder path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FolderPath {
    pub path: std::path::PathBuf,
}
impl FromArgStr for FolderPath {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        std::path::PathBuf::from_str(s)
            .map_err(|_e| ParseError {
                err_msg: cformat!(
                    "将 <green,bold>{}</> 转换为 <cyan,bold>{}</> 时出错.",
                    s,
                    "FolderPath",
                ),

                tips: "".to_string(),
            })
            .and_then(|x| {
                if x.is_dir() {
                    Ok(Self { path: x })
                } else {
                    Err(ParseError {
                        err_msg: cformat!("{} 不是文件夹。", x.to_str().unwrap_or_default(),),

                        tips: "示例: /path/to/folder/".to_string(),
                    })
                }
            })
    }
}

pub trait Have {
    fn aslip_have_str(&self, val: &str) -> bool;
    fn aslip_have_all_strs<const N: usize>(&self, flags: [&str; N]) -> bool;
}
impl Have for Vec<String> {
    fn aslip_have_str(&self, val: &str) -> bool {
        self.contains(&val.to_string())
    }

    fn aslip_have_all_strs<const N: usize>(&self, flags: [&str; N]) -> bool {
        for x in 0..N {
            if !self.contains(&flags[x].to_string()) {
                return false;
            }
        }

        return true;
    }
}
#[test]
fn test_have_functions() {
    let arr: Vec<String> = ["a", "b", "c", "d"]
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect();

    assert!(arr.aslip_have_str("a"));
    assert!(arr.aslip_have_str("c"));

    assert!(arr.aslip_have_all_strs(["a", "b"]));
}

/// "-p" "--path" 这样的 flag，
/// flag 目前不支持 参数。
#[derive(Debug)]
pub struct Flag {
    pub value: String,
}
impl Flag {
    fn new(s: &str) -> Self {
        return Self { value: s.into() };
    }
}

impl FromArgStr for Flag {
    fn from_arg_str(s: &str) -> Result<Self, ParseError> {
        return match s {
            "-" => Err(ParseError {
                err_msg: format!("需要 flag 的名称"),
                tips: cformat!("示例: -h"),
            }),
            "--" => Err(ParseError {
                err_msg: format!("需要 flag 的名称"),
                tips: cformat!("示例: --help"),
            }),
            x if x.starts_with("--") && s.len() > 2 => Ok(Flag::new(x)),
            x if x.starts_with("-") && x.len() == 2 => Ok(Flag::new(x)),
            s if s.starts_with("-") && s.len() >= 2 => Err(ParseError {
                err_msg: format!(
                    "一个短划线开头的 flag 短划线后面只能有一个字符，是 flag 的简写形式"
                ),
                tips: cformat!("示例: --help"),
            }),
            s => Err(ParseError {
                err_msg: cformat!("<green,s>\"{}\"</> 不是 flag", s),
                tips: cformat!("flag 需要以 <green,s>\"-\"</> 或者 <green,s>\"--\"</> 开头。"),
            }),
        };
    }
}
