mod parser;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::fs::File;



fn response(path: String) -> String{
	let status_line = "HTTP/1.1 200 OK\r\n\r\n";
	
	let mut file;
	match &*path {
		"/" => file = File::open("index.html").unwrap(),
		_ => file = File::open("404.html").unwrap(),
	}

	let mut contents = String::new();
	
	file.read_to_string(&mut contents).unwrap();

	return format!("{}{}", status_line, contents);
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 1024];

	stream.read(&mut buf).unwrap();

	let req = parser::parse(&buf);

	if req.method == "GET" {
		let response = response(req.request_uri);

		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}
}

fn main() {
	// ポートに接続する
	let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

	// ストリームを受け取る
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		thread::spawn(|| {
			handle_connection(stream);
		});
	}
}