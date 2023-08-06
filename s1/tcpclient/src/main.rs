use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    stream.write("hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!("Respnse from server:{:?}", String::from_utf8_lossy(&buffer[..]));
}
