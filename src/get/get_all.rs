use crate::types::{Difficulty, Level, Record};
use serde_json::{self, Map, Value};
use std::{ffi::OsStr, fs};
use url::Url;

const REPO_DATA: &str = "repo/data/";

fn get_files() -> fs::ReadDir {
    match fs::read_dir(REPO_DATA) {
        Ok(paths) => paths,
        Err(e) => {
            panic!("Error reading directory: {}", e);
        }
    }
}

pub fn get_all() -> Vec<Level> {
    let mut list: Vec<Level> = Vec::new();
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
                        Err(_e) => {
                            continue;
                        }
                    };

                    let id: i64 = match file.get_mut("id").and_then(|c| c.as_i64()) {
                        Some(id) => id,
                        None => -1,
                    };

                    let name: String = match file.get_mut("name") {
                        Some(name) => name.to_string(),
                        None => String::new(),
                    };

                    // A vector of this level's creators
                    let creators: Vec<String> = match file
                        .get_mut("creators")
                        .and_then(|c: &mut Value| c.as_array_mut())
                    {
                        Some(creators) => creators
                            .iter()
                            .filter_map(|c| c.as_str().map(|s| s.to_string()))
                            .collect(),
                        None => vec![],
                    };

                    let verifier: String = match file.get_mut("verifier") {
                        Some(verifier) => verifier.to_string(),
                        None => String::new(),
                    };

                    let verification: String = match file.get_mut("verification") {
                        Some(verification) => match Url::parse(&verification.to_string()) {
                            Ok(_) => verification.to_string(),
                            Err(_) => String::new(),
                        },
                        None => String::new(),
                    };

                    let percent_to_qualify: f64 =
                        match file.get_mut("percentToQualify").and_then(|v| v.as_f64()) {
                            Some(percent_to_qualify) => percent_to_qualify,
                            None => 100.0,
                        };

                    let song_name: String = match file.get_mut("song") {
                        Some(song_name) => song_name.to_string(),
                        None => String::new(),
                    };

                    let song_link: Option<String> = file
                        .get_mut("songLink")
                        .and_then(|song_link| song_link.as_str().map(|s| s.to_string()));

                    let difficulty: Difficulty = match file.get_mut("id").and_then(|c| c.as_u64()) {
                        Some(0) => Difficulty::BeginnerLayout,
                        Some(1) => Difficulty::EasyLayout,
                        Some(2) => Difficulty::MediumLayout,
                        Some(3) => Difficulty::HardLayout,
                        Some(4) => Difficulty::InsaneLayout,
                        Some(5) => Difficulty::MythicalLayout,
                        Some(6) => Difficulty::ExtremeLayout,
                        Some(7) => Difficulty::SupremeLayout,
                        Some(8) => Difficulty::EtherealLayout,
                        Some(9) => Difficulty::LegendaryLayout,
                        Some(10) => Difficulty::SilentLayout,
                        Some(11) => Difficulty::ImpossibleLayout,
                        Some(12_u64..=u64::MAX) => Difficulty::None,
                        None => Difficulty::None,
                    };

                    // A vector of this level's creators
                    let records: Vec<Record> = match file
                        .get_mut("records")
                        .and_then(|c: &mut Value| c.as_array_mut())
                    {
                        Some(records) => records
                            .iter()
                            .filter_map(|record| {
                                let user = record.get("user")?.as_str()?.to_string();
                                let link = record.get("link")?.as_str()?.to_string();
                                let percent = record.get("percent")?.as_i64()? as i8;
                                let hz = record.get("hz")?.as_i64()? as i16;
                                let mobile = record.get("mobile")?.as_bool()?;
                                let enjoyment = record
                                    .get("enjoyment")
                                    .and_then(|e| e.as_i64())
                                    .map(|e| e as i8);
                                Some(Record {
                                    user,
                                    link,
                                    percent,
                                    hz,
                                    mobile,
                                    enjoyment,
                                })
                            })
                            .collect(),
                        None => vec![],
                    };

                    let file: Level = Level {
                        id,
                        name,
                        creators,
                        verifier,
                        verification,
                        percent_to_qualify,
                        song_name,
                        song_link,
                        difficulty,
                        records,
                    };

                    if file.id != 0 {
                        list.push(file);
                    }
                }
            }
        }
    }
    return list;
}
