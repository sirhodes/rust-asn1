
const CODES : [char; 64] = [
    'A','B','C','D','E','F','G','H',
    'I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X',
    'Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n',
    'o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3',
    '4','5','6','7','8','9','+','/'];

const TOP2_BITS_MASK : u8 = 0b11000000;
const TOP4_BITS_MASK : u8 = 0b11110000;
const TOP6_BITS_MASK : u8 = 0b11111100;

const BOTTOM2_BITS_MASK: u8 = !TOP6_BITS_MASK;
const BOTTOM4_BITS_MASK : u8 = !TOP4_BITS_MASK;
const BOTTOM6_BITS_MASK : u8 = !TOP2_BITS_MASK;

pub trait CharWriter {
    fn write(self: &mut Self, c : char);
}

pub fn encode<T : CharWriter>(bytes: &[u8], writer: &mut T) -> () {

    let mut pos : usize = 0;

    while pos < bytes.len() {
        let remainder = bytes.len() - pos;
        let cursor = &bytes[pos ..];
        match remainder {
            1 => {
                writer.write(get_first_char(cursor[0]));
                writer.write(get_second_char(cursor[0], 0));
                writer.write('=');
                writer.write('=');
            },
            2 => {
                writer.write(get_first_char(cursor[0]));
                writer.write(get_second_char(cursor[0], cursor[1]));
                writer.write(get_third_char(cursor[1], 0));
                writer.write('=');
            }
            _ => {  // 3 or more
                writer.write(get_first_char(cursor[0]));
                writer.write(get_second_char(cursor[0], cursor[1]));
                writer.write(get_third_char(cursor[1], cursor[2]));
                writer.write(get_fourth_char(cursor[2]));
            },
        }
        pos += 3;
    }
}

impl CharWriter for String {
    fn write(self: &mut Self, c : char) {
        self.push(c);
    }
}

pub fn encode_as_string(bytes: &[u8]) -> String {
    let mut value = String::with_capacity((bytes.len()*4)/3);
    encode::<String>(bytes, &mut value);
    value
}

fn get_first_char(first: u8) -> char {
    CODES[((first & TOP6_BITS_MASK) >> 2) as usize]
}

fn get_second_char(first: u8, second: u8) -> char {
    let index = ((first & BOTTOM2_BITS_MASK) << 4) | ((second & TOP4_BITS_MASK) >> 4);
    CODES[index as usize]
}

fn get_third_char(second: u8, third: u8) -> char {
    let index = ((second & BOTTOM4_BITS_MASK) << 2) | ((third & TOP2_BITS_MASK) >> 6);
    CODES[index as usize]
}

fn get_fourth_char(third: u8) -> char {
    CODES[(third & BOTTOM6_BITS_MASK) as usize]
}

#[test]
fn correctly_encodes_empty_array() {
    let bytes = b"";
    assert_eq!(encode_as_string(&bytes[..]), "");
}

#[test]
fn correctly_encodes_even_multiple_of_three() {
    let bytes = b"ManMan";
    assert_eq!(encode_as_string(&bytes[..]), "TWFuTWFu");
}

#[test]
fn correctly_encodes_modulo_one() {
    let bytes = b"ManM";
    assert_eq!(encode_as_string(&bytes[..]), "TWFuTQ==");
}

#[test]
fn correctly_encodes_modulo_two() {
    let bytes = b"ManMa";
    assert_eq!(encode_as_string(&bytes[..]), "TWFuTWE=");
}