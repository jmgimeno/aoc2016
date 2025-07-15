use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_file_as_string(path: &str) -> String {
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");
    input.trim().to_string()
}

pub fn read_file_as_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn read_file_as_elements<T>(path: &str) -> Vec<T>
 where T: FromStr, <T as FromStr>::Err: Debug {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}
