use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct ParseNumberInRangeError;

/// 一个数字。
#[derive(Debug, Clone)]
pub struct NumberInRange<const MIN: isize, const MAX: isize> {
    pub value: f64,
}

impl<const START: isize, const END: isize> ::core::str::FromStr for NumberInRange<START, END> {
    type Err = ParseNumberInRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = f64::from_str(s).map_err(|_| ParseNumberInRangeError {})?;

        let s = START as f64;
        let e = END as f64;

        if re >= s && re <= e {
            // ok

            return Ok(Self { value: re });
        } else {
            // err
            return Err(ParseNumberInRangeError {});
        }
    }
}

impl<const START: isize, const END: isize> crate::from_arg_sttr::FromArgStr
    for NumberInRange<START, END>
{
    type Err = ParseNumberInRangeError;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            <f64 as ::core::str::FromStr>::from_str(s).map_err(|_| ParseNumberInRangeError {})?;

        let s = START as f64;
        let e = END as f64;

        if re >= s && re <= e {
            // ok

            return Ok(Self { value: re });
        } else {
            // err
            return Err(ParseNumberInRangeError {});
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OnOff {
    On,
    Off,
}

impl crate::from_arg_sttr::FromArgStr for OnOff {
    type Err = String;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "on" => Ok(OnOff::On),
            "off" => Ok(OnOff::Off),

            x => {
                let err_msg = format!(
                    "{err}参数错误。{val} 不是类型 ON_OFF 的值。
提示：ON_OFF 类型的值只有 {on} 和 {off}",
                    err = "error: ".bold().red(),
                    val = x,
                    on = "\"on\"",
                    off = "\"off\"",
                );

                Err(err_msg)
            }
        };
    }
}
