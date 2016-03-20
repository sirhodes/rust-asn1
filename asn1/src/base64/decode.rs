

//  this would likely be faster with a 256-byte table, but
//  damn, the patteren matching is really slick
fn get_value(c: u8) -> Option<u8> {

    match c as char {
        // ascii 65 -> 90, base64 0 -> 25
        'A' ... 'Z' => Some(c as u8 - 65),
        // ascii 97 - 122, base64 26 -> 51
        'a' ... 'z' => Some(c as u8 - 71),
        // ascii 48 - 57, base64 52 -> 61
        '0' ... '9' => Some(c as u8 + 4),

        '+' => Some(62),
        '/' => Some(63),

        _ => None,
    }
}

/// whitespace characters that are ignored during decoding
fn is_whitespace(c: u8) -> bool {
    match c as char {
        '\n' => true,
        '\r' => true,
        '\t' => true,
        ' ' => true,
        _ => false,
    }
}

struct SkipWhitespace<'a> {
    bytes: &'a[u8],
    pos: usize
}

impl<'a> SkipWhitespace<'a> {
    fn new(bytes: &'a[u8]) -> SkipWhitespace<'a> {
        SkipWhitespace {
            bytes: bytes,
            pos: 0,
        }
    }
}

impl<'a> Iterator for SkipWhitespace<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        while self.pos < self.bytes.len() {
            let value = self.bytes[self.pos];
            self.pos += 1;
            if !is_whitespace(value) {
                return Some(value)
            }
        }
        None
    }
}

fn get_first_byte(b1: u8, b2: u8) -> u8 {
    ((b1 & 0b00111111) << 2) | ((b2 & 0b00110000) >> 4)
}

fn get_second_byte(b2: u8, b3: u8) -> u8 {
    ((b2 & 0b00001111) << 4) | ((b3 & 0b00111100) >> 2)
}

fn get_third_byte(b3: u8, b4: u8) -> u8 {
    ((b3 & 0b00000011) << 6) | (b4 & 0b00111111)
}

pub trait ByteWriter {
    fn write(self: &mut Self, b: u8);
}

#[derive(Debug, PartialEq)]
pub enum DecodeErr {
    NotMultFour,
    BadValue(u8),
    BadEndChar(u8)
}

// returns the number of bytes written or an error
pub fn decode<T : ByteWriter>(bytes: &[u8], writer: &mut T) -> Option<DecodeErr> {

    let mut iter = SkipWhitespace::new(bytes);

    loop {

        let c1 = match iter.next() {
            Some(c) => c,
            None => return None, // success! we reached the end of input on an multiple of 4
        };

        let c2 = match iter.next() {
            Some(c) => c,
            None => return Some(DecodeErr::NotMultFour),
        };

        let c3 = match iter.next() {
            Some(c) => c,
            None => return Some(DecodeErr::NotMultFour),
        };

        let c4 = match iter.next() {
            Some(c) => c,
            None => return Some(DecodeErr::NotMultFour),
        };

        match (c1, c2, c3, c4) {
            (a, b, b'=', b'=') => {
                let v1 = match get_value(a) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(a)),
                };
                let v2 = match get_value(b) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(b)),
                };
                writer.write(get_first_byte(v1, v2));
                return iter.next().map(|b| DecodeErr::BadEndChar(b)); //  must be end of input
            },
            (a, b, c, b'=') => {
                let v1 = match get_value(a) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(a)),
                };
                let v2 = match get_value(b) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(b)),
                };
                let v3 = match get_value(c) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(c)),
                };
                writer.write(get_first_byte(v1, v2));
                writer.write(get_second_byte(v2, v3));
                return iter.next().map(|b| DecodeErr::BadEndChar(b)); //  must be end of input
            },
            (a, b, c, d) => {
                let v1 = match get_value(a) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(a)),
                };
                let v2 = match get_value(b) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(b)),
                };
                let v3 = match get_value(c) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(c)),
                };
                let v4 = match get_value(d) {
                    Some(v) => v,
                    None => return Some(DecodeErr::BadValue(d)),
                };
                writer.write(get_first_byte(v1, v2));
                writer.write(get_second_byte(v2, v3));
                writer.write(get_third_byte(v3, v4));
            },
        }
    }
}

impl ByteWriter for Vec<u8> {
    fn write(self: &mut Self, b: u8) {
        self.push(b);
    }
}

pub fn decode_as_vec(bytes: &[u8]) -> Result<Vec<u8>, DecodeErr> {
    let mut vec : Vec<u8> = Vec::new();
    match decode(bytes, &mut vec) {
        None => Ok(vec),
        Some(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_success(input: &[u8], output: &[u8])
    {
        let mut vec : Vec<u8> = Vec::new();
        let result = decode(input, &mut vec);
        assert_eq!(None, result);
        assert_eq!(&vec[..], output);
    }

    fn test_failure(input: &[u8], err: DecodeErr)
    {
        let mut vec : Vec<u8> = Vec::new();
        let result = decode(input, &mut vec);
        assert_eq!(Some(err), result);
    }

    #[test]
    fn rejects_bad_size() {
        test_failure(b"TQ=", DecodeErr::NotMultFour);
    }

    #[test]
    fn rejects_trailing_bytes() {
        test_failure(b"TQ==TWFu", DecodeErr::BadEndChar(b'T'));
    }

    #[test]
    fn rejects_bad_characters() {
        test_failure(b"TQ!=", DecodeErr::BadValue(b'!'));
    }

    #[test]
    fn correctly_decodes_one_byte() {
        test_success(b"TQ==", b"M");
    }

    #[test]
    fn correctly_skips_whitespace() {
        test_success(b"\r\nT Q =\t=\t\r\n", b"M");
    }

    #[test]
    fn correctly_decodes_two_bytes() {
        test_success(b"TWE=", b"Ma");
    }

    #[test]
    fn correctly_decodes_three_bytes() {
        test_success(b"TWFu", b"Man");
    }

    #[test]
    fn correctly_decodes_six_bytes() {
        test_success(b"TWFuTQ==", b"ManM");
    }

    #[test]
    fn correctly_decodes_long_input() {

        let input =     b"TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGl\
                        zIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
                        dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlud\
                        WVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZS\
                        BzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

        let result =    b"Man is distinguished, not only by his reason, but by this singular passion from \
                        other animals, which is a lust of the mind, that by a perseverance of delight in the \
                        continued and indefatigable generation of knowledge, exceeds the short vehemence of \
                        any carnal pleasure.";

        test_success(input, result);
    }
}
