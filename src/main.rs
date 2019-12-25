use std::env;
use std::process;

use sloth::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_| {
        sloth::slothsay("I couldn't determine where to look for your tasks file. Neither SLOTH_TASKS nor HOME was set");
        process::exit(1);
    });
    if let Err(err) = sloth::run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
