mod parser;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::fs::File;
use std::io;

/*
HTTP/1.1 200 OK\r\n
Content-Type: image/jpeg\r\n
\r\n
Body
 */

fn response(path: String) -> String{
	let status_line = "HTTP/1.1 200 OK\r\nContent-Type: ";

	let ext = path.split(".").last().unwrap_or("");// extention:拡張子
	let content_type = parser::ContentType::from_file_ext(ext);
	println!("path: {}", path);
	println!("content type: {:?}", content_type);
	
	let mut file;
	match content_type {
		parser::ContentType::TEXT => file = File::open("index.html").unwrap(),
		parser::ContentType::JPEG => file = File::open(path).unwrap(),
		_ => file = File::open("404.html").unwrap(),
	}

	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return format!("{}Content-Type: {}\r\n\r\n{}", status_line, content_type.value(), contents);
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 1024];

	stream.read(&mut buf).unwrap();

	let req = parser::parse(&buf);

	if req.method == "GET" {
		let response = response(req.path);

		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}
}

fn main() -> io::Result<()> {
	// ポートに接続する
	let listener = TcpListener::bind("0.0.0.0:12345")?;
	println!("Server started!");

	// ストリームを受け取る
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		thread::spawn(|| {
			handle_connection(stream);
		});
	}

	Ok(())
}