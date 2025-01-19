use serde_json::{self, Map, Value, json_internal};
use std::{ffi::OsStr, fs};

const REPO_DATA: &str = "repo/data/";

fn get_files() -> fs::ReadDir {
    match fs::read_dir(REPO_DATA) {
        Ok(paths) => paths,
        Err(e) => {
            panic!("Error reading directory: {}", e);
        }
    }
}

fn main() {
    // Get all files in the data folder
    let paths = get_files();

    // Loop through all data
    for path in paths {
        match path {
            Err(e) => eprintln!("Error reading path: {}", e),
            Ok(entry) => {
                // Check if the file is a JSON file
                if let Some(ext) = entry.path().extension().and_then(OsStr::to_str) {
                    if entry.path().starts_with("_") || ext != "json" {
                        continue;
                    }

                    // Parse the file as JSON
                    let file_content =
                        fs::read_to_string(entry.path()).expect("Unable to read file");

                    let mut file: Map<String, Value> = match serde_json::from_str(&file_content) {
                        Ok(json) => json,
                        Err(_) => {
                            eprintln!(
                                "Skipping file {}: JSON was not well-formatted",
                                entry.path().display()
                            );
                            continue;
                        }
                    };

                    // If the author exists
                    if let Some(author) = file.get("author") {
                        if author.as_str().unwrap_or("").is_empty() {
                            continue;
                        }
                        println!("Processing file {}", entry.path().display());
                        let author_clone = author.clone();
                        if let Some(creators) =
                            file.get_mut("creators").and_then(|c| c.as_array_mut())
                        {
                            if !creators.contains(&author_clone) {
                                creators.push(author_clone);
                            }
                        } else {
                            // If "creators" does not exist or is not an array, create a new array with the author
                            file.insert("creators".to_string(), serde_json::json!([author_clone]));
                        }
                    }

                    // Remove the author key
                    file.remove("author");

                    // Serialize the JSON with pretty formatting
                    let updated_content =
                        serde_json::to_string_pretty(&file).expect("Failed to serialize JSON");
                    let updated_content_with_tabs = updated_content.replace("    ", "\t");

                    // Write the updated JSON back to the file
                    fs::write(entry.path(), updated_content_with_tabs)
                        .expect("Unable to write file");
                }
            }
        }
    }
}
