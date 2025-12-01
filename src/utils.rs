use std::fs;
use std::path::Path;

/// Reads the input file for a given day and returns its contents as a String.
pub fn read_input(day: u32) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    let path = Path::new(&filename);
    
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!(
            "Failed to read input file '{}'. Please ensure the file exists and contains your puzzle input. Error: {}",
            filename, err
        ))
}

/// Reads the input file for a given day and returns it as a vector of lines.
pub fn read_lines(day: u32) -> Vec<String> {
    read_input(day)
        .lines()
        .map(String::from)
        .collect()
}

/// Reads the input file for a given day and parses each line as the specified type.
pub fn read_parsed<T>(day: u32) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_lines(day)
        .iter()
        .map(|line| line.parse::<T>().unwrap_or_else(|err| panic!(
            "Failed to parse line '{}'. Error: {:?}",
            line, err
        )))
        .collect()
}
