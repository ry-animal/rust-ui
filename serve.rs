use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Server running on http://127.0.0.1:9001");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let path = if request_line.starts_with("GET / ") {
        "dist/index.html".to_string()
    } else if request_line.starts_with("GET /") {
        let path_part = request_line.split_whitespace().nth(1).unwrap_or("/");
        let path_part = &path_part[1..]; // Remove leading /
        format!("dist/{}", path_part)
    } else {
        "dist/index.html".to_string()
    };

    let (status_line, contents) = if Path::new(&path).exists() {
        let contents = fs::read(&path).unwrap_or_default();
        ("HTTP/1.1 200 OK", contents)
    } else {
        let contents = fs::read("dist/index.html").unwrap_or_default();
        ("HTTP/1.1 200 OK", contents)
    };

    let content_type = if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".css") {
        "text/css"
    } else {
        "application/octet-stream"
    };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        content_type,
        contents.len()
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
    stream.flush().unwrap();
}