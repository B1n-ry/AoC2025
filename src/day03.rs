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

fn find_max_indexed<T: PartialOrd>(num_array: &[T]) -> Option<(usize, &T)> {
    let (Some(i), Some(highest)) = num_array
        .iter()
        .enumerate()
        .fold((None, None), |(idx, max), (i, curr)| {
            if max < Some(curr) {
                (Some(i), Some(curr))
            } else {
                (idx, max)
            }
    }) else {
        return None;
    };
    Some((i, highest))
}

fn solve_part1(input: &str) -> String {
    let sum = input.lines().map(|bank| {
        let jolts: Vec<u32> = bank
            .chars()
            .map(|c| c.to_digit(10).expect("Malformed input: Char not a digit"))
            .collect();
        let (i, &highest) = find_max_indexed(&jolts[..(jolts.len() - 1)]).expect("Bank empty?");
        
        let &next_high = jolts[(i + 1)..].iter().max().expect("Could not find 2nd highest");
        format!("{}{}", highest, next_high).parse::<u32>().expect("Could not parse digits")
    }).sum::<u32>();
    format!("Total jolts used: {}", sum)
}

fn solve_part2(input: &str) -> String {
    let prob_2_n: usize = 12;
    let sum = input.lines().map(|bank| {
        let jolts: Vec<u32> = bank
            .chars()
            .map(|c| c.to_digit(10).expect("Malformed input: Char not a digit"))
            .collect();

        let mut next_idx = 0;
        let activations: Vec<u32> = (1..=prob_2_n).map(|i| {
            let sub_vec = &jolts[next_idx..(jolts.len() - (prob_2_n - i))];
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
