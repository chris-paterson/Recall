use std::env;
use std::error::Error;

mod file_manager;

pub enum Task {
    New,
    Read,
    Edit,
    Delete,
    Help,
}

pub struct Config {
    pub root_path: String,
    pub arguments: Vec<String>,
    pub task: Task,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let task_args = args[1..].to_vec(); // Skip program name.
        let task = match &*task_args[0] {
            "-n" => Task::New,
            "-e" => Task::Edit,
            "-d" => Task::Delete,
            "-h" => Task::Help,
            _ => Task::Read,
        };

        // Ensure we have enough args for the task.
        let min_args = match task {
            Task::Help => 0,
            Task::Read => 1,
            _ => 2,
        };

        if task_args.len() < min_args {
            return Err("Not enough arguments.");
        }

        let path_args = args[min_args..].to_vec();

        let rp = env::var("RECALL_PATH");
        if rp.is_err() {
            return Err("Expected RECALL_PATH env variable but found none");
        };

        let root_path = format!("{}/{}", rp.unwrap(), path_args.join("/"));

        Ok(Config {
            root_path,
            arguments: path_args,
            task,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let task_result = match config.task {
        Task::New => unimplemented!(),
        Task::Read => execute_read(&config),
        Task::Edit => unimplemented!(),
        Task::Delete => unimplemented!(),
        Task::Help => unimplemented!(),
    };

    if task_result.is_err() {
        return task_result;
    };

    Ok(())
}

fn execute_read(config: &Config) -> Result<(), Box<dyn Error>> {
    // Go to the dir and grab anything in that and lower
    // we then want to concat the files into one and output it
    let filenames = match file_manager::recursively_get_filepaths(&config.root_path) {
        Some(filenames) => filenames,
        None => return Err("No files found in given dir.")?,
    };

    let mut file_contents = Vec::new();
    for f in filenames {
        match file_manager::get_contents_of_file(&f) {
            Ok(contents) => file_contents.push(contents),
            Err(error) => println!("ERROR: {}", error),
        }
    }

    // TODO: Currently just unix.
    // TODO: Robustness.
    if std::process::Command::new("clear")
        .status()
        .unwrap()
        .success()
    {
        println!("{}", file_contents.join("\n\n\n"));
    }
    Ok(())
}

#[test]
fn not_enough_program_args() {
    let args: [String; 1] = [String::from("recall")];

    match Config::new(&args) {
        Ok(_) => assert!(false, "Config should require at least two arguments."),
        Err(_) => assert!(true),
    }
}

#[test]
fn not_enough_task_args() {
    let create_args: [String; 2] = [String::from("recall"), String::from("-n")];

    match Config::new(&create_args) {
        Ok(_) => assert!(false, "-n flag should require at least two arguments."),
        Err(_) => assert!(true),
    }
}

#[test]
fn config_arguments_not_include_flag() {
    env::set_var("RECALL_PATH", "./test/test_dir");
    let args: [String; 4] = [
        String::from("recall"),
        String::from("-n"),
        String::from("tmux"),
        String::from("layouts"),
    ];

    let config = Config::new(&args).unwrap();
    assert!(config.arguments.len() == 2);
    assert!(!config.arguments.contains(&args[0]));
    assert!(!config.arguments.contains(&args[1]));
    assert!(config.arguments.contains(&args[2]));
    assert!(config.arguments.contains(&args[3]));
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
    assert!(&config.root_path == "./test/test_dir/tmux/layouts");
}
