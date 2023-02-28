use std::{ fs, str::from_utf8_unchecked};

fn encode() -> &'static [u8] {
    let _type = 1;
    let text = "hello world!";
    let bytes : &[u8] = text.as_bytes();

    println!("{:?}", bytes);
    
    bytes
}

fn decode(bytes: &[u8]) {
    let binding = bytes.to_vec();
    let text = String::from_utf8_lossy(&binding);

    println!("{}", text);
}

fn main() {
    let _bytes = encode();
    decode(_bytes);
}
