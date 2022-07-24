use std::net::TcpStream;

use std::io::Read;

pub struct Request {
    content: String,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Self {
        Request {
            content: parse(stream),
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn path(&self) -> String {
        let mut first_line = String::new();
        let mut lines = self.content.lines().into_iter();

        if let Some(line) = lines.next() {
            first_line = line.to_string();
        }

        let path: Vec<&str> = first_line.split_whitespace().collect();
        path[1].to_string()
    }
}

fn parse(stream: &TcpStream) -> String {
    let mut buffer = [0; 1024];

    let mut stream_clone = stream.clone();

    stream_clone.read(&mut buffer).unwrap();

    String::from_utf8_lossy(&buffer).to_string()
}
