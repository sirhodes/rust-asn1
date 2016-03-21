

#[derive(Debug, PartialEq)]
pub enum Class {
    Univeral,
    Application,
    ContextSpecific,
    Private
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    EndOfContent,
    Boolean,
    Integer,
    BitString,
    OctetString,
    Null,
    ObjectId,
    // ... define more
    Unknown(u8)
}

#[derive(Debug, PartialEq)]
pub struct TypeId {
    class: Class,
    is_constructed: bool,
    tag: Tag
}

const CLASS_MASK : u8 = 0b11000000;
const CONSTRUCTED_MASK: u8 = 0b00100000;
const TAG_MASK: u8 = 0b00011111;

const UNIVERAL_VALUE : u8 = 0;
const APPLICATION_VALUE : u8 = 0b01000000;
const CONTEXT_SPECIFIC_VALUE: u8 = 0b10000000;

impl TypeId {

    pub fn from_byte(id: u8) -> TypeId {
        TypeId {
            class: TypeId::get_class(id),
            is_constructed: TypeId::is_constructed(id),
            tag: TypeId::get_tag(id),
        }
    }

    fn get_class(id: u8) -> Class {
        match id & CLASS_MASK {
            UNIVERAL_VALUE => Class::Univeral,
            APPLICATION_VALUE => Class::Application,
            CONTEXT_SPECIFIC_VALUE => Class::ContextSpecific,
            _ => Class::Private
        }
    }

    fn is_constructed(id: u8) -> bool {
        (id & CONSTRUCTED_MASK) != 0
    }

    fn get_tag(id: u8) -> Tag {
        match id & TAG_MASK {
            0 => Tag::EndOfContent,
            1 => Tag::Boolean,
            2 => Tag::Integer,
            3 => Tag::BitString,
            4 => Tag::OctetString,
            5 => Tag::Null,
            6 => Tag::ObjectId,
            x => Tag::Unknown(x)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn correctly_parses_typeid() {

        let expected = TypeId {
            class: Class::Univeral,
            is_constructed: false,
            tag: Tag::Integer,
        };

        assert_eq!(expected, TypeId::from_byte(2));
    }
}
