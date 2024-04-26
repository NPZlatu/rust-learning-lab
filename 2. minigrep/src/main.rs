
/**
 * Generating a command line tool named grep "globally search a regular expression and print"
 */
extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    match Config::new(env::args()) {
        Ok(config) => {
            if let Err(e) = minigrep::run(config) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };
}
