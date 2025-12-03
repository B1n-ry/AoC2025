use crate::utils;

pub fn solve() {
    println!("Day 03");
    let input = utils::read_input(3);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> String {
    let sum = input.lines().map(|bank| {
        let jolts: Vec<u32> = bank
            .chars()
            .map(|c| c.to_digit(10).expect("Malformed input: Char not a digit"))
            .collect();
        let (Some(i), Some(highest)) = jolts
            .iter()
            .take(jolts.len() - 1)
            .enumerate()
            .fold((None, None), |(idx, max), (i, &curr)| {
                if max < Some(curr) {
                    (Some(i), Some(curr))
                } else {
                    (idx, max)
                }
        }) else {
            panic!("Bank empty?");
        };
        let &next_high = jolts[(i + 1)..].iter().max().expect("Could not find 2nd highest");
        format!("{}{}", highest, next_high).parse::<u32>().expect("Could not parse digits")
    }).sum::<u32>();
    format!("Total jolts used: {}", sum)
}

fn solve_part2(input: &str) -> String {
    // TODO: Implement part 2 solution
    format!("Not implemented yet. Input has {} characters.", input.len())
}
