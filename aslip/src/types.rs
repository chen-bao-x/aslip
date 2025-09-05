///! 这里的类型都可一用作 #[command] 标记的函数的参数。
pub use String;
pub use std::path::PathBuf;

#[allow(type_alias_bounds)]
pub type Vec<T: FromArgStr> = ::std::vec::Vec<T>;

pub use bool;
pub use char;
pub use f32;
pub use f64;
pub use i8;
pub use i16;
pub use i32;
pub use i64;
pub use i128;
pub use isize;
pub use u8;
pub use u16;
pub use u32;
pub use u64;
pub use u128;
pub use usize;

pub use std::ffi::CString;
pub use std::ffi::OsString;
pub use std::net::IpAddr;
pub use std::net::Ipv4Addr;
pub use std::net::Ipv6Addr;
pub use std::net::SocketAddr;
pub use std::net::SocketAddrV4;
pub use std::net::SocketAddrV6;

use crate::FromArgStr;
pub use crate::custom_types::NumberInRange;
pub use crate::custom_types::OnOff;

// email
// url
