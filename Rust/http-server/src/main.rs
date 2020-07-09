mod parser;
mod response;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::io;
use std::io::Read;

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 1024];
	stream.read(&mut buf).unwrap();

	let req = parser::parse(&buf);

	if req.method == "GET" {
		let (res, contents) = response::build_response(req.path);

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
		//let stream = stream.unwrap();
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