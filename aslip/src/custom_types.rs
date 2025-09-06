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
                        err_msg: cformat!("{} 不是文件。", x.to_str().unwrap_or_default(),),

                        tips: "示例: /path/to/folder/".to_string(),
                    })
                }
            })
    }
}
