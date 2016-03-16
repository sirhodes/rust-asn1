extern crate asn1;

use asn1::base64;
use std::io;

fn main() {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            println!("{}", base64::encode_as_string(&line.as_bytes()[..]));
        }
        Err(error) => println!("error: {}", error),
    }
}
