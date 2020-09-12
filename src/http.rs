pub struct HttpResponse {
    pub content_type: String,
    pub content_length: usize,
    pub body: String,
}

impl HttpResponse {
    pub fn write_response(&self) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.content_type,
            self.content_length,
            self.body,
        )
    }
}
