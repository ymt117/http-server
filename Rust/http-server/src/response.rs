use super::parser;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

// /root以下のファイルのみアクセスを許可できるように
// リクエストされたパスを整形する
fn get_path(path: String) -> String {
    let root = "root";
    if path == "/" {
        return format!("{}{}", root, "/index.html");
    };

    return format!("{}{}", root, path)
}

pub fn build_response(mut path: String) -> (String, String, Vec<u8>) {
	let mut status_line = "HTTP/1.1 200 OK\r\n";

    path = get_path(path);
    println!("path: {}", path);

	let ext = path.split(".").last().unwrap_or("");// extention:拡張子
	let mut content_type = parser::ContentType::from_file_ext(ext);
	//println!("path: {}", path);
	//println!("content type: {:?}", content_type);
	
	let file = File::open(path);
	let mut file = match file {
		Ok(file) => file,
		Err(ref e) if e.kind() == ErrorKind::NotFound => {
			// 404 Errorの処理を書く
			status_line = "HTTP/1.1 404 NOT FOUND\r\n";
			content_type = parser::ContentType::from_file_ext("html");
			File::open(get_path("/404.html".to_string())).unwrap()
		}
		Err(e) => {
			panic!("There was a ploblem opening the file: {:?}", e)
		},
    };

	let mut body = Vec::new();
	file.read_to_end(&mut body).unwrap();

	let header = format!("Content-Type: {}\r\n\r\n", content_type.value());

	( status_line.to_string(),
	  header,
	  body
	)
}