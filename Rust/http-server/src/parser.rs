#[derive(Debug)]
pub struct Request {
	pub method: String,
	pub request_uri: String,
	http_version: String,
}

pub fn parse(buf: &[u8]) -> Request{
	// 受け取ったリクエストを表示する
	//println!("Request: {}", String::from_utf8_lossy(&buf[..]));

	let req = split_whitespace(&buf);

	let request = Request {
		method: req[0].clone(),
		request_uri: req[1].clone(),
		http_version: req[2].clone(),
	};

	return request;

	//println!("{:#?}", status_line);
	//println!("{:#?}",split_whitespace(&buf));
}

fn split_whitespace(bytes: &[u8]) -> Vec<String> {
	let mut req: Vec<String> = Vec::new();
	let mut first_point: usize = 0;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' || item == b'\r' || item == b'\n' {
			req.push(String::from_utf8(bytes[first_point..i].to_vec()).unwrap());
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
	assert_eq!(status_line.request_uri, "/");
	assert_eq!(status_line.http_version, "HTTP/1.1");
}