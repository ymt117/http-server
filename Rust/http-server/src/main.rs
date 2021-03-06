mod parser;
mod response;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::io;
use std::io::Read;

fn handle_connection(mut stream: TcpStream) {
	const BUFFER_SIZE: usize = 1024;
	let mut buf = [0; BUFFER_SIZE];
	stream.read(&mut buf).unwrap();

	let req = parser::parse(&buf);

	if req.method == "GET" {
		let (status_line, header, body) = response::build_response(req.path);

		stream.write(status_line.as_bytes()).unwrap();
		stream.write(header.as_bytes()).unwrap();
		stream.write(&body).unwrap();
		stream.flush().unwrap();
	}
}

fn main() -> io::Result<()> {
	// ポートに接続する
	let listener = TcpListener::bind("0.0.0.0:12345")?;
	println!("Server started!");

	// ストリームを受け取る
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(|| {
					handle_connection(stream);
				});
			}
			Err(e) => println!("Error: {}", e),
		}

	}

	Ok(())
}