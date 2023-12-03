use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_digits(line: &str) -> Vec<u32> {
    line.chars().filter_map(|a| a.to_digit(10)).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn concat(vec: &[u32]) -> u32 {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    acc
}

pub fn p1() -> u32 {
    println!("--- Day 1: Trebuchet?! Part 1 ---");
    let mut calibration: u32 = 0;
    if let Ok(lines) = read_lines("./input/day01.txt") {
        for line in lines {
            if let Ok(text) = line {
                let digits: Vec<u32> = parse_digits(&text);
                let value: u32 = concat(&[
                    *digits.first().unwrap_or(&0u32),
                    *digits.last().unwrap_or(&0u32),
                ]);
                calibration += value
            }
        }
    }
    calibration
}
