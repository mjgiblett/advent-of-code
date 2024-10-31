use std::fs;
use std::io;

mod day01;

fn main() {
    println!("--- Advent of Code 2023 ---\n");
    loop {
        let puzzle_count: u32 = fs::read_dir("./src")
            .into_iter()
            .count()
            .try_into()
            .unwrap();

        println!("Select which puzzle to solve (1 - {puzzle_count}): ");
        let mut puzzle = String::new();
        io::stdin()
            .read_line(&mut puzzle)
            .expect("Failed to read line");
        let puzzle: u32 = match puzzle.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result: u32;
        match puzzle {
            11 => result = day01::p1(),
            12 => result = day01::p2(),
            _ => continue,
        }
        println!("{result}");
        break;
    }
}
