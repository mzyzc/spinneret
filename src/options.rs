use clap::{Arg, App};

pub struct Options {
    pub address: String,
    pub config_path: String,
    pub root_path: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            address: String::from("127.0.0.1:80"),
            config_path: String::from("settings.conf"),
            root_path: String::from("../www/index.html"),
        }
    }
}

impl Options {
    pub fn new() -> Self {
        let mut opts: Options = Default::default();
        Self::set_args(&mut opts);
        Self::print_options(&opts);

        opts
    }

    fn set_args(opts: &mut Options) {
        let args = Self::get_args();

        if args.value_of("address").is_some() {
            opts.address = String::from(args.value_of("address").unwrap());
        }

        if args.value_of("config").is_some() {
            opts.config_path = String::from(args.value_of("config").unwrap());
        }

        if args.value_of("webroot").is_some() {
            opts.root_path = String::from(args.value_of("webroot").unwrap());
        }
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

    fn print_options(opts: &Options) {
        println!("Serving on address: {}", opts.address);
        println!("Config file located at: {}", opts.config_path);
        println!("Web root directory located at: {}", opts.root_path);
        println!();
    }
}
