pub use self::tag::{TypeId, Tag, Class};
pub use self::length::{read_len, Length, LengthError};
pub use self::base64::decode::{decode};
pub use self::base64::encode::{encode, encode_as_string};

mod tag;
mod length;
mod base64 {
    pub mod encode;
    pub mod decode;
}
