use std::{thread::sleep, time::Duration};

use crate::utils::{self, Grid};

pub fn solve() {
    println!("Day 04");
    let input = utils::read_input(4);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn get_neighbour_rolls(grid: &Grid<char>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbour_pos = Vec::new();
    for x_i in x.saturating_sub(1)..=x.saturating_add(1) {
        for y_i in y.saturating_sub(1)..=y.saturating_add(1) {
            if x_i == x && y_i == y {
                continue;
            }
            if grid.get(x_i, y_i) == Some(&'@') {
                neighbour_pos.push((x_i, y_i));
            }
        }
    }
    neighbour_pos
}

fn get_neighbour_lists(grid: &Grid<char>) -> Vec<(usize, usize, Vec<(usize, usize)>)> {
    grid.iter().filter_map(|(x, y, &item)| {
        if item != '@' { return None; }

        let neighbour_rolls = get_neighbour_rolls(&grid, x, y);

        if neighbour_rolls.len() < 4 {
            Some((x, y, neighbour_rolls))
        } else {
            None
        }
    }).collect()
}

fn solve_part1(input: &str) -> String {
    let grid: Grid<char> = input.parse().expect("Not proper grid");
    let accessible = get_neighbour_lists(&grid);

    format!("There are {} accessible paper rolls", accessible.len())
}

fn solve_part2(input: &str) -> String {
    let mut removed = 0;
    let mut grid: Grid<char> = input.parse().expect("Not proper grid");

    let mut remove = get_neighbour_lists(&grid);
    let mut checkable = Vec::new();
    loop {
        if remove.len() == 0 {
            break;
        }
        let mut removed_poses = Vec::new();
        let mut remove_count = 0;
        for (x, y, next) in &remove {
            if !removed_poses.contains(&(*x, *y)) {
                remove_count += 1;
                removed_poses.push((*x, *y));
                grid.set(*x, *y, 'x');
            }
            for &pos in next {
                if !checkable.contains(&pos) {
                    checkable.push(pos);
                }
            }
        }
        removed += remove_count;
        remove.clear();
        for &(x, y) in &checkable {
            let rolls = get_neighbour_rolls(&grid, x, y);
            if rolls.len() < 4 && grid.get(x, y) == Some(&'@') {
                remove.push((x, y, rolls));
            }
        }
        checkable.clear();
    }

    
    format!("There are {} paper rolls that can be removed", removed)
}
