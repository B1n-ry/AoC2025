mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let day = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or_else(|_| {
            eprintln!("Invalid input '{}'. Please provide a valid day number (1-12)", args[1]);
            std::process::exit(1);
        })
    } else {
        println!("Usage: cargo run <day>");
        println!("Example: cargo run 1");
        return;
    };

    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => day09::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        _ => println!("Invalid day: {}. Please choose a day between 1 and 12.", day),
    }
}
