use std::collections::HashMap;

use crate::utils;

pub fn solve() {
    println!("Day 06");
    let input = utils::read_input(6);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> String {
    let mut sum: usize = 0;
    let mut columns: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        if words.contains(&"+") || words.contains(&"*") {
            for (i, &word) in words.iter().enumerate() {
                let col = columns.get(&i).expect("Error: Unmatched key");
                match word {
                    "+" => sum += col.iter().sum::<usize>(),
                    "*" => sum += col.iter().product::<usize>(),
                    _ => panic!("Malformed input: Line with mult/add signs contained other words"),
                };
            }
            break;
        }

        for (i, num) in words.iter().map(|word| word.parse::<usize>().expect("Malformed input: Word NaN")).enumerate() {
            columns.entry(i).or_insert(Vec::new()).push(num);
        }
    }
    format!("Total sum: {}", sum)
}

fn solve_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.windows(2).all(|w| w[0].len() == w[1].len()));  // Make sure all rows are of equal length

    let length = lines.get(0).map_or(0, |l| l.len());
    let mut batch: Vec<usize> = Vec::new();
    let mut sum = 0;
    for i in (0..length).rev() {
        let number_string = lines.iter().filter_map(|line| {
            line.chars().nth(i).filter(|c| c.is_ascii_digit() || ['*', '+'].contains(c))
        }).collect::<String>();
        if number_string.is_empty() {
            batch.clear();
            continue;
        }

        let sign = number_string.chars().last().filter(|c| ['*', '+'].contains(c));
        let digit: usize = if sign.is_some() {
            number_string[..(number_string.len() - 1)].parse().expect("Malformed input: NaN")
        } else {
            number_string.parse().expect("Malformed input: NaN")
        };
        batch.push(digit);

        match sign {
            Some('+') => sum += batch.iter().sum::<usize>(),
            Some('*') => sum += batch.iter().product::<usize>(),
            None => (),
            _ => panic!("'{}' ???", sign.unwrap_or('-')),
        }
    }

    format!("New sum is {}", sum)
}
