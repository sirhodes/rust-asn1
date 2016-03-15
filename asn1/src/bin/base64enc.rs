extern crate asn1;

use asn1::base64;
use std::io;
use std::io::Read;

fn main() {
    let mut vec : Vec<u8> = Vec::new();
    match io::stdin().read_to_end(&mut vec) {
        Ok(_) => {            
            println!("{}", base64::encode_as_string(&vec[..]));
        }
        Err(error) => println!("error: {}", error),
    }
}
