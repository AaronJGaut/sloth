// 1. Determine filename from default or env
// 2. Open file
// 3. Build list of tasks
// 4. Pick task at random
// 5. Wrap text of task based on max width
// 6. Determine text rectangle dimensions
// 7. Render text

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
