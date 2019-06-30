use std::env;
use std::process;

use recall;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = recall::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        recall::execute_help().expect("Printing help should never fail...");
        process::exit(1);
    });

    if let Err(e) = recall::run(config) {
        println!("Error running recall: {}", e);
        process::exit(1);
    }
}
