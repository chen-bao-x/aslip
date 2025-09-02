// 标准库里的很多都没有实现 FromStr trait，所以我自己做一个，让更多的类型能转换自 &str.

use std::str::FromStr;

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

/// 给实现了 `core::str::FromStr` trait 的类型实现 `FromArgStr` trait。
/// String
/// bool
/// char
/// f32
/// f64
/// i8
/// i16
/// i32
/// i64
/// i128
/// isize
/// u8
/// u16
/// u32
/// u64
/// u128
/// usize
/// std::ffi::CString
/// std::ffi::OsString
/// std::net::IpAddr
/// std::net::Ipv4Addr
/// std::net::Ipv6Addr
/// std::net::SocketAddr
/// std::net::SocketAddrV4
/// std::net::SocketAddrV6
/// std::path::PathBuf
impl<T> FromArgStr for T
where
    T: core::str::FromStr,
{
    type Err = <T as core::str::FromStr>::Err;

    fn from_arg_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
