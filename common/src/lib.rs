use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct RootPathError(String);

impl Display for RootPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to get to root dir: {}", self.0)
    }
}
impl Error for RootPathError {}

pub fn read_file_as_string(path: &str) -> Result<String, Box<dyn Error>> {
    let path = get_path_from_root(path)?;
    let input = fs::read_to_string(path)?;
    Ok(input.trim().to_string())
}

pub fn read_file_as_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = get_path_from_root(path)?;
    Ok(fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.to_string())
        .collect())
}

pub fn read_file_as_elements<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let path = get_path_from_root(path)?;
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse()
            .map_err(|e| Box::<dyn Error>::from(format!("{:?}", e))))
        .collect()
}

fn get_path_from_root(path: &str) -> Result<PathBuf, RootPathError> {
    match Path::new(env!("CARGO_MANIFEST_DIR")).parent() {
        None => Err(RootPathError("Unable to get to root dir".to_string())),
        Some(parent) => Ok(parent.join(path)),
    }
}
