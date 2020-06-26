use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;
use std::fs::File;

fn parse(buf: &[u8]) -> String{

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")        
    }
    // 404を返す
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("{}{}", status_line, contents)
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();

        thread::spawn(move || {
            let response = parse(&buf);

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }
}

#[test]
fn test_parse() {
    
}
