use utils::read_input;

use std::collections::VecDeque;

fn part_one(input: &str, days: i32) {
    let fishies: Vec<u8> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    // so i was originally keeping track of ALL the fishies, meaning my vector would grow to be
    // !!massive!! thus murdering my poor little cpu. claude told me that i can store the count of
    // each fishy based on their remaining days until mitosis
    //
    // idk how many orders of magnitude this helps but i like it. i think this is my best aoc
    // solution so far
    let mut fishy_counts = VecDeque::from(vec![0; 9]);
    for fish in fishies {
        fishy_counts[fish as usize] += 1;
    }

    println!("initial state: {:?}", fishy_counts);

    for _ in 0..days {
        let spawners = fishy_counts.pop_front().unwrap();
        fishy_counts[6] += spawners;
        fishy_counts.push_back(spawners);
    }

    let res: u64 = fishy_counts.iter().sum();
    println!("05 part1: {}", res);
}

fn main() {
    let input = read_input("./inputs/6");

    part_one(&input, 80);
    part_one(&input, 256);
}
