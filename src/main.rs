#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

static ADR: &str = "#### Context
#### Decision
#### Status
#### Consequences
";
static DIRECTORY: &str = "./docs/adr";

fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("cargo-fak")
        .bin_name("cargo")
        .setting(AppSettings::SubcommandRequired)
        .version(concat!("v", crate_version!()))
        .author("louise <email@luisignac.io>")
        .subcommand(SubCommand::with_name("adr")
            .about("Cargo Faaaaaak! is an ADR tool for your Rust projects. You don't need help, you are human, you are fine. What you need is better tools, Cargo Faaaaaak! aims to help you out.")
            .arg(Arg::with_name("TITLE")
                .help("A title of your ADR")
                .required(true)))
        .get_matches()
}

fn get_key() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn get_title<'a>(matches: &'a ArgMatches) -> &'a str {
    if let Some(matches) = matches.subcommand_matches("adr") {
        if matches.is_present("TITLE") {
            matches.value_of("TITLE").unwrap()
        } else {
            &"No title"
        }
    } else {
        &"No title"
    }
}

fn get_file(key: u64, title: &str) -> File {
    let filepath = format!("{}/{}_{}.md", DIRECTORY, key, title).to_string();
    let path = Path::new(&filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    }
}

fn main() -> std::io::Result<()> {
    fs::create_dir_all(DIRECTORY)?;
    let matches = get_matches();
    let key = get_key();
    let title = get_title(&matches);
    let mut file = get_file(key, title);
    file.write_all(ADR.as_bytes())
}
