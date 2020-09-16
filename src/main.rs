mod http;

use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use crate::http::{HttpRequest, HttpResponse};
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
    let request = HttpRequest::from_stream(&stream);

    let matches = get_args();
    let webroot = matches.value_of("webroot").unwrap_or("../www/index.html");
    let html = fs::read_to_string(webroot).unwrap();

    let response = HttpResponse {
        content_type: String::from("text/html"),
        content_length: html.len(),
        body: html,
    };

    stream.write(response.to_string().as_bytes()).unwrap();
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
