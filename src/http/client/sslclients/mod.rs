//! SSL client support.
//!
//! Which particular library is used depends upon the configuration used at
//! compile time; at present it can only be OpenSSL (`--cfg openssl`); without
//! that, you won't be able to use SSL (an attempt to make an HTTPS connection
//! will return an error).

#[cfg(not(target_os="android"))]
pub use self::openssl::NetworkStream;
#[cfg(target_os="android")]
pub use self::none::NetworkStream;

#[cfg(not(target_os="android"))]
mod openssl;
#[cfg(target_os="android")]
mod none;
