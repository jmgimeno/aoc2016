use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn read_file_as_string(path: &str) -> String {
    let path = get_path_from_root(path);
    let input = fs::read_to_string(path).expect("Something went wrong reading the file");
    input.trim().to_string()
}

pub fn read_file_as_lines(path: &str) -> Vec<String> {
    let path = get_path_from_root(path);
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn read_file_as_elements<T>(path: &str) -> Vec<T>
 where T: FromStr, <T as FromStr>::Err: Debug {
    let path = get_path_from_root(path);
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_path_from_root(path: &str) -> String {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.parent().unwrap().join(path).to_str().unwrap().to_string()
}
