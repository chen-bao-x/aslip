// # ::core::str::FromStr

// url email

#[derive(Debug)]
pub struct ParseNumberInRangeError;

#[derive(Debug, Clone)]
pub struct NumberInRange<const START: isize, const END: isize> {
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

// impl<T> core::str::FromStr for Box<T>
// where
//     T: core::str::FromStr,
// {
//     type Err = <T as core::str::FromStr>::Err;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         T::from_str(s).map(|x| Box::new(x))
//     }
// }

pub use String;
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
pub use std::path::PathBuf;
