use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = get_args();

    let address = matches.value_of("address").unwrap_or("127.0.0.1:80");
    let config = matches.value_of("config").unwrap_or("settings.conf");
    let webroot = matches.value_of("webroot").unwrap_or("../www/index.html");
    println!("Serving on address: {}", address);
    println!("Config file located at: {}", config);
    println!("Web root directory located at: {}", webroot);

    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);
    let req_status = request.parse(&buffer).unwrap();

    let matches = get_args();
    let webroot = matches.value_of("webroot").unwrap_or("../www/index.html");
    let html = fs::read_to_string(webroot).unwrap();

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

fn get_args() -> clap::ArgMatches {
    App::new("spinneret")
        .arg(Arg::with_name("address")
            .short('a')
            .long("address")
            .value_name("ADDRESS")
            .takes_value(true)
            .about("Sets address to serve on"))
        .arg(Arg::with_name("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .takes_value(true)
            .about("Sets config file"))
        .arg(Arg::with_name("webroot")
            .short('r')
            .long("root")
            .value_name("DIRECTORY")
            .takes_value(true)
            .about("Sets the web root directory"))
        .get_matches()
}
