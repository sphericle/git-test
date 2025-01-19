use std::{fs, ffi::OsStr};
use serde_json::{self, json_internal};

const REPO_DATA: &str = "repo/data/";

fn get_files() -> fs::ReadDir {
    match fs::read_dir(REPO_DATA) {
        Ok(paths) => return paths,
        Err(e) => {
            panic!("Error reading directory: {}", e);
        }
    };
}

fn main() {
    // get all files in data folder
    let paths = get_files();
    
    // loop through all data
    for path in paths {
        match path {
            Ok(entry) => {

                // check if the file is a json file
                if let Some(ext) = entry.path()
                    .extension()
                    .and_then(OsStr::to_str) {
                    
                    if ext != "json" {
                        panic!("fuck")
                    }
                    if entry.path().starts_with("_") {
                        continue;
                    }

                    // parse the file as json
                    let file_content = fs::read_to_string(entry.path())
                        .expect("Unable to read file");
                    
                    let mut file: serde_json::Value = match serde_json::from_str(&file_content) {
                        Ok(json) => json,
                        Err(_) => {
                            eprintln!("Skipping file {}: JSON was not well-formatted", entry.path().display());
                            continue;
                        }
                    };
                            
                    // if the author exists
                    if let Some(author) = file.get("author") {
                        println!("Processing file {}", entry.path().display());
                        let author_clone = author.clone();
                        if let Some(creators) = file.get_mut("creators").and_then(|c| c.as_array_mut()) {
                            if !creators.contains(&author_clone) {
                                creators.push(author_clone);
                            }
                        } else {
                            // If "creators" does not exist or is not an array, create a new array with the author
                            file.as_object_mut().unwrap().insert("creators".to_string(), serde_json::json!([author_clone]));
                        }
                        
                        file.as_object_mut().unwrap().remove("author");
                        let updated_content = serde_json::to_string_pretty(&file).expect("Failed to serialize JSON");
                        let updated_content_with_tabs = updated_content.replace("    ", "\t");
                        fs::write(entry.path(), updated_content_with_tabs).expect("Unable to write file");
                    }
                    
                }
            },
            Err(e) => eprintln!("Error reading path: {}", e),
        }
    }
}
