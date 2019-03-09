use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub arguments: Vec<String>,
    pub root_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let rp = env::var("RECALL_PATH");
        if rp.is_err() {
            return Err("Expected RECALL_PATH env variable but found none");
        };

        Ok(Config {
            arguments: args[1..].to_vec(),
            root_path: rp.unwrap(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Constructs string of file contents
    for c in config.arguments {
        println!("{}", c);
    }

    // Go to the dir and grab anything in that and lower
    // we then want to concat the files into one and output it
    let filenames = list_files_in_dir(&config.root_path);
    for f in filenames {
        println!("{}", f);
    }

    Ok(())
}

// TODO: Make this return an optional.
fn list_files_in_dir(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut filenames = Vec::new();

    for path in paths {
        let p = path.unwrap().path();

        // Don't include hidden files.
        let filename = p.file_name().unwrap();
        if filename.to_str().unwrap().starts_with(".") {
            continue;
        }

        let is_dir = fs::metadata(&p).unwrap().is_dir();

        if is_dir {
            let nested_filenames = list_files_in_dir(&p.display().to_string());
            for filename in nested_filenames {
                filenames.push(filename);
            }
        } else {
            filenames.push(p.display().to_string());
        }
    }

    filenames
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

    #[test]
    fn config_removes_first_cl_parameter() {
        let args: [String; 3] = [
            String::from("recall"),
            String::from("tmux"),
            String::from("layouts"),
        ];

        let config = Config::new(&args).unwrap();
        assert!(config.arguments.len() == 2);
        assert!(!config.arguments.contains(&args[0]));
        assert!(config.arguments.contains(&args[1]));
        assert!(config.arguments.contains(&args[2]));
    }
}
