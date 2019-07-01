extern crate glob;

use glob::glob;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn markdown_paths(dir: &str) -> Option<Vec<PathBuf>> {
    let paths = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(_) => return None,
    };

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    // We want to ensure files are added before we look at child directories.
    for p in paths {
        let path = p.unwrap().path();

        let is_dir = fs::metadata(&path).unwrap().is_dir();
        if is_dir {
            dirs.push(path);
        } else {
            let filename = path.file_name().unwrap();
            if filename.to_str().unwrap().ends_with(".md") {
                files.push(path);
            }
        }
    }

    dirs.sort();
    for dir in dirs {
        match markdown_paths(dir.to_str().unwrap()) {
            Some(nested_files) => files.extend(nested_files),
            None => continue,
        }
    }

    Some(files)
}

pub fn all_paths(dir: &str) -> Option<Vec<PathBuf>> {
    let format = format!("{}/**/*", dir);
    let paths: Vec<PathBuf> = glob(&format).unwrap().map(|r| r.unwrap()).collect();

    match paths.len() {
        0 => None,
        _ => Some(paths),
    }
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

pub fn delete_dir(string: &str) -> std::io::Result<()> {
    match fs::remove_dir_all(string) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

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
    let root_dir = "./test/create_test_dir";
    let args: [String; 2] = [String::from("swift"), String::from("keypath")];
    let arg_vec = args.to_vec();
    let _create_missing_files = create_missing_files(root_dir, &arg_vec);

    let swift_md_path = format!("{}/swift/swift.md", root_dir);
    assert!(Path::new(&swift_md_path).exists());

    let keypath_md_path = format!("{}/swift/swift.md", root_dir);
    assert!(Path::new(&keypath_md_path).exists());

    // Cleanup.
    let cleanup_dir = format!("{}/swift", root_dir);
    let _ = delete_dir(&cleanup_dir);

    assert!(Path::new(&format!("{}/swift", root_dir)).exists() == false);
}
