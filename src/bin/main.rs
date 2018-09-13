use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

extern crate webserver;
use webserver::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream)
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  stream.read(&mut buffer).unwrap();

  let home = b"GET / HTTP/1.1\r\n";
  let slow = b"GET /slow HTTP/1.1\r\n";
  
  let (status_line, content_file) = if buffer.starts_with(home) {
    ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
  } else if buffer.starts_with(slow) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK\r\n\r\n", "html/slow.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
  };

  let contents = fs::read_to_string(content_file).unwrap();
  let response = format!("{}{}", status_line, contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();  
}