extern crate asn1;

use asn1::base64;
use std::io;
use std::str;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let output = base64::decode_as_vec(&line.trim_right()[..].as_bytes()).unwrap();

    println!("bytes: {:?}", output);
    let utf8 = str::from_utf8(&output[..]).unwrap();
    println!("{}", utf8);
}
