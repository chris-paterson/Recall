use std::env;
use std::process;

pub struct Config {
    pub arguments: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let mut arguments = Vec::with_capacity(args.len() - 1);
        arguments.clone_from_slice(&args[1..]);
        Ok(Config { arguments })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_args() {
        let args: [String; 1] = [String::from("recall")];

        match Config::new(&args) {
            Ok(_) => assert!(false, "Config should require at least two arguments."),
            Err(_) => assert!(true),
        }
    }
}
