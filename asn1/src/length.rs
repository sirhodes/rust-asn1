

// various errors that can occurs during decoding
#[derive(Debug, PartialEq)]
pub enum LengthError {
    InsufficentBytes,
    UnsupportedLength(u8) // number of bytes
}

#[derive(Debug, PartialEq)]
pub enum Length {
    None,
    Single(u8),
    Extended(usize),
}

// (number of bytes consumed, result) or error
pub type ParseResult =  Result<(usize, Length), LengthError>;

pub fn read_len(data: &[u8]) -> ParseResult {
    if data.is_empty() {
            return Err(LengthError::InsufficentBytes);
    }

    let top_bit = data[0] & 0b10000000;
    let count = data[0] & 0b01111111;

    if top_bit == 0 { // single byte length
        Ok((1,Length::Single(count)))
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
        Ok((acc+1, Length::Extended(data[0] as usize)))
    }
}

fn read_two_byte_len(acc: usize, data: &[u8]) -> ParseResult {
    if data.len() < 2 {
        Err(LengthError::InsufficentBytes)
    } else {
        let value = ((data[0] as usize) << 8) | (data[1] as usize);
        Ok((acc+2, Length::Extended(value)))
    }
}

#[test]
fn returns_error_on_empty_slice() {

    let bytes : [u8; 0] = [];
    let result = read_len(&bytes[..]);

    assert_eq!(Err(LengthError::InsufficentBytes), result);
}

#[test]
fn parses_no_length() {

    let bytes : [u8; 1] = [0x80];
    let result = read_len(&bytes[..]);

    assert_eq!(Ok((1, Length::None)), result);
}

#[test]
fn parses_one_byte_length() {

    let bytes : [u8; 1] = [0x07];
    let result = read_len(&bytes[..]);

    assert_eq!(Ok((1, Length::Single(7))), result);
}

#[test]
fn parses_one_byte_extended_length() {

    let bytes : [u8; 2] = [0x81, 254];
    let result = read_len(&bytes[..]);

    assert_eq!(Ok((2, Length::Extended(254))), result);
}

#[test]
fn parses_two_byte_extendend_length() {

    let bytes : [u8; 3] = [0x82, 0x01, 0xFF];
    let result = read_len(&bytes[..]);

    assert_eq!(Ok((3, Length::Extended(256+255))), result);
}
