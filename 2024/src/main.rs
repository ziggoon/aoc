use aoc::solutions::{day01};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Day 1:");
    println!("Part 1: {:?}", day01::part1());
    println!("Part 2: {:?}", day01::part2());

    println!("\nTotal time: {:?}", start.elapsed());
}