// various errors that can occurs during decoding
pub enum Error {
    InsufficentBytesForLength,
    UnsupportedLength(u8)
}
