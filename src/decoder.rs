use length::{LengthError};
use tag::{TypeId};

pub enum Token<'a> {
    ObjectIdentifier{ bytes: &'a[u8] },
}

pub enum DecodeError {
    Len(LengthError),
    UnknownType(TypeId),
}

pub struct Parser<'a> {
    bytes: &'a[u8],
    pos: usize,
    // todo: there will be some kind of state
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a[u8]) -> Parser {
        Parser {
            bytes: input,
            pos: 0,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Token<'a>, DecodeError>;
    fn next(&mut self) -> Option<Result<Token<'a>, DecodeError>> {
        None
    }
}
