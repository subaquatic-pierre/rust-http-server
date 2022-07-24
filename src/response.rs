pub struct Response {
    status_code: u16,
    content: String,
}

impl Response {
    pub fn new(status_code: u16, content: &str) -> Self {
        Response {
            status_code,
            content: content.to_string(),
        }
    }

    pub fn format(&self) -> String {
        let response = format!(
            "HTTP/1.1 {status_code} OK\r\nContent-Length: {content_len}\r\n\r\n{content}",
            status_code = self.status_code,
            content_len = self.content.len(),
            content = self.content
        );
        response
    }
}
