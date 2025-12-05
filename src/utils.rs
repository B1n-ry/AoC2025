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
    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}
impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }

        let result = self.grid.get(self.x, self.y).map(|value| (self.x, self.y, value));

        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        result
    }
}
