use utils::read_input;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part_one(input: &str) {
    let mut total: u32 = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);

        for letter in left.chars() {
            if right.contains(letter as char) {
                total += LETTERS.find(letter).unwrap() as u32 + 1;
                break;
            }
        }
    }

    println!("03 part1: {}", total);
}

fn part_two(input: &str) {
    let mut total: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0;

    loop {
        if i > lines.len() - 3 {
            break;
        }

        let (first, second, third) = (lines[i], lines[i + 1], lines[i + 2]);

        for letter in first.chars() {
            if second.contains(letter) && third.contains(letter) {
                total += LETTERS.find(letter).unwrap() as u32 + 1;
                break;
            }
        }
        i += 3;
    }

    println!("03 part2: {}", total);
}

fn main() {
    let input = read_input("./inputs/3.test");

    part_one(&input);
    part_two(&input);
}
