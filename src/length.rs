use std::u8;

// various errors that can occurs during decoding
#[derive(Debug, PartialEq)]
pub enum LengthErr {
    InsufficentBytes,
    BadRepresentation(u8, usize),   // Not encoded in minimum number of bytes: (# bytes, value)
    UnsupportedLength(u8)           // number of bytes
}

// (number of bytes consumed, value) or error
pub type LengthResult =  Result<(usize, usize), LengthErr>;

pub fn read_len(data: &[u8]) -> LengthResult {

    if data.is_empty() {
            return Err(LengthErr::InsufficentBytes);
    }

    let len     = data[0];
    let count   = len & 0b01111111u8;
    let top_bit = len & 0b10000000u8;

    if top_bit == 0 { // single byte length
        Ok((1, count as usize))
    }
    else { // number of bytes that follow
        match count {
            1 => read_one_byte_len(1, &data[1..]),
            2 => read_two_byte_len(1, &data[1..]),
            // TODO: support 3 and 4 byte lengths?
            // includes Indeterminate count of 0
            _ => Err(LengthErr::UnsupportedLength(len)),
        }
    }
}

fn read_one_byte_len(acc: usize, data: &[u8]) -> LengthResult {
    if data.is_empty() {
        Err(LengthErr::InsufficentBytes)
    } else {
        let value = data[0] as usize;
        if value <= 127 {
            Err(LengthErr::BadRepresentation(1, value))
        } else {
            Ok((acc+1, value))
        }
    }
}

fn read_two_byte_len(acc: usize, data: &[u8]) -> LengthResult {
    if data.len() < 2 {
        Err(LengthErr::InsufficentBytes)
    } else {
        let value = ((data[0] as usize) << 8) | (data[1] as usize);
        if value <= 255 {
            Err(LengthErr::BadRepresentation(2, value))
        } else {
            Ok((acc+2, value))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fails_on_empty_slice() {
        assert_eq!(Err(LengthErr::InsufficentBytes), read_len(b""));
    }

    #[test]
    fn fails_on_indeterminate_length() {
        assert_eq!(Err(LengthErr::UnsupportedLength(0x80)), read_len(&[0x80]));
    }

    #[test]
    fn parses_one_byte_length() {
        assert_eq!(Ok((1, 7)), read_len(&[0x07]));
    }

    #[test]
    fn fails_on_bad__one_byte_extended_length() {
        assert_eq!(Err(LengthErr::BadRepresentation(1, 127)), read_len(&[0x81, 0x7F]));
    }

    #[test]
    fn parses_one_byte_extended_length() {
        assert_eq!(Ok((2, 254)), read_len(&[0x81, 254]));
    }

    #[test]
    fn parses_two_byte_extendend_length() {
        assert_eq!(Ok((3, 256+255)), read_len(&[0x82, 0x01, 0xFF]));
    }

    #[test]
    fn fails_on_bad_two_byte_extendend_length() {
        assert_eq!(Err(LengthErr::BadRepresentation(2, 255)), read_len(&[0x82, 0x00, 0xFF]));
    }
}
