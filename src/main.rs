use std::fs::{self};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listerner.incoming(){
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream){
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let content = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",content.len(), content);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("error.html").unwrap();
        let response = format!("{}\r\nContent-Length:{}\r\n\r\n{}",status_line, content.len(),content);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap(); 

    }

}
