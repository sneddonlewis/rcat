use std::{error::Error, fs::{self, File}, io::{BufRead, self, BufReader}};

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
    // read a file(files) line by line and print to STDIN
    if config.number_lines {
        for line in number_lines(read_lines(config.files)) {
            println!("{}", line);
        }
    } else if config.number_nonblank_lines {
        for line in number_nonblank_lines(read_lines(config.files)) {
            println!("{}", line);
        }
    } else {
        for line in read_lines(config.files) {
            println!("{}", line);
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

fn number_nonblank_lines(lines: Vec<String>) -> Vec<String> {
    lines.iter().enumerate().map(|(idx, line)| {
        if line.is_empty() {
            String::from(line)
        } else {
            format!("{}\t{}", idx + 1, line)
        }
    }).collect()
}

fn number_lines(lines: Vec<String>) -> Vec<String> {
    lines.iter().enumerate().map(|(idx, line)| {
        format!("{}\t{}", idx + 1, line)
    }).collect()
}

fn read_lines(files: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for file in &files {
        for line in fs::read_to_string(file).unwrap().lines() {
            result.push(String::from(line));
        }
    }

    result
}
