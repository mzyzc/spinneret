use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = get_args();

    let config = matches.value_of("config").unwrap_or("settings.conf");
    println!("Config file located at: {}", config);

    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let html = fs::read_to_string("../www/index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        "text/html",
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_args() -> clap::ArgMatches {
    App::new("spinneret")
        .arg(Arg::with_name("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .takes_value(true)
            .about("Sets config file"))
        .arg(Arg::with_name("INPUT")
            .required(true))
        .get_matches()
}
