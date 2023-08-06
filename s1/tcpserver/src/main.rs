use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
                stream.write(&mut buffer).unwrap();
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
