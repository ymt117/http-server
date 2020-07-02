#[derive(Debug)]
pub enum ContentType {
	CSS,
	GIF,
	HTML,
	JPEG,
	PNG,
	SVG,
	TEXT,
	XML,
	ICO,
}

impl ContentType {
	pub fn from_file_ext(ext: &str) -> ContentType {
		match ext {
			"css" => ContentType::CSS,
			"gif" => ContentType::GIF,
			"htm" => ContentType::HTML,
			"html" => ContentType::HTML,
			"jpeg" => ContentType::JPEG,
			"jpg" => ContentType::JPEG,
			"png" => ContentType::PNG,
			"svg" => ContentType::SVG,
			"txt" => ContentType::TEXT,
			"xml" => ContentType::XML,
			"ico" => ContentType::ICO,
			_ => ContentType::TEXT,
		}
	}

	pub fn value(&self) -> &str {
		match *self {
			ContentType::CSS => "text/css",
			ContentType::GIF => "image/gif",
			ContentType::HTML => "text/html",
			ContentType::JPEG => "image/jpeg",
			ContentType::PNG => "image/png",
			ContentType::SVG => "image/svg+xml",
			ContentType::TEXT => "text/plain",
			ContentType::XML => "application/xml",
			ContentType::ICO => "image/ico",
		}
	}
}

#[derive(Debug)]
pub struct Request {
	pub method: String,
	pub path: String,
	http_version: String,
}

pub fn parse(buf: &[u8]) -> Request{
	// 受け取ったリクエストを表示する
	//println!("Request: {}", String::from_utf8_lossy(&buf[..]));

	let req = split_request(&buf);
	//println!("{:#?}", req);

	let request = Request {
		method: req[0].clone(),
		path: req[1].clone(),
		http_version: req[2].clone(),
	};

	return request;

	//println!("{:#?}",split_whitespace(&buf));
}

fn split_request(bytes: &[u8]) -> Vec<String> {
	let mut req: Vec<String> = Vec::new();
	let mut first_point: usize = 0;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' || item == b':' {
			req.push(String::from_utf8(bytes[first_point..i].to_vec()).unwrap());
			first_point = i + 1;
		}

		if item == b'\r' || item == b'\n' {
			// first_pointを変更する処理
			first_point = i + 1;
		}
	}
	//println!("Vector {:#?}", req);
	req
}

#[test]
fn test_parse() {
	let buf: &[u8] = "GET / HTTP/1.1\r\n".as_bytes();
	let status_line = parse(&buf);

	assert_eq!(status_line.method, "GET");
	assert_eq!(status_line.path, "/");
	assert_eq!(status_line.http_version, "HTTP/1.1");
}