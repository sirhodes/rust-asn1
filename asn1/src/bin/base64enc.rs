extern crate asn1;

use asn1::base64;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", base64::encode_as_string(&line.as_bytes()[..]));
}
