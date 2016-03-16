

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

enum DecodeState {
    HaltBadValue,
    Continue,
    Done,
}

fn decode_four_char<T : ByteWriter>(set: &(u8, u8, u8, u8), writer: &mut T) -> DecodeState {
    match *set {
        (a, b, b'=', b'=') => decode_single_byte(a, b, writer),
        (a, b, c, b'=') => decode_two_bytes(a, b, c, writer),
        (a, b, c, d) => decode_three_bytes(a, b, c, d, writer),
    }
}

fn decode_single_byte<T : ByteWriter>(c1: u8, c2: u8, writer: &mut T) -> DecodeState {
    match (get_value(c1), get_value(c2)) {
        (Some(b1), Some(b2)) => {
            writer.write(get_first_byte(b1, b2));
            DecodeState::Done
        },
        _ => DecodeState::HaltBadValue,
    }
}

fn decode_two_bytes<T : ByteWriter>(c1: u8, c2: u8, c3: u8, writer: &mut T) -> DecodeState {
    match (get_value(c1), get_value(c2), get_value(c3)) {
        (Some(b1), Some(b2), Some(b3)) => {
            writer.write(get_first_byte(b1, b2));
            writer.write(get_second_byte(b2, b3));
            DecodeState::Done
        },
        _ => DecodeState::HaltBadValue,
    }
}

fn decode_three_bytes<T : ByteWriter>(c1: u8, c2: u8, c3: u8, c4: u8, writer: &mut T) -> DecodeState {
    match (get_value(c1), get_value(c2), get_value(c3), get_value(c4)) {
        (Some(b1), Some(b2), Some(b3), Some(b4)) => {
            writer.write(get_first_byte(b1, b2));
            writer.write(get_second_byte(b2, b3));
            writer.write(get_third_byte(b3, b4));
            DecodeState::Continue
        },
        _ => DecodeState::HaltBadValue,
    }
}

fn get_first_byte(b1: u8, b2: u8) -> u8 {
    ((b1 & 0b00111111) << 2) | ((b2 & 0b00110000) >> 4)
}

fn get_second_byte(b2: u8, b3: u8) -> u8 {
    println!("Second byte! {} {}", b2, b3);
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
    BadValue,
    BadEndChar
}

// returns the number of bytes written or an error
pub fn decode<T : ByteWriter>(c: &[u8], writer: &mut T) -> Option<DecodeErr> {

    if c.len() % 4 != 0 {
        return Some(DecodeErr::NotMultFour);
    }

    let mut pos = 0;

    while pos < c.len() {
        let cursor = &c[pos ..];
        let set = (cursor[0], cursor[1], cursor[2], cursor[3]);
        match decode_four_char(&set, writer) {
            DecodeState::HaltBadValue => return Some(DecodeErr::BadValue),
            DecodeState::Continue => {
                pos += 4;
            },
            DecodeState::Done => {
                pos += 4;
                break;
            },
        }
    }

    if c.len() - pos == 0 {
        None
    } else {
        Some(DecodeErr::BadEndChar)
    }
}

impl ByteWriter for Vec<u8> {
    fn write(self: &mut Self, b: u8) {
        self.push(b);
    }
}

#[test]
fn returns_error_on_bad_size() {
    let mut vec : Vec<u8> = Vec::new();
    let result = decode(&b"TQ="[..], &mut vec);
    assert_eq!(Some(DecodeErr::NotMultFour), result);
}

#[test]
fn correctly_decodes_one_byte() {
    let mut vec : Vec<u8> = Vec::new();
    let result = decode(&b"TQ=="[..], &mut vec);
    assert_eq!(None, result);
    assert_eq!(&vec[..], [77]);
}

#[test]
fn correctly_decodes_two_bytes() {
    let mut vec : Vec<u8> = Vec::new();    
    let result = decode::<Vec<u8>>(&b"TWE="[..], &mut vec);
    assert_eq!(None, result);
    assert_eq!(&vec[..], [77,97]);
}

#[test]
fn correctly_decodes_three_bytes() {
    let mut vec : Vec<u8> = Vec::new();
    //let input = b"TWFu";
    let result = decode::<Vec<u8>>(&b"TWFu"[..], &mut vec);
    assert_eq!(None, result);
    assert_eq!(&vec[..], [77,97,110]);
}

#[test]
fn correctly_decodes_six_bytes() {
    let mut vec : Vec<u8> = Vec::new();
    let result = decode::<Vec<u8>>(&b"TWFuTQ=="[..], &mut vec);
    assert_eq!(None, result);
    assert_eq!(&vec[..], [77,97,110,77]);
}

#[test]
fn rejects_trailing_bytes() {
    let mut vec : Vec<u8> = Vec::new();
    let result = decode::<Vec<u8>>(&b"TQ==TWFu"[..], &mut vec);
    assert_eq!(Some(DecodeErr::BadEndChar), result);
}
