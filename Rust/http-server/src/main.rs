mod parser;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn response(mut path: String) -> (String, Vec<u8>) {
	let mut status_line = "HTTP/1.1 200 OK\r\n";

	if path == "/" {
		path = "index.html".to_string();
	}

	let ext = path.split(".").last().unwrap_or("");// extention:拡張子
	let mut content_type = parser::ContentType::from_file_ext(ext);
	//println!("path: {}", path);
	//println!("content type: {:?}", content_type);
	
	path.retain(|c| c != '/');
	let file = File::open(path);
	let mut file = match file {
		Ok(file) => file,
		Err(ref e) if e.kind() == ErrorKind::NotFound => {
			// 404 Errorの処理を書く
			status_line = "HTTP/1.1 404 NOT FOUND\r\n";
			content_type = parser::ContentType::from_file_ext("html");
			File::open("404.html").unwrap()
		}
		Err(e) => {
			panic!("There was a ploblem opening the file: {:?}", e)
		},
	};

	let mut contents = Vec::new();
	file.read_to_end(&mut contents).unwrap();

	(format!("{}Content-Type: {}\r\n\r\n", status_line, content_type.value()), contents)
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 1024];
	stream.read(&mut buf).unwrap();

	let req = parser::parse(&buf);

	if req.method == "GET" {
		let (res, contents) = response(req.path);

		stream.write(res.as_bytes()).unwrap();
		stream.write(&contents).unwrap();
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