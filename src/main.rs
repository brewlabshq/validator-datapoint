use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufRead, BufReader, Error},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_path: String,
}

#[derive(Serialize)]
struct DataPoints {
    unique_datapoints: Vec<String>,
}

fn main() -> Result<(), Error> {
    let config_str = fs::read_to_string("config.toml").expect("Failed to load config");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config");

    println!("Config loaded successfully");
    println!("Log path: {}", config.log_path);
    let log_file = File::open(&config.log_path).expect("Failed to create log file");
    let reader = BufReader::new(log_file);
    let mut unique_datapoints = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(dp_name) = extract_datapoint_name(&line) {
            println!("Found Datapoint: {:?}", dp_name);
            unique_datapoints.insert(dp_name.to_string());
        }
    }

    let datapoints_vec: Vec<String> = unique_datapoints.into_iter().collect();

    let output = DataPoints {
        unique_datapoints: datapoints_vec,
    };

    let out_file = File::create("unique_datapoints.json")?;
    to_writer_pretty(out_file, &output)?;

    println!("Extracted datapoints written to unique_datapoints.json");

    Ok(())
}

fn extract_datapoint_name(line: &str) -> Option<&str> {
    line.split("datapoint: ")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.split(',').next())
}
