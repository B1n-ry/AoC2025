use crate::utils;

pub fn solve() {
    println!("Day 01");
    let input = utils::read_input(1);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> String {
    let mut zero_counter = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction = line.chars().nth(0).expect("Malformed input");
        let direction: i16 = match direction {
            'L' => -1,
            'R' => 1,
            c => panic!("Incorrect character: {}", c)
        };
        let movement = line[1..].parse::<i16>().expect("Movement of dial not expressed as number");
        dial = (100 + dial + movement * direction) % 100;

        if dial == 0 {
            zero_counter += 1;
        }
    }

    format!("Solution to problem 1: {}", zero_counter)
}

fn solve_part2(input: &str) -> String {
    let mut zero_counter = 0;
    let mut dial = 50;
    for line in input.lines() {
        let direction = line.chars().nth(0).expect("Malformed input");
        let direction: i32 = match direction {
            'L' => -1,
            'R' => 1,
            c => panic!("Incorrect character: {}", c)
        };
        let movement = line[1..].parse::<i32>().expect("Movement of dial not expressed as number");

        if movement >= 100 {
            zero_counter += movement / 100;
        }
        let movement = movement % 100;
        if movement == 0 {
            continue;
        }

        let prev = dial;
        dial += movement * direction;

        if dial >= 100 || (dial <= 0 && prev != 0) {
            zero_counter += 1;
        }
        dial = (dial + 100) % 100;
    }

    format!("Solution to problem 1: {}", zero_counter)
}
