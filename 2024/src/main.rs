use aoc::solutions::{day01, day02, day03, day04};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Day 1:");
    println!("Part 1: {:?}", day01::part1());
    println!("Part 2: {:?}", day01::part2());

    println!("Day 2:");
    println!("Part 1: {:?}", day02::part1());
    println!("Part 2: {:?}", day02::part2());

    println!("Day 3:");
    println!("Part 1: {:?}", day03::part1());
    println!("Part 2: {:?}", day03::part2());

    println!("Day 4:");
    println!("Part 1: {:?}", day04::part1());
    println!("Part 2: {:?}", day04::part2());


    println!("\nTotal time: {:?}", start.elapsed());
}
