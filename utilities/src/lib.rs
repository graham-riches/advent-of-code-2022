#![allow(dead_code)]

use std::{
   fs::File,
   io::{self, BufRead, BufReader},
   path::Path,
};

use itertools::Itertools;

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


/// Parse a tuple pair out of a string. Pairs can be any parseable 
/// value separated by a delimiter
/// 
/// # Arguments
/// * `s` - The string to parse the tuple out of
/// * `delimiter` - The delimiter separating the tuple
/// 
/// # Examples
/// ```no_run
/// assert_eq!(utilities::parse_pair::<i32>("1,2", ","), Some((1, 2)));
/// ```
/// 
pub fn parse_pair<T>(s: &str, delimiter: &str) -> Option<(T, T)>
where
    T: std::str::FromStr
{
    s.split(delimiter)
     .flat_map(|x| x.parse::<T>().ok())
     .next_tuple()
}
