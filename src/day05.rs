use std::ops::RangeInclusive;

use crate::utils;

pub fn solve() {
    println!("Day 05");
    let input = utils::read_input(5);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> String {
    let ranges: Vec<RangeInclusive<usize>> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (from, to) = line.split_once("-").expect("Malformed input: Ranges missing hyphen");
            from.parse::<usize>().expect("Range FROM NaN")..=to.parse::<usize>().expect("Range TO NaN")
    }).collect();
    let fresh: usize = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|ingredient| {
            let id = ingredient.parse::<usize>().expect("Malformed input: Ingredient ID NaN");
            if ranges.iter().any(|range| range.contains(&id)) {
                1
            } else {
                0
            }
        }).sum();
    format!("There are {} fresh ingredients used", fresh)
}

fn solve_part2(input: &str) -> String {
    let mut input_ranges: Vec<(usize, usize)> = input.lines().map_while(|line| {
        let (from, to) = line.split_once("-")?;
        Some((
            from.parse::<usize>().expect("Range FROM NaN"),
            to.parse::<usize>().expect("Range TO NaN")
        ))
    }).collect();
    // If we sort we avoid cases where a later range is encapsulating a previous one
    input_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(input_ranges.windows(2).all(|v| v[0].0 <= v[1].0));

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for range in input_ranges {
        let (mut range_from, mut range_to) = range;
        
        while ranges.iter().any(|&(f, t)| (f..=t).contains(&range_from) || (f..=t).contains(&range_to)) {
            for &range in &ranges {
                let r = range.0..=range.1;
                if r.contains(&range_from) {
                    range_from = range.1.saturating_add(1)
                }
                if r.contains(&range_to) {
                    range_to = range.0.saturating_sub(1)
                }
            }
        }

        if range_from <= range_to {
            ranges.push((range_from, range_to));
        }
    }
    let fresh_count: usize = ranges.iter().map(|&(from, to)| {
        to - from + 1
    }).sum();
    format!("There is a total of {} fresh ingredient IDs", fresh_count)
}
