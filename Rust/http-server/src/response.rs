mod parser;

use http::StatusCode;

enum ContentType {
	CSS,
	GIF,
	HTML,
	JPEG,
	PNG,
	SVG,
	TEXT,
	XML,
}

impl ContentType {
	fn from_file_ext(ext: &str) -> ContentType {
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
			_ => ContentType::TEXT,
		}
	}

	fn value(&self) -> &str {
		match *self {
			ContentType::CSS => "text/css",
			ContentType::GIF => "image/gif",
			ContentType::HTML => "text/html",
			ContentType::JPEG => "image/jpeg",
			ContentType::PNG => "image/png",
			ContentType::SVG => "image/svg+xml",
			ContentType::TEXT => "text/plain",
			ContentType::XML => "application/xml",
		}
	}
}

struct ResponseHeaders {
	content_type: Option<ContentType>,
}

impl ResponseHeaders {
	fn new() -> ResponseHeaders {
		ResponseHeaders {
			content_type: None,
		}
	}
}

struct Response {
	body: Option<Vec<u8>>,
	headers: ResponseHeaders,
	status: StatusCode,
}

impl Response {
	fn new() -> Response {
		Response {
			body: None,
			header: ResponseHeaders::new(),
			status: StatusCode::OK,
		}
	}
}

fn build_response(request: &Request) -> Response {
	let mut response = Response::new();
	if request.method != "GET" {
		response.status = StatusCode::METHOD_NOT_ALLOWED;
	}
	else {
		add_file_to_response(&request.path, &mut response);
	}

	response
}

fn add_file_to_response(path: &String, response: &mut Response) {
	let path = format!("{}", path);
	let contents = fs::read(&path);
	match contents {
		Ok(contents) => {
			response.body = Some(contnets);
			let ext = path.split(".").last().unwrap_or("");
			response.headers.content_type = Some(ContentType::from_file_ext(ext));
		},
		Err(e) => {
			response.status = match e.kind() {
				ErrorKind::NotFound => StatusCode::NOT_FOUND,
				ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
				_ => StatusCode::INTERNAL_SERVER_ERROR,
			}
		}
	}
}