use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:8014").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler_connection(stream);
    }
}

// Request : GET / HTTP/1.1
// Host: localhost:8014
// Connection: keep-alive
// Cache-Control: max-age=0
// sec-ch-ua: " Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101"
// sec-ch-ua-mobile: ?0
// sec-ch-ua-platform: "Windows"
// Upgrade-Insecure-Requests: 1
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
// Sec-Fetch-Site: none
// Sec-Fetch-Mode: navigate
// Sec-Fetch-User: ?1
// Sec-Fetch-Dest: document
// Accept-Encoding: gzip, deflate, br
// Accept-Language: zh-CN,zh;q=0.9
fn handler_connection(mut s: TcpStream) {
    let mut buffer = [0; 1024];
    s.read(&mut buffer).unwrap();
    //字符串前面加b，会把字符串转换为字节字符串
    let get = b"GET / HTTP/1.1";
    let get_sleep = b"GET /sleep HTTP/1.1";


    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    }else if  buffer.starts_with(get_sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let file = fs::read_to_string(file_name).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, file.len(), file);
    s.write(response.as_bytes()).unwrap();
    s.flush().unwrap();
}