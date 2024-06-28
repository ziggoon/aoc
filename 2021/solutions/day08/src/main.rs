use utils::read_input;

use std::collections::HashMap;

fn part_one(input: &str) {
    let mut res = 0;
    for line in input.lines() {
        let parts = line.split(" | ").collect::<Vec<&str>>();

        let outputs = parts[1].split(' ').collect::<Vec<&str>>();

        outputs.iter().for_each(|&x| match x.len() {
            2 | 3 | 4 | 7 => res += 1,
            _ => {}
        });
    }

    println!("08 part1: {}", res);
}

// i got eepy and dont want to finish this hehe
fn part_two(input: &str) {
    let mut res = 0;

    let display_map = HashMap::from([
        (0, vec![0, 1, 2, 3, 4]),
        (1, vec![1, 2]),
        (2, vec![0, 1, 3, 4, 6]),
        (3, vec![0, 1, 2, 3, 6]),
        (4, vec![1, 2, 5, 6]),
        (5, vec![0, 2, 3, 5, 6]),
        (6, vec![0, 2, 3, 4, 5, 6]),
        (7, vec![0, 1, 2]),
        (8, vec![0, 1, 2, 3, 4, 5, 6]),
        (9, vec![0, 1, 2, 3, 5, 6]),
    ]);

    for line in input.lines() {
        let mut positions = [' '; 7];
        let mut digit_map = HashMap::<u8, &str>::new();

        let parts = line.split(" | ").collect::<Vec<&str>>();
        let signal_patterns = parts[0].split(' ').collect::<Vec<&str>>();
        let outputs = parts[1].split(' ').collect::<Vec<&str>>();

        signal_patterns.iter().for_each(|&x| match x.len() {
            2 => {
                digit_map.insert(1, x);
            }
            3 => {
                digit_map.insert(7, x);
            }
            4 => {
                digit_map.insert(4, x);
            }
            7 => {
                digit_map.insert(8, x);
            }
            _ => {}
        });

        signal_patterns
            .iter()
            .for_each(|&pattern| match pattern.len() {
                6 => {
                    if digit_map[&4].chars().all(|c| pattern.contains(c)) {
                        digit_map.insert(9, pattern);
                    } else if digit_map[&1].chars().all(|c| pattern.contains(c)) {
                        digit_map.insert(0, pattern);
                    } else {
                        digit_map.insert(6, pattern);
                    }
                }
                5 => {
                    if digit_map[&1].chars().all(|c| pattern.contains(c)) {
                        digit_map.insert(3, pattern);
                    } else if pattern.chars().all(|c| digit_map[&9].contains(c)) {
                        digit_map.insert(5, pattern);
                    } else {
                        digit_map.insert(2, pattern);
                    }
                }
                _ => {}
            });
    }

    println!("08 part2: {}", res);
}

fn main() {
    let input = read_input("./inputs/8.test");

    part_one(&input);
    part_two(&input);
}
