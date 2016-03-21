
const CODES : [char; 64] = [
    'A','B','C','D','E','F','G','H',
    'I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X',
    'Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n',
    'o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3',
    '4','5','6','7','8','9','+','/'];

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
    CODES[((first & 0b11111100) >> 2) as usize]
}

fn get_second_char(first: u8, second: u8) -> char {
    let index = ((first & 0b00000011) << 4) | ((second & 0b11110000) >> 4);
    CODES[index as usize]
}

fn get_third_char(second: u8, third: u8) -> char {
    let index = ((second & 0b00001111) << 2) | ((third & 0b11000000) >> 6);
    CODES[index as usize]
}

fn get_fourth_char(third: u8) -> char {
    CODES[(third & 0b00111111) as usize]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn correctly_encodes_empty_array() {
        assert_eq!(encode_as_string(b""), "");
    }

    #[test]
    fn correctly_encodes_even_multiple_of_three() {
        assert_eq!(encode_as_string(b"ManMan"), "TWFuTWFu");
    }

    #[test]
    fn correctly_encodes_modulo_one() {
        assert_eq!(encode_as_string(b"ManM"), "TWFuTQ==");
    }

    #[test]
    fn correctly_encodes_modulo_two() {
        assert_eq!(encode_as_string(b"ManMa"), "TWFuTWE=");
    }
}
