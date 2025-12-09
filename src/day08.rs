use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use crate::utils;

pub fn solve() {
    println!("Day 08");
    let input = utils::read_input(8);
    
    // Part 1
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    // Part 2
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

#[derive(Eq, PartialEq)]
struct PosPair {
    pos1: (usize, usize, usize),
    pos2: (usize, usize, usize),
    id1: usize,
    id2: usize,
}
impl PosPair {
    fn square_dist(&self) -> usize {
        [
            self.pos1.0.abs_diff(self.pos2.0),
            self.pos1.1.abs_diff(self.pos2.1),
            self.pos1.2.abs_diff(self.pos2.2)
        ].map(|i| i.pow(2)).iter().sum()
    }
}
impl Ord for PosPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.square_dist().cmp(&other.square_dist()).reverse()
    }
}
impl PartialOrd for PosPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part1(input: &str) -> String {
    let mut pairs: BinaryHeap<PosPair> = BinaryHeap::new();
    let coords: Vec<(usize, usize, usize)> = input.lines().map(|line| {
        let coords: Vec<usize> = line
            .split(',')
            .map(|n| n.parse::<usize>().expect("NaN"))
            .collect();
        (coords[0], coords[1], coords[2])
    }).collect();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            pairs.push(PosPair { pos1: coords[i], pos2: coords[j], id1: i, id2: j });
        }
    }
    
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..1000 {
        let Some(pair) = pairs.pop() else {
            break;
        };
        let ids = [pair.id1, pair.id2];
        if circuits.iter().any(|circuit| {
            ids.iter().all(|id| circuit.contains(id))
        }) {
            continue;
        }
        let mut matching_circuit_indexes: Vec<usize> = ids.iter().filter_map(|id| {
            circuits.iter().position(|circuit| circuit.contains(id))
        }).collect();
        matching_circuit_indexes.sort_by(|a, b| b.cmp(a));
        
        assert!(matching_circuit_indexes.windows(2).all(|w| w[0] >= w[1]));
        let mut circuit: HashSet<usize> = matching_circuit_indexes.iter().flat_map(|&idx| {
            let c = circuits[idx].clone();
            circuits.remove(idx);
            c
        }).collect();
        for id in ids {
            circuit.insert(id);
        }
        circuits.push(circuit);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let product = circuits.iter().take(3).map(|hash_set| hash_set.len()).product::<usize>();
    format!("Product: {}", product)
}

fn solve_part2(input: &str) -> String {
    let mut s = String::new();
    let mut pairs: BinaryHeap<PosPair> = BinaryHeap::new();
    let coords: Vec<(usize, usize, usize)> = input.lines().map(|line| {
        let coords: Vec<usize> = line
            .split(',')
            .map(|n| n.parse::<usize>().expect("NaN"))
            .collect();
        (coords[0], coords[1], coords[2])
    }).collect();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            pairs.push(PosPair { pos1: coords[i], pos2: coords[j], id1: i, id2: j });
        }
    }
    
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    while let Some(pair) = pairs.pop() {
        let ids = [pair.id1, pair.id2];
        if circuits.iter().any(|circuit| {
            ids.iter().all(|id| circuit.contains(id))
        }) {
            continue;
        }
        let mut matching_circuit_indexes: Vec<usize> = ids.iter().filter_map(|id| {
            circuits.iter().position(|circuit| circuit.contains(id))
        }).collect();
        matching_circuit_indexes.sort_by(|a, b| b.cmp(a));
        
        assert!(matching_circuit_indexes.windows(2).all(|w| w[0] >= w[1]));
        let mut circuit: HashSet<usize> = matching_circuit_indexes.iter().flat_map(|&idx| {
            let c = circuits[idx].clone();
            circuits.remove(idx);
            c
        }).collect();
        for id in ids {
            circuit.insert(id);
        }
        circuits.push(circuit);
        if circuits.len() == 1 {
            s = format!("{} * {} = {}", pair.pos1.0, pair.pos2.0, pair.pos1.0 * pair.pos2.0);
        }
    }
    s
}
