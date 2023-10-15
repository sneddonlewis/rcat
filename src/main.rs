use rcat::{run, get_config};

fn main() {
    let config_result = get_config();
    match config_result {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(config) => {
            if let Err(e) = run(config) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
