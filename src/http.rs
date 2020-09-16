use std::io::prelude::*;
use std::net::TcpStream;

pub struct HttpRequest {
    pub method: String,
    pub target: String,
}

impl HttpRequest {
    pub fn from_stream(&stream: &TcpStream) -> HttpRequest {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let string = String::from_utf8_lossy(&buffer[..]);
        println!("{}", string);

        Self::from_string(&string)
    }

    fn from_string(text: &str) -> HttpRequest {
        let lines = text.split("\r\n");
        let request_line = lines.nth(0);

        let request_line_parts = request_line.unwrap().split_whitespace();
        let method = request_line_parts.nth(0).unwrap().clone().to_string();
        let target = request_line_parts.nth(1).unwrap().clone().to_string();

        HttpRequest {
            method,
            target,
        } }
}

pub struct HttpResponse {
    pub content_type: String,
    pub content_length: usize,
    pub body: String,
}

impl HttpResponse {
    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: {}\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
            self.content_type,
            self.content_length,
            self.body,
        )
    }
}
