

// various errors that can occurs during decoding
#[derive(Debug, PartialEq)]
pub enum Error {
    InsufficentBytesForLength,
    UnsupportedLength(u8)
}
