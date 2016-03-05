

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

impl TypeId {

    pub fn from(id: u8) -> TypeId {
        TypeId {
            class: TypeId::get_class(id),
            is_constructed: TypeId::is_constructed(id),
            tag: TypeId::get_tag(id),
        }
    }

    fn get_class(id: u8) -> Class {
        match id & 0b11000000 {
            0 => Class::Univeral,
            0b01000000 => Class::Application,
            0b10000000 => Class::ContextSpecific,
            _ => Class::Private
        }
    }

    fn is_constructed(id: u8) -> bool {
        (id & 0b00100000) != 0
    }

    fn get_tag(id: u8) -> Tag {
        match id & 0b00011111 {
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


#[test]
fn correctly_parses_typeid() {

    let expected = TypeId {
        class: Class::Univeral,
        is_constructed: false,
        tag: Tag::Integer,
    };

    assert_eq!(expected, TypeId::from(2));
}
