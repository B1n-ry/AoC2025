use crate::utils;

pub fn solve() {
    println!("Day 03");
    let input = utils::read_input(3);
    
    // Part 1
    let part1 = solve_part(&input, 2);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part(&input, 12);
    println!("Part 2: {}", part2);
}

fn find_max_indexed<T: PartialOrd>(num_array: &[T]) -> Option<(usize, &T)> {
    let (i, highest) = num_array
        .iter()
        .enumerate()
        .fold((None, None), |(idx, max), (i, curr)| {
            if max < Some(curr) {
                (Some(i), Some(curr))
            } else {
                (idx, max)
            }
    });
    i.zip(highest)
}

fn solve_part(input: &str, battery_count: usize) -> String {
    let sum = input.lines().map(|bank| {
        let jolts: Vec<u32> = bank
            .chars()
            .map(|c| c.to_digit(10).expect("Malformed input: Char not a digit"))
            .collect();

        let mut next_idx = 0;
        let activations: Vec<u32> = (1..=battery_count).map(|i| {
            let sub_vec = &jolts[next_idx..(jolts.len() - (battery_count - i))];
            let (i, &highest) = find_max_indexed(sub_vec).expect("Bank empty?");
            next_idx += i + 1;
            highest
        }).collect();
        activations
            .iter()
            .map(|&i| char::from_digit(i, 10).expect("Can't turn back into char"))
            .collect::<String>()
            .parse::<usize>()
            .expect("Can't turn number string into number")
    }).sum::<usize>();
    format!("Total jolts used: {}", sum)
}
