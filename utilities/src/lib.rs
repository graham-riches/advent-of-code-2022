#![allow(dead_code)]

use std::{
   fs::File,
   io::{self, BufRead, BufReader},
   path::Path,
};

/// Read the lines from a file into a vector of strings
/// 
/// # Arguments
/// * `filename` - The path or filename to read from
/// 
/// # Examples
/// ```no_run
/// let lines = utilities::lines_from_file("sample_path.txt");
/// ```
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
   BufReader::new(File::open(filename)?).lines().collect()
}
