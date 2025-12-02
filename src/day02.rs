use std::ops::Div;

use crate::utils;

pub fn solve() {
    println!("Day 02");
    let input = utils::read_input(2);
    
    // Part 1
    let part1 = solve_problem(&input, repeats_p1);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_problem(&input, repeats_p2);
    println!("Part 2: {}", part2);
}

fn solve_problem(input: &str, repeat_identification_fn: fn(usize) -> bool) -> String {
    let sum = input.split(",").map(|range| {
        let (from, to) = range.split_once("-").expect("Malformed input: Found no hyphen (-)");
        let mut it_num = from.parse().expect("Malformed: Start in range NaN");
        let end = to.parse().expect("Malformed: End in range NaN");

        let mut sum = 0;
        while it_num <= end {
            if repeat_identification_fn(it_num) {
                sum += it_num;
            }
            it_num += 1;
        }
        
        sum
    }).sum::<usize>();

    format!("Sum of invalid IDs is {}", sum)
}

fn repeats_p1(number: usize) -> bool {
    let s = format!("{}", number);
    let (s1, s2) = s.split_at(s.len() / 2);
    s1 == s2
}

fn repeats_p2(number: usize) -> bool {
    let s = format!("{}", number);
    (1..=s.len().div(2)).any(|i| {
        s.as_bytes().chunks(i).collect::<Vec<&[u8]>>().windows(2).all(|l| l[0] == l[1])
    })
}
