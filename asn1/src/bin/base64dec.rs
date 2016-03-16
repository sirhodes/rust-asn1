extern crate asn1;

use asn1::base64;
use std::io;

fn main() {
    let mut line = String::new();
    let mut output : Vec<u8> = Vec::new();

    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            match base64::decode(&line[..].as_bytes(), &mut output) {
                None => println!("success!"),
                Some(error) => println!("error: {:?}", error),
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
