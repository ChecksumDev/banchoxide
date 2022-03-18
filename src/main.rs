use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = std::fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", index);
    loop {
        let listener = TcpListener::bind("127.0.0.1:80").unwrap();
        for stream in listener.incoming() {
            let mut buffer = [0; 512];
            let mut stream = stream.unwrap();
            stream.read(&mut buffer).unwrap();
            //println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
            stream.write(response.as_bytes()).unwrap();
        }
    }
}
