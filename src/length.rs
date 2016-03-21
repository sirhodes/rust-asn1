

// various errors that can occurs during decoding
#[derive(Debug, PartialEq)]
pub enum LengthError {
    InsufficentBytes,
    UnsupportedLength(u8) // number of bytes
}

#[derive(Debug, PartialEq)]
pub enum Length {
    None,
    Value(usize)
}

// (number of bytes consumed, result) or error
pub type ParseResult =  Result<(usize, Length), LengthError>;

pub fn read_len(data: &[u8]) -> ParseResult {

    if data.is_empty() {
            return Err(LengthError::InsufficentBytes);
    }

    let count   = data[0] & 0b01111111u8;
    let top_bit = data[0] & 0b10000000u8;

    if top_bit == 0 { // single byte length
        Ok((1,Length::Value(count as usize)))
    }
    else { // number of bytes that follow
        match count {
            0 => Ok((1, Length::None)), // no length
            1 => read_one_byte_len(1, &data[1..]),
            2 => read_two_byte_len(1, &data[1..]),
            // TODO: support 3 and 4 byte lengths?
            x => Err(LengthError::UnsupportedLength(x)),
        }
    }
}

fn read_one_byte_len(acc: usize, data: &[u8]) -> ParseResult {
    if data.is_empty() {
        Err(LengthError::InsufficentBytes)
    } else {
        Ok((acc+1, Length::Value(data[0] as usize)))
    }
}

fn read_two_byte_len(acc: usize, data: &[u8]) -> ParseResult {
    if data.len() < 2 {
        Err(LengthError::InsufficentBytes)
    } else {
        let value = ((data[0] as usize) << 8) | (data[1] as usize);
        Ok((acc+2, Length::Value(value)))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn returns_error_on_empty_slice() {
        assert_eq!(Err(LengthError::InsufficentBytes), read_len(b""));
    }

    #[test]
    fn parses_no_length() {
        assert_eq!(Ok((1, Length::None)), read_len(&[0x80]));
    }

    #[test]
    fn parses_one_byte_length() {
        assert_eq!(Ok((1, Length::Value(7))), read_len(&[0x07]));
    }

    #[test]
    fn parses_one_byte_extended_length() {
        assert_eq!(Ok((2, Length::Value(254))), read_len(&[0x81, 254]));
    }

    #[test]
    fn parses_two_byte_extendend_length() {
        assert_eq!(Ok((3, Length::Value(256+255))), read_len(&[0x82, 0x01, 0xFF]));
    }
}
