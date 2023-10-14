use std::error::Error;

pub type RcatResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> RcatResult<()> {
    println!("Hi there");

    Ok(())
}
