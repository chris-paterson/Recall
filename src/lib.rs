use std::env;
use std::error::Error;

mod file_manager;

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
    let sub_dir = generate_sub_dir_path(&config);
    // Go to the dir and grab anything in that and lower
    // we then want to concat the files into one and output it
    let filenames = file_manager::recursively_get_filepaths(&sub_dir);

    let mut file_contents = Vec::new();
    for f in filenames {
        match file_manager::get_contents_of_file(&f) {
            Ok(contents) => file_contents.push(contents),
            Err(error) => println!("ERROR: {}", error),
        }
    }

    println!("{}", file_contents.join("\n\n\n"));

    Ok(())
}

fn generate_sub_dir_path(config: &Config) -> String {
    let sub_dir = format!("{}/{}", config.root_path, &config.arguments.join("/"));
    sub_dir
}

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
fn path_only_uses_args() {
    env::set_var("RECALL_PATH", "./test/test_dir");
    let args: [String; 3] = [
        String::from("recall"),
        String::from("tmux"),
        String::from("layouts"),
    ];

    let config = Config::new(&args).unwrap();
    let sub_dir = generate_sub_dir_path(&config);
    assert!(sub_dir == "./test/test_dir/tmux/layouts");
}
