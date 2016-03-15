#[derive(Debug, PartialEq)]
pub enum CharResult {
    Line,
    Value(u8),
    Invalid
}

fn in_range(c: char, begin: char, end: char) -> bool {
    (c >= begin) && (c <= end)
}

//  this could be faster if backed by a 256-byte table
pub fn get_value(c: char) -> CharResult {

    match c {
        // ascii 65 -> 90, base64 0 -> 25
        x if in_range(x, 'A', 'Z') => CharResult::Value(c as u8 - 65),
        // ascii 97 - 122, base64 26 -> 51
        x if in_range(x, 'a', 'z') => CharResult::Value(c as u8 - 71),
        // ascii 48 - 57, base64 52 -> 61
        x if in_range(x, '0', '9') => CharResult::Value(c as u8 + 4),

        '+' => CharResult::Value(62),
        '/' => CharResult::Value(63),
        
        '\r' | '\n' => CharResult::Line,
        _ => CharResult::Invalid,
    }
}

#[test]
fn correct_gets_value_for_upper() {
    assert_eq!(CharResult::Value(1), get_value('B'));
}

#[test]
fn rejects_invalid_char() {
    assert_eq!(CharResult::Invalid, get_value(':'));
}
