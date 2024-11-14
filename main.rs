use std::net:: TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
fn main() {
    let listener: TcpListener = 
    TcpListener::bind(addr: "127.0.0.1:7878").unwrap();
    for stream: Result<TcpStream, Error> in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8;1024] = [0; 1024];

    stream.read(buf: &mut buffer).unwrap();
    let get: &[u8;16] = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(needle: get) {
        let contents = fs::read_to_string(path:"index.html");
        let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
        contents.len,
        contents
    );
    
    stream.write(buf: response.as_bytes()).unwrap();
    stream.flush().unwrap;
}


    
    
    
    } else {
        let status_line: &str = "HTTP/1.1 404 NOT FOUND";
        LET CONTENTS: String = 
        fs:: read_to_string(path:"404.html").unwrap();
        let response: String =  format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(buf: response.as_bytes()).unwrap();
    stream.flush().unwrap;
    }
    
    
    
    
   