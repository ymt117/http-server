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
	println!("Request: {}", String::from_utf8_lossy(&buf[..]));

	// リクエストを１行ずつ取り出す
	let req_line = read_line(&buf);

	// 開始行からメソッド、リクエスト対象（パス）、HTTPバージョンを取り出す
	// ex) GET / HTTP/1.1
	let start_line = split_request(req_line[0].as_bytes());

	// ヘッダー行からHTTPヘッダーを取り出す
	// ヘッダー行と本文の境目は改行文字「\r\n」だけの行で判断する
	// ここに処理を書く

	let request = Request {
		method: start_line[0].clone(),
		path: start_line[1].clone(),
		http_version: start_line[2].clone(),
	};

	return request;
}

fn read_line(bytes: &[u8]) -> Vec<String> {
	let mut req: Vec<String> = Vec::new();
	let mut sp: usize = 0; // sp: starting point

	for (i, &item) in bytes.iter().enumerate() {
		if item == b'\n' {
			req.push(String::from_utf8(bytes[sp..i+1].to_vec()).unwrap());
			sp = i + 1;
		}
	}
	//println!("{:#?}", req);
	req
}

fn split_request(bytes: &[u8]) -> Vec<String> {
	let mut req: Vec<String> = Vec::new();
	let mut first_point: usize = 0;

	for (i, &item) in bytes.iter().enumerate() {
		// 半角スペース「 」またはコロン「:」、改行文字「\r」で区切る
		if item == b' ' || item == b':' || item == b'\r'{
			req.push(String::from_utf8(bytes[first_point..i].to_vec()).unwrap());
			first_point = i + 1;
		}
	}
	//println!("Vector {:#?}", req);
	req
}


#[test]
fn test_split_request() {
	let buf: &[u8] = "GET / HTTP/1.1\r\n".as_bytes();
	let req = split_request(buf);

	assert_eq!(req[0], "GET");
	assert_eq!(req[1], "/");
	assert_eq!(req[2], "HTTP/1.1");
}

#[test]
fn test_parse() {
	let buf: &[u8] = "GET / HTTP/1.1\r\n".as_bytes();
	let status_line = parse(&buf);

	assert_eq!(status_line.method, "GET");
	assert_eq!(status_line.path, "/");
	assert_eq!(status_line.http_version, "HTTP/1.1");
}

#[test]
fn test_value_jpg() {
	let content_type = ContentType::from_file_ext("jpg");
	assert_eq!(content_type.value(), "image/jpeg");

	let content_type = ContentType::from_file_ext("jpeg");
	assert_eq!(content_type.value(), "image/jpeg");
}

#[test]
fn test_value_html() {
	let content_type = ContentType::from_file_ext("html");
	assert_eq!(content_type.value(), "text/html");
}
