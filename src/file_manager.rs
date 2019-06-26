extern crate glob;

use glob::glob;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn markdown_paths(dir: &str) -> Option<Vec<PathBuf>> {
    let format = format!("{}/**/*.md", dir);
    let paths: Vec<PathBuf> = glob(&format).unwrap().map(|r| r.unwrap()).collect();

    return match paths.len() {
        0 => None,
        _ => Some(paths),
    };
}

pub fn read_file(path: &Path) -> std::io::Result<String> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(error) => Err(error),
    }
}

pub fn create_missing_files(recall_path: &str, path_parts: &[String]) -> std::io::Result<String> {
    // We want to include a stub file in each path we create.
    let mut deepest_filepath = String::new();
    for (index, path_part) in path_parts.iter().enumerate() {
        let merged_path_part = path_parts[0..=index].join("/");

        let full_path = format!("{}/{}", recall_path, merged_path_part);
        let filepath = format!("{}/{}.md", full_path, path_part);

        if !Path::new(&filepath).exists() {
            fs::create_dir(full_path.clone())?;
            println!("Created directory: {}", full_path);

            let mut file = fs::File::create(&filepath)?;
            println!("Created file: {}", &filepath);

            // Give file a heading.
            let file_heading = format!(
                "{} {}",
                "#".repeat(index + 1),
                capitalize_first_letter(path_part)
            );
            file.write_all(file_heading.as_bytes())?;
        }

        deepest_filepath = filepath.clone();
    }

    Ok(deepest_filepath.to_string())
}

fn capitalize_first_letter(string: &str) -> String {
    let mut characters = string.chars();
    match characters.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + characters.as_str(),
    }
}

#[ignore]
#[test]
fn lists_files_recursively() {
    let dirs = markdown_paths("./test/test_dir").unwrap();

    assert!(dirs[0].ends_with("grep.md"));
    assert!(dirs[1].ends_with("layouts.md"));
    assert!(dirs[2].ends_with("tabs/tabs.md"));
    assert!(dirs[3].ends_with("tmux.md"));
    assert!(dirs[4].ends_with("surround.md"));
    assert!(dirs[5].ends_with("vim.md"));
}

#[test]
fn valid_path_returns_some() {
    let dirs = markdown_paths("./test/test_dir");
    assert!(dirs.is_some());
}

#[test]
fn non_valid_paths_return_none() {
    let dirs = markdown_paths("./thispathdoesnotexist");

    assert!(dirs.is_none(), true);
}

#[test]
fn create_successfully_creates_files_for_each_level() {
    let args: [String; 2] = [String::from("swift"), String::from("keypath")];
    let arg_vec = args.to_vec();
    let _create_missing_files = create_missing_files("./test/test_dir", &arg_vec);

    assert!(Path::new("./test/test_dir/swift/swift.md").exists());
    assert!(Path::new("./test/test_dir/swift/keypath/keypath.md").exists());

    // Cleanup.
    if Path::new("./test/test_dir/swift").exists() {
        let _cleanup = fs::remove_dir_all("./test/test_dir/swift");
    }

    assert!(Path::new("./test/test_dir/swift").exists() == false);
}
