use length::{Length, LengthError, read_len};
use tag::{Class, TypeId, Tag};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    ObjectIdentifier(&'a[u8]),
    NoMoreTokens
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Len(LengthError),
    UnknownType(TypeId),
    BadLength(Length)
}

impl From<LengthError> for DecodeError {
    fn from(err: LengthError) -> DecodeError {
        DecodeError::Len(err)
    }
}

pub struct Parser<'a> {
    bytes: &'a[u8],
    pos: usize,
}

impl<'a> Parser<'a> {

    pub fn new(input: &'a[u8]) -> Parser {
        Parser {
            bytes: input,
            pos: 0,
        }
    }

    fn remainder(&self) -> usize {
        self.bytes.len() - self.pos
    }

    fn next(&mut self) -> Result<Token<'a>, DecodeError> {

        if self.remainder() == 0 {
            return Ok(Token::NoMoreTokens);
        }

        // read the typeid and the length
        let id = TypeId::from_byte(self.bytes[self.pos]);
        self.pos += 1;
        let (num, len) = try!(read_len(&self.bytes[self.pos..]));
        self.pos += num;

        match id {
            TypeId { class: Class::Univeral, is_constructed: false, tag: Tag::ObjectId } => {
                self.decode_object_id(len)
            },
            x => {
                Err(DecodeError::UnknownType(x))
            },
        }
    }

    fn decode_object_id(&mut self, len: Length) -> Result<Token<'a>, DecodeError> {
        let remainder = self.remainder();
        match len {
            Length::None => {
                Ok(Token::ObjectIdentifier(&self.bytes[0..0]))
            },
            Length::Value(x) => {
                if x > remainder  {
                    Err(DecodeError::BadLength(len))
                }
                else {
                    Ok(Token::ObjectIdentifier(&self.bytes[self.pos..self.pos+x]))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn accepts_empty_input() {
        let input = b"";
        let mut parser = Parser::new(input);
        assert_eq!(Ok(Token::NoMoreTokens), parser.next());
    }
}
