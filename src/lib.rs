use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::Command;

mod file_manager;

pub enum Task {
    New,
    Read,
    Edit,
    Delete,
    Help,
}

pub struct Config {
    pub recall_root_dir: String,
    pub path_parts: Vec<String>,
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
            Task::Read => 1, // TODO: Maybe read with 0 args lists everything?
            _ => 2,
        };

        if task_args.len() < min_args {
            return Err("Not enough arguments.");
        }

        let path_parts = args[min_args..].to_vec();

        let rd = env::var("RECALL_DIR");
        if rd.is_err() {
            return Err("Expected RECALL_DIR env variable but found none");
        };

        let recall_root_dir = rd.unwrap();

        Ok(Config {
            recall_root_dir,
            path_parts,
            task,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let task_result = match config.task {
        Task::Read => execute_read(&config),
        Task::New => execute_create(&config),
        Task::Edit => execute_edit(&config),
        Task::Delete => execute_delete(&config),
        Task::Help => unimplemented!(),
    };

    if task_result.is_err() {
        return task_result;
    };

    Ok(())
}

fn generate_sub_root_dir(config: &Config) -> String {
    let sub_root_dir = format!("{}/{}", config.recall_root_dir, config.path_parts.join("/"));
    sub_root_dir
}

fn execute_read(config: &Config) -> Result<(), Box<dyn Error>> {
    let sub_root_dir = generate_sub_root_dir(&config);
    // Go to the dir and grab anything in that and lower.
    // The returned list has the deepest files at the start of the list.
    let paths = match file_manager::markdown_paths(&sub_root_dir) {
        Some(paths) => paths,
        None => return Err(format!("No such directory {}", &sub_root_dir))?,
    };

    for p in paths {
        match file_manager::read_file(&p) {
            Ok(contents) => println!("{}", contents),
            Err(error) => return Err(error.into()),
        }
    }

    Ok(())
}

fn execute_create(config: &Config) -> Result<(), Box<dyn Error>> {
    match file_manager::create_missing_files(&config.recall_root_dir, &config.path_parts) {
        Ok(deepest_file) => {
            Command::new("vim")
                .arg(deepest_file)
                .status()
                .expect("Unable to open file in vim.");
        }
        Err(error) => Err(format!("Error creating file: {}", error))?,
    };

    Ok(())
}

// Is pretty much the exact same as create.
fn execute_edit(config: &Config) -> Result<(), Box<dyn Error>> {
    execute_create(&config)
}

fn execute_delete(config: &Config) -> Result<(), Box<dyn Error>> {
    let sub_root_dir = generate_sub_root_dir(&config);

    // Get list of files that will be deleted.
    let all_files_to_delete = match file_manager::all_paths(&sub_root_dir) {
        Some(paths) => paths,
        None => return Err(format!("No such directory {}", &sub_root_dir))?,
    };

    // Ensure user is ok with that.
    println!("You are about to delete: ");

    let formatted_list = all_files_to_delete
        .into_iter()
        .map(PathBuf::into_os_string)
        .map(|f| format!("- {}\n", f.into_string().unwrap()));

    for name in formatted_list {
        println!("{}", name);
    }

    println!("Are you sure? YES/NO");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user confirmation.");

    let success = match input.trim_end() {
        "YES" => file_manager::delete_dir(&sub_root_dir),
        _ => {
            return {
                println!("Delete cancelled.");
                Ok(())
            };
        }
    };

    match success {
        Ok(_) => {
            println!("Deleted {}", &sub_root_dir);
            Ok(())
        }
        Err(error) => Err(format!("Error deleting dir: {}", error))?,
    }
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
fn config_path_parts_not_include_flag() {
    env::set_var("RECALL_DIR", "./test/test_dir");
    let args: [String; 4] = [
        String::from("recall"),
        String::from("-n"),
        String::from("tmux"),
        String::from("layouts"),
    ];

    let config = Config::new(&args).unwrap();
    assert!(config.path_parts.len() == 2);
    assert!(!config.path_parts.contains(&args[0]));
    assert!(!config.path_parts.contains(&args[1]));
    assert!(config.path_parts.contains(&args[2]));
    assert!(config.path_parts.contains(&args[3]));
}

#[test]
fn path_only_uses_args() {
    env::set_var("RECALL_DIR", "./test/test_dir");
    let args: [String; 3] = [
        String::from("recall"),
        String::from("tmux"),
        String::from("layouts"),
    ];

    let config = Config::new(&args).unwrap();
    let sub_root_dir = generate_sub_root_dir(&config);
    assert!(sub_root_dir == "./test/test_dir/tmux/layouts");
}
