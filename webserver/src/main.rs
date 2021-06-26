use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use webserver::ThreadPool;

use std::thread;
use std::time::Duration;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    thread::sleep(Duration::from_secs(5));
    let content = fs::read_to_string("test.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        // let strem = stream.unwrap();
        match stream {
            Ok(v) => {
                pool.execute(|| {
                    handle_connection(v);
                });
            },
            Err(e) => println!("error parsing header: {:?}", e),
        }
        // println!("connetion established");
        // handle_connection(strem);
    }

    println!("Hello, world!");
}
