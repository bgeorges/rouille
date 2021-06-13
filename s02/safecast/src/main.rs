use chrono::{DateTime, Utc};
use std::env;
use std::fs;

// Process Safecast logs using their APIs 
// We will use the following log as an example:
//          https://api.safecast.org/en-US/bgeigie_imports/35932

fn main() {
    println!("Safecast Log Processor");
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args);
    println!("{} file {}", cfg.action, cfg.filename);

    let contents = fs::read_to_string(cfg.filename).expect("Error reading file");
    if cfg.action == "raw" {
        print_raw_log(contents);
    } else if cfg.action == "decode" {
        println!("Nothing to do")
    } else {
        println!("unknown action")
    }
}

fn print_raw_log(data: String) {
    println!("Printing raw log content:\n{}", data);
}

#[allow(dead_code)]

struct Config {
    action: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let action = args[1].clone();
        let filename = args[2].clone();

        Config { action, filename }
    }
}

/**
 *
* Measurements
* The Measurement is the basic unit in the Safecast database.
* A measurement is composed of four key parts.
*    Timestamp captured_at
*    Unit unit
*    Value value
*    Geopoint location (latitude, longitude)

 */

struct Measurements {
    captured_at: DateTime<Utc>,
    unit: String,
    value: String,
    location: Geopoint,
}

struct Geopoint {
    longitude: f64,
    latitude: f64,
}
