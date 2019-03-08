use std::error::Error;
use std::fs;

pub struct Config {
    pub arguments: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        Ok(Config {
            arguments: args[1..].to_vec(),
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
    let filenames = list_files_in_dir("/Users/chris-paterson/Documents/notes");
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
        let metadata = fs::metadata(&p).unwrap();
        let is_dir = metadata.is_dir();

        // TODO: Check name to see if starts with . (is hidden). If so, skip.

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
