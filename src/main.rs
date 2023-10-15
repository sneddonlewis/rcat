use rcat::{run, get_config};

fn main() {
    if let Err(e) = get_config().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
