use std::{
    error::Error,
    fs::File,
    io::{BufRead, self, BufReader},
};

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
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
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
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(handle) => {
                if config.number_lines {
                    handle
                        .lines()
                        .into_iter()
                        .enumerate()
                        .for_each(|(idx, l)| println!("{:>6}\t{}", idx + 1, l.unwrap()));
                } else if config.number_nonblank_lines {
                    let mut line_number = 0;
                    for line_result in handle.lines() {
                        let line = line_result?;
                        if line.is_empty() {
                            println!();
                        } else {
                            line_number += 1;
                            println!("{:>6}\t{}", line_number, line);
                        }
                    }
                }else {
                    handle
                        .lines()
                        .for_each(|l| println!("{}", l.unwrap()));
                }
            },
        }
    }

    Ok(())
}

fn open(filename: &str) -> RcatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

