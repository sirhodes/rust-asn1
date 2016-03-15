
#[derive(Debug, PartialEq)]
enum CharResult {
    Line,
    Value(u8),
    Invalid
}

//  this would likely be faster with a 256-byte table, but
//  damn, the patteren matching is really slick
fn get_value(c: char) -> CharResult {

    match c {
        // ascii 65 -> 90, base64 0 -> 25
        'A' ... 'Z' => CharResult::Value(c as u8 - 65),
        // ascii 97 - 122, base64 26 -> 51
        'a' ... 'z' => CharResult::Value(c as u8 - 71),
        // ascii 48 - 57, base64 52 -> 61
        '0' ... '9' => CharResult::Value(c as u8 + 4),

        '+' => CharResult::Value(62),
        '/' => CharResult::Value(63),

        '\r' | '\n' => CharResult::Line,
        _ => CharResult::Invalid,
    }
}



#[test]
fn correct_gets_value_for_upper() {
    assert_eq!(CharResult::Value(0), get_value('A'));
    assert_eq!(CharResult::Value(25), get_value('Z'));
}

#[test]
fn rejects_invalid_char() {
    assert_eq!(CharResult::Invalid, get_value(':'));
}
