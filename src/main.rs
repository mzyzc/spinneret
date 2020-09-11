use clap::{Arg, App};
use std::net::{TcpListener, TcpStream};

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

fn handle_client(stream: TcpStream) {
    println!("stream received");
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
