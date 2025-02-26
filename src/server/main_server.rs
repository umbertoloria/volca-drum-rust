use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

pub fn main_server() -> JoinHandle<()> {
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:9666").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    })
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
