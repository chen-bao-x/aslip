///! 标准库里的很多都没有实现 FromStr trait，所以我自己做一个，让更多的类型能转换自 &str.
///! 标准库里的很多都没有实现 FromStr trait，所以我自己做一个，让更多的类型能转换自 &str.
///! 标准库里的很多都没有实现 FromStr trait，所以我自己做一个，让更多的类型能转换自 &str.

/// 所有使用 `#[command]` 标记的函数的参数类型都需要实现了 `::aslip::from_arg_sttr::FromArgStr` trait.
pub trait FromArgStr: Sized {
    /// The associated error which can be returned from parsing.
    type Err;

    /// Parses a string `s` to return a value of this type.
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`]. The error type is specific to the implementation of the trait.
    ///
    /// # Examples
    fn from_arg_str(s: &str) -> Result<Self, Self::Err>;
}

impl<T> FromArgStr for T
where
    T: core::str::FromStr,
{
    type Err = <T as core::str::FromStr>::Err;

    /// 给实现了 `core::str::FromStr` trait 的类型实现 `FromArgStr` trait。  
    ///
    /// `String`   `bool`   `char`  
    ///
    /// `i8`      `u8`  
    ///
    /// `i16`     `u16`  
    ///
    /// `i32`     `u32`     `f32`  
    ///
    /// `i64`     `u64`     `f64`  
    ///
    /// `i128`    `u128`  
    ///
    /// `isize`   `usize`  
    ///
    /// `std::ffi::CString`  
    ///
    /// `std::ffi::OsString`  
    ///
    /// `std::net::IpAddr`  
    ///
    /// `std::net::Ipv4Addr`  
    ///
    /// `std::net::Ipv6Addr`  
    ///
    /// `std::net::SocketAddr`  
    ///
    /// `std::net::SocketAddrV4`  
    ///
    /// `std::net::SocketAddrV6`  
    ///
    /// `std::path::PathBuf`  
    ///
    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
