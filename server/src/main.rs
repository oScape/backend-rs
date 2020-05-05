use server_lib::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(_stream);
        });
    }

    println!("Shutting dowm!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let data = b"GET /data HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello")
    } else if buffer.starts_with(data) {
        ("HTTP/1.1 200 OK\r\n\r\n", "api data")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "not found")
    };

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
