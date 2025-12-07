use core::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

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

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<T>>
}
impl FromStr for Grid<char> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid.get(0).map_or(0, |row| row.len());

        if grid.iter().any(|line| line.len() != width) {
            Err(String::from("Grid not uniform"))
        } else {
            Ok(Self {
                width,
                height,
                grid
            })
        }
    }
}
impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.grid
            .iter()
            .map(|row| row.iter().map(|c| format!("{}", c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y)?.get(x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(y)?.get_mut(x)
    }
    pub fn set(&mut self, x: usize, y: usize, new_value: T) -> Option<()> {
        *(self.get_mut(x, y)?) = new_value;
        Some(())
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, value)| (x, y, value))
            })
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.grid
            .iter_mut()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, value)| (x, y, value))
            })
    }
}
