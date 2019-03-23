use std::fs;
use std::io::prelude::*;

// TODO: Make this return an optional.
pub fn recursively_get_filepaths(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
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
            let nested_filenames = recursively_get_filepaths(&path.display().to_string());
            for filename in nested_filenames {
                filenames.push(filename);
            }
        } else {
            filenames.push(path.display().to_string());
        }
    }

    filenames
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

#[test]
fn lists_files_recursively() {
    let dirs = recursively_get_filepaths("./test/test_dir");

    // Laziest way to test the vec contains the files we want.
    let dir_string = dirs.join(", ");

    assert!(dir_string.contains("surround.md"));
    assert!(dir_string.contains("grep.md"));
    assert!(dir_string.contains("layouts.md"));
    assert!(dir_string.contains("tmux.md"));
}

//#[test]
//fn non_valid_paths_handled_gracefully() {
//    let dirs = recursively_get_filepaths("./thispathdoesnotexist");

//    assert!(dirs.is_err(), true);
//}
