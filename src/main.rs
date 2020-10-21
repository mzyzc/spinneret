mod options;

use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let opts = options::Options::new();

    let listener = TcpListener::bind(&opts.address)?;

    for stream in listener.incoming() {
        handle_client(stream?, &opts);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, opts: &options::Options) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    //let mut headers = [httparse::EMPTY_HEADER; 16];
    //let mut request = httparse::Request::new(&mut headers);
    //let req_status = request.parse(&buffer).unwrap();

    let html = fs::read_to_string(&opts.root_path).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: {}\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        "text/html",
        html.len(),
        html,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
