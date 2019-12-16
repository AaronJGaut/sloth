use std::env;
use std::process;

use sloth::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    if let Err(err) = sloth::run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
