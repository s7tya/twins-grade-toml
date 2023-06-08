extern crate csv;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).expect("Failed to open csv file.");

    let mut rdr = csv::Reader::from_reader(file);

    let mut buf = String::new();

    for result in rdr.records() {
        let record = result.expect("err");

        buf += &get_toml_string(
            &record[2],
            &record[3],
            record[4].trim().parse().unwrap(),
            &record[7] != "履修中",
        );
    }

    let mut file = fs::File::create("grades/grades.toml").unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

fn get_toml_string(id: &str, name: &str, credits: f64, get: bool) -> String {
    return format!(
        "[[class]]\n\
        id = \"{}\"\n\
        name = \"{}\"\n\
        credits = {}\n\
        {}\n\n\
    ",
        id,
        name,
        credits,
        if !get { "get = false" } else { "" }
    );
}
