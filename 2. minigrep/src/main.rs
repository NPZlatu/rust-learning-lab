/**
 * Generating a command line tool named grep "globally search a regular expression and print"
 */
extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::new(&args) {
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
