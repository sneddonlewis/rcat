use std::error::Error;

use clap::{App, Arg};

pub type RcatResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_config() -> RcatResult<Config> {
    let matches = App::new("rcat")
        .version("0.1.0")
        .author("Lewis Sneddon")
        .about("A Rusty Unix Cat, or a clone of the cat Unix tool to Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> RcatResult<()> {
    dbg!(config);
    Ok(())
}
