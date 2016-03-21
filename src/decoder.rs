use length::{LengthErr, read_len};
use tag::{Class, TypeId, Tag};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    ObjectId(&'a[u8]),
    NoMoreTokens
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Len(LengthErr),
    UnknownType(TypeId),
    BadLength(usize)
}

impl From<LengthErr> for DecodeError {
    fn from(err: LengthErr) -> DecodeError {
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
        let (consumed, len) = try!(read_len(&self.bytes[self.pos..]));
        self.pos += consumed;

        match id {
            TypeId { class: Class::Univeral, is_constructed: false, tag: Tag::ObjectId } => {
                self.decode_object_id(len)
            },
            x => {
                Err(DecodeError::UnknownType(x))
            },
        }
    }

    fn decode_object_id(&mut self, len: usize) -> Result<Token<'a>, DecodeError> {
        let remainder = self.remainder();
        if len > remainder  {
            Err(DecodeError::BadLength(len))
        }
        else {
            let ret = Ok(Token::ObjectId(&self.bytes[self.pos..self.pos+len]));
            self.pos += len;
            ret
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

    #[test]
    fn accepts_valid_objectid() {
        let input : [u8; 6] = [0x06, 0x04, 0xDE, 0xAD, 0xBE, 0xEF];
        let mut parser = Parser::new(&input);
        assert_eq!(Ok(Token::ObjectId(&input[2..])), parser.next());
        assert_eq!(Ok(Token::NoMoreTokens), parser.next());
    }
}
