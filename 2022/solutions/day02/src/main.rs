use utils::read_input;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

fn part_one(input: &str) {
    let mut total: u32 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        match (parts[0], parts[1]) {
            ("A", "X") => total += ROCK + DRAW,
            ("B", "X") => total += ROCK,
            ("C", "X") => total += ROCK + WIN,
            ("A", "Y") => total += PAPER + WIN,
            ("B", "Y") => total += PAPER + DRAW,
            ("C", "Y") => total += PAPER,
            ("A", "Z") => total += SCISSORS,
            ("B", "Z") => total += SCISSORS + WIN,
            ("C", "Z") => total += SCISSORS + DRAW,
            (_, _) => {}
        };
    }
    println!("02 part1: {}", total);
}

fn part_two(input: &str) {
    let mut total: u32 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        match (parts[0], parts[1]) {
            ("A", "X") => total += SCISSORS,
            ("A", "Y") => total += ROCK + DRAW,
            ("A", "Z") => total += PAPER + WIN,
            ("B", "X") => total += ROCK,
            ("B", "Y") => total += PAPER + DRAW,
            ("B", "Z") => total += SCISSORS + WIN,
            ("C", "X") => total += PAPER,
            ("C", "Y") => total += SCISSORS + DRAW,
            ("C", "Z") => total += ROCK + WIN,
            (_, _) => {}
        }
    }
    println!("02 part2: {}", total);
}

fn main() {
    let input = read_input("./inputs/2");

    part_one(&input);
    part_two(&input);
}
