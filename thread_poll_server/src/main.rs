mod lib;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use lib::ThreadPool;


fn main() {
    let server = TcpListener::bind("localhost:8081").unwrap();

    let thread_poll = ThreadPool::new(4).unwrap();
    for stream in server.incoming() {
        let stream = stream.unwrap();
        thread_poll.exec(|| {
            handler_connection(stream);
        });
    }
}

fn handler_connection(mut s: TcpStream) {
    let mut buffer = [0; 1024];
    s.read(&mut buffer).unwrap();
    //字符串前面加b，会把字符串转换为字节字符串
    let get = b"GET / HTTP/1.1";
    let get_sleep = b"GET /sleep HTTP/1.1";


    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(get_sleep) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let file = fs::read_to_string(file_name).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, file.len(), file);
    s.write(response.as_bytes()).unwrap();
    s.flush().unwrap();
}