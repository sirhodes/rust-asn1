extern crate asn1;

use asn1::base64;
use std::io;
use std::str;

fn main() {
    let mut line = String::new();
    let mut output : Vec<u8> = Vec::new();

    io::stdin().read_line(&mut line).unwrap();

    match base64::decode(&line.trim_right()[..].as_bytes(), &mut output) {
        None => {
            println!("bytes: {:?}", output);
            match str::from_utf8(&output[..]) {
                Ok(str) => println!("{}", str),
                Err(err) => println!("Not UTF8: {}", err),
            }
        }
        Some(error) => println!("error: {:?}", error),
    }    
}
