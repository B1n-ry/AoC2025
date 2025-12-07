use std::collections::HashMap;

use crate::utils::{self, Grid};

pub fn solve() {
    println!("Day 07");
    let input = utils::read_input(7);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn handle_ray_at(grid: &mut Grid<char>, x: usize, y: usize) -> usize {
    let here = grid.get(x, y);
    let mut splits = 0;

    match here {
        Some(&'.') => { grid.set(x, y, '|'); },
        Some(&'^') => {
            let (left, right) = (x.checked_sub(1), x.checked_add(1));
            [left, right].iter().try_for_each(|&x| grid.set(x?, y, '|'));
            splits += 1;
        },
        _ => (),
    }

    splits
}

fn solve_part1(input: &str) -> String {
    let mut grid = input.parse::<Grid<char>>().expect("Failed to parse as grid");
    let mut splits = 0;

    for y in 1..grid.height {  // Top row does not matter based on incoming rays
        for x in 0..grid.width {
            let above = y.checked_sub(1).map(|y| grid.get(x, y)).flatten();
            if above.is_some_and(|above| ['S', '|'].contains(&above)) {
                splits += handle_ray_at(&mut grid, x, y);
            }
        }
    }
    format!("The ray is being split {} times", splits)
}

fn get_splits_from(x: usize, y: usize, grid: &Grid<char>, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if !(0..grid.width).contains(&x) || !(0..grid.height).contains(&y) {
        return 1;
    }
    if let Some(&memoized_splits) = memo.get(&(x, y)) {
        return memoized_splits;
    }
    let cell = grid.get(x, y);
    let splits = if cell == Some(&'^') {
        let (left, right) = (x.checked_sub(1), x.checked_add(1));
        [left, right].iter().filter_map(|&x| {
            Some(get_splits_from(x?, y, grid, memo))
        }).sum()
    } else if cell.is_some() {
        y.checked_add(1).map_or(0, |y| get_splits_from(x, y, grid, memo))
    } else {
        1
    };
    memo.insert((x, y), splits);
    
    splits
}

fn solve_part2(input: &str) -> String {
    let grid = input.parse::<Grid<char>>().expect("Could not parse grid");
    let (start_x, start_y, _) = grid.iter().find(|(_, _, &val)| val == 'S').expect("Did not find a starting point");
    let mut memo_table: HashMap<(usize, usize), usize> = HashMap::new();

    let splits = get_splits_from(start_x, start_y, &grid, &mut memo_table);

    format!("One tachyon creates {} universes", splits)
}
