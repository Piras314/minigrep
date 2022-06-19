// Programmed by Piboy314

use std::env; // Using env for environment variables
use std::process; // Using process to handle exiting with nonzero exit code

use minigrep::Config;

fn main() {
    // Collect command line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}
