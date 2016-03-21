#[derive(Debug, PartialEq)]
pub enum DecodeErr {
    // The non-whitespace input is not a multiple of four
    NotMultFour,
    // The input contains non-whitespace characters after the terminating padding
    BadValue(u8),
    // The input contains a non-base64 value
    BadEndChar(u8)
}

//  this would likely be faster with a 256-byte table, but
//  damn, the pattern matching is really slick
fn get_value(c: u8) -> Result<u8, DecodeErr> {

    match c as char {
        // ascii 65 -> 90, base64 0 -> 25
        'A' ... 'Z' => Ok(c as u8 - 65),
        // ascii 97 - 122, base64 26 -> 51
        'a' ... 'z' => Ok(c as u8 - 71),
        // ascii 48 - 57, base64 52 -> 61
        '0' ... '9' => Ok(c as u8 + 4),

        '+' => Ok(62),
        '/' => Ok(63),

        _ => Err(DecodeErr::BadValue(c)),
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

pub fn to_result(opt: Option<&u8>) -> Result<u8, DecodeErr> {
     match opt {
         Some(&c) => Ok(c),
         None => return Err(DecodeErr::NotMultFour),
     }
}

// returns the number of bytes written or an error
pub fn decode<T : ByteWriter>(bytes: &[u8], writer: &mut T) -> Result<usize, DecodeErr> {

    let is_whitespace = |c| match c as char {
        '\n' => true,
        '\r' => true,
        '\t' => true,
        ' ' => true,
        _ => false,
    };

    let mut iter = bytes.iter().filter(|&&c| !is_whitespace(c));
    let mut count = 0;

    loop {

        let c1 = match iter.next() {
            Some(&c) => c,
            None => return Ok(count), // success! we reached the end of input on a multiple of 4
        };

        let c2 = try!(to_result(iter.next()));
        let c3 = try!(to_result(iter.next()));
        let c4 = try!(to_result(iter.next()));

        match (c1, c2, c3, c4) {
            (a, b, b'=', b'=') => {
                let v1 = try!(get_value(a));
                let v2 = try!(get_value(b));
                writer.write(get_first_byte(v1, v2));
                count += 1;
                return match iter.next() { //  must be end of input
                    Some(&x) => Err(DecodeErr::BadEndChar(x)),
                    None => Ok(count),
                };
            },
            (a, b, c, b'=') => {
                let v1 = try!(get_value(a));
                let v2 = try!(get_value(b));
                let v3 = try!(get_value(c));
                writer.write(get_first_byte(v1, v2));
                writer.write(get_second_byte(v2, v3));
                count += 2;
                return match iter.next() { //  must be end of input
                    Some(&x) => Err(DecodeErr::BadEndChar(x)),
                    None => Ok(count),
                };
            },
            (a, b, c, d) => {
                let v1 = try!(get_value(a));
                let v2 = try!(get_value(b));
                let v3 = try!(get_value(c));
                let v4 = try!(get_value(d));
                writer.write(get_first_byte(v1, v2));
                writer.write(get_second_byte(v2, v3));
                writer.write(get_third_byte(v3, v4));
                count += 3;
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
    decode(bytes, &mut vec).map(|_| vec)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_success(input: &[u8], output: &[u8])
    {
        let mut vec : Vec<u8> = Vec::new();
        let result = decode(input, &mut vec);
        assert_eq!(Ok(output.len()), result);
        assert_eq!(&vec[..], output);
    }

    fn test_failure(input: &[u8], err: DecodeErr)
    {
        let mut vec : Vec<u8> = Vec::new();
        let result = decode(input, &mut vec);
        assert_eq!(Err(err), result);
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
