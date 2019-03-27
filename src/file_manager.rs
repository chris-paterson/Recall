use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn recursively_get_filepaths(dir: &str) -> Option<Vec<String>> {
    let paths = match fs::read_dir(dir) {
        Ok(path) => path,
        Err(_) => return None,
    };

    let mut filenames = Vec::new();

    for p in paths {
        let path = p.unwrap().path();

        // Don't include hidden files.
        let filename = path.file_name().unwrap();
        if filename.to_str().unwrap().starts_with('.') {
            continue;
        }

        let is_dir = fs::metadata(&path).unwrap().is_dir();

        if is_dir {
            let nested_filenames = match recursively_get_filepaths(&path.display().to_string()) {
                Some(filenames) => filenames,
                None => continue, // Just skip over this one.
            };

            for filename in nested_filenames {
                filenames.push(filename);
            }
        } else {
            filenames.push(path.display().to_string());
        }
    }

    Some(filenames)
}

pub fn get_contents_of_file(dir: &str) -> std::io::Result<String> {
    let mut file = match fs::File::open(dir) {
        Ok(f) => f,
        Err(error) => return Err(error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}

pub fn create_file(recall_path: &str, path_parts: &Vec<String>) -> std::io::Result<()> {
    // We want to include a stub file in each path we create.
    for (index, path_part) in path_parts.iter().enumerate() {
        let merged_path_part = path_parts[0..index+1].join("/");

        let full_path = format!("{}/{}", recall_path, merged_path_part);
        let filepath = format!("{}/{}.md", full_path, path_part);

        println!("asdf{}", merged_path_part);
        if !Path::new(&filepath).exists() {
            fs::create_dir(full_path)?;
            fs::File::create(filepath)?;
        }
    }

    Ok(())
}

#[test]
fn lists_files_recursively() {
    let dirs = recursively_get_filepaths("./test/test_dir").unwrap();

    // Laziest way to test the vec contains the files we want.
    let dir_string = dirs.join(", ");

    assert!(dir_string.contains("surround.md"));
    assert!(dir_string.contains("grep.md"));
    assert!(dir_string.contains("layouts.md"));
    assert!(dir_string.contains("tmux.md"));
}

#[test]
fn valid_path_returns_some() {
    let dirs = recursively_get_filepaths("./test/test_dir");
    assert!(dirs.is_some());
}

#[test]
fn non_valid_paths_return_none() {
    let dirs = recursively_get_filepaths("./thispathdoesnotexist");

    assert!(dirs.is_none(), true);
}

#[test]
fn create_successfully_creates_files_for_each_level() {
    let args: [String; 2] = [
        String::from("swift"),
        String::from("keypath"),
    ];
    let arg_vec = args.to_vec();
    let _create_file = create_file("./test/test_dir", &arg_vec);

    assert!(Path::new("./test/test_dir/swift/swift.md").exists());
    assert!(Path::new("./test/test_dir/swift/keypath/keypath.md").exists());

    // Cleanup.
    if Path::new("./test/test_dir/swift").exists() {
        let _cleanup = fs::remove_dir_all("./test/test_dir/swift");
    }

    assert!(!Path::new("./test/test_dir/swift").exists());
}
