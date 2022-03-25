use std::{net::{TcpListener, TcpStream}};
use std::io::prelude::*;
use std::thread;
use chrono;


fn main() {
    let listener = match TcpListener::bind("127.0.0.1:2345") {
        Ok(listener) => listener,
        Err(e) => {
            panic!("create listener failed: {}", e)
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("connection failed {}", e);
            }
        }

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (response, request) = handle_req(buffer);
    let date = chrono::Local::now().format("%F %T").to_string();

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("{} {}", date, request),
        Err(_) => println!("{} 408", date),
    }

    match stream.flush() {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
}

fn handle_req(buf: [u8; 1024]) -> (String, String) {
    let get = b"GET / HTTP/1.1\r\n";
    let head = b"HEAD / HTTP/1.1\r\n";

    let (response, request) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\nServer: dumb-http\r\n\r\n<html><body>Up!</body></html>\r\n".to_string(), "GET /".to_string())
    } else if buf.starts_with(head) {
        ("HTTP/1.1 200 OK\r\nServer: dumb-http\r\n\r\n".to_string(), "HEAD /".to_string())
    } else {
        ("HTTP/1.1 400 BAD REQUEST\r\nServer: dumb-http\r\n\r\n".to_string(), "".to_string())
    };

    (response, request)
}
