use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    for stream in listener.incoming() {
       println!("connection established");
    }
}

#[test]
fn test() {
    let str = "Hello, world!";
    assert_eq!("Hello, world!", str);
}