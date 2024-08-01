use rayon::prelude::*;
use std::env;
use std::fs::{self, ReadDir};
use std::path::PathBuf;

fn main() {
    let mut args = env::args().skip(1);
    let arg1 = args.next();
    let arg2 = args.next();

    if arg1.is_none() && arg2.is_none() {
        panic!("No arguments supplied");
    }

    let (dir, file_name) = if let Some(arg2) = arg2 {
        (
            (PathBuf::from(arg1.expect("Please provide a file name to search for"))),
            arg2,
        )
    } else if let Some(arg1) = arg1 {
        (PathBuf::from("./"), arg1)
    } else {
        panic!("No arguments supplied")
    };

    let current_path = env::current_dir().expect("Failed to get current directory");

    let target_dir = fs::canonicalize(current_path.join(&dir))
        .expect(&format!("Could not joint paths {:?} and {:?}", &current_path, &dir).to_owned());

    let read_dir_result = fs::read_dir(&target_dir).expect("Unable to read directory");

    search_for_file_in_dir(read_dir_result, &file_name);
}

fn search_for_file_in_dir(dir: ReadDir, file_to_search: &str) {
    dir.filter_map(Result::ok).par_bridge().for_each(|entry| {
        let path = entry.path();
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                if let Ok(new_dir) = fs::read_dir(path) {
                    search_for_file_in_dir(new_dir, file_to_search);
                }
            } else {
                if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                    if file_name.to_lowercase().contains(file_to_search) {
                        println!("{}", path.to_str().unwrap());
                    }
                }
            }
        }
    });
}
