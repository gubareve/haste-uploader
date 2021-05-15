use clap::{App, Arg};
use colored::*;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct HasteResponse {
    key: String,
}

fn main() {
    let matches = App::new("Haste Uploader")
        .version("0.1.0")
        .author("Evan Gubarev <evan@evangubarev.com>")
        .about("Makes uploading files to hastebin super easy!")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File location")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_location = matches.value_of("file").unwrap();
    let haste_location = "https://h.evang.dev";

    let file_contents: String = match fs::read_to_string(file_location) {
        Err(_e) => {
            println!(
                "{}",
                format!("🚨 {} 🚨", "Could not find file".red().bold())
            );
            std::process::exit(1)
        }
        Ok(output) => output,
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!("{}/documents", haste_location))
        .body(file_contents)
        .send();

    let result_json = match res {
        Err(e) => {
            println!(
                "{}",
                format!("🚨 {} 🚨\n {}", "Got server error".red().bold(), e)
            );
            std::process::exit(1)
        }
        Ok(output) => match output.text() {
            Err(_e) => String::new(),
            Ok(e) => e,
        },
    };

    let result_struct: HasteResponse = serde_json::from_str(&String::from(result_json)).unwrap();

    let result_code = result_struct.key;

    println!("{}/{}", haste_location, result_code);
}
