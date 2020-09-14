mod http;

use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use crate::http::HttpResponse;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = get_args();

    let config = matches.value_of("config").unwrap_or("settings.conf");
    let webroot = matches.value_of("webroot").unwrap_or("../www/index.html");
    println!("Config file located at: {}", config);
    println!("Web root directory located at: {}", webroot);

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

    let matches = get_args();
    let webroot = matches.value_of("webroot").unwrap_or("../www/index.html");
    let html = fs::read_to_string(webroot).unwrap();

    let response = HttpResponse {
        content_type: String::from("text/html"),
        content_length: html.len(),
        body: html,
    };

    stream.write(response.write_response().as_bytes()).unwrap();
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
        .arg(Arg::with_name("webroot")
            .short('r')
            .long("root")
            .value_name("DIRECTORY")
            .takes_value(true)
            .about("Sets the web root directory"))
        .arg(Arg::with_name("INPUT")
            .required(true))
        .get_matches()
}
