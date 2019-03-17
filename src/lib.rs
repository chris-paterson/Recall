use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

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
    let sub_dir = create_sub_dir_path(&config);
    // Go to the dir and grab anything in that and lower
    // we then want to concat the files into one and output it
    let filenames = list_files_in_dir(&sub_dir);

    let mut file_contents = Vec::new();
    for f in filenames {
        match get_contents_of_file(&f) {
            Ok(contents) => file_contents.push(contents),
            Err(error) => println!("ERROR: {}", error),
        }
    }

    println!("{}", file_contents.join("\n\n\n"));

    Ok(())
}

fn create_sub_dir_path(config: &Config) -> String {
    let sub_dir = format!("{}/{}", config.root_path, &config.arguments.join("/"));
    sub_dir
}

// TODO: Make this return an optional.
fn list_files_in_dir(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut filenames = Vec::new();

    for p in paths {
        let path = p.unwrap().path();

        // Don't include hidden files.
        let filename = path.file_name().unwrap();
        if filename.to_str().unwrap().starts_with(".") {
            continue;
        }

        let is_dir = fs::metadata(&path).unwrap().is_dir();

        if is_dir {
            let nested_filenames = list_files_in_dir(&path.display().to_string());
            for filename in nested_filenames {
                filenames.push(filename);
            }
        } else {
            filenames.push(path.display().to_string());
        }
    }

    filenames
}

fn get_contents_of_file(dir: &str) -> std::io::Result<String> {
    let mut file = match fs::File::open(dir) {
        Ok(f) => f,
        Err(error) => return Err(error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => return Err(error),
    }
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
        env::set_var("RECALL_PATH", "./test/test_dir");
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

    #[test]
    fn lists_files_recursively() {
        let dirs = list_files_in_dir("./test/test_dir");

        // Laziest way to test the vec contains the files we want.
        let dir_string = dirs.join(", ");

        assert!(dir_string.contains("surround.md"));
        assert!(dir_string.contains("grep.md"));
        assert!(dir_string.contains("layouts.md"));
        assert!(dir_string.contains("tmux.md"));
    }

    #[test]
    fn path_only_uses_args() {
        env::set_var("RECALL_PATH", "./test/test_dir");
        let args: [String; 3] = [
            String::from("recall"),
            String::from("tmux"),
            String::from("layouts"),
        ];

        let config = Config::new(&args).unwrap();
        let sub_dir = create_sub_dir_path(&config);
        assert!(sub_dir == "./test/test_dir/tmux/layouts");
    }
}
