use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    for stream in listener.incoming() {
       let mut stream = stream.unwrap();
       let mut buffer = [0; 1024];
       stream.read(&mut buffer).unwrap();

       stream.write(String::from_utf8_lossy(&buffer[..]).as_bytes());
    }
}

#[test]
fn test() {
    let str = "Hello, world!";
    assert_eq!("Hello, world!", str);
}