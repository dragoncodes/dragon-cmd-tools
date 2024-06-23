use core::panic;
use std::fs::{self, ReadDir};
use std::path::PathBuf;

fn main() {
    let arg1 = std::env::args().nth(1);
    let arg2 = std::env::args().nth(2);

    if arg1.is_none() && arg2.is_none() {
        panic!("No arguments supplied");
    }

    let file_name: String = if arg2.is_none() {
        arg1.clone().unwrap()
    } else {
        arg2.clone().unwrap()
    }
    .to_lowercase();

    let dir = if arg2.is_some() {
        PathBuf::from(arg1.unwrap().clone())
    } else {
        PathBuf::from("./".to_owned())
    };

    let mut current_path = std::env::current_dir().expect("To have a current path");

    current_path.push(&dir);

    let read_dir_result = fs::read_dir(&dir).expect("Unable to read dir");

    search_for_file_in_dir(read_dir_result, &file_name);
}

fn search_for_file_in_dir(dir: ReadDir, file_to_search: &String) {
    for path in dir {
        match path {
            Ok(entry) => match entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        let new_dir = fs::read_dir(entry.path());
                        match new_dir {
                            Ok(res) => search_for_file_in_dir(res, file_to_search),
                            Err(_) => continue,
                        }
                    } else {
                        let file = entry.file_name().into_string().unwrap().to_lowercase();

                        if file.contains(file_to_search) {
                            println!("{}", entry.path().to_str().unwrap());
                        }
                    }
                }
                Err(_) => continue,
            },

            Err(_) => continue,
        }
    }
}
