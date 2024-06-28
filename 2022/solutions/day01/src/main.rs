use utils::read_input;

fn part_one(input: &str) {
    let mut max = 0;
    let mut current: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    println!("total: {}", max);
}

fn part_two(input: &str) {
    let mut max = (0, 0, 0);
    let mut current = 0;

    for line in input.lines() {
        if !line.is_empty() {
            current += line.parse::<u32>().unwrap();
        } else {
            match max {
                (a, b, c) if current > c => {
                    if current < b {
                        max = (a, b, current);
                    } else if current < a {
                        max = (a, current, b);
                    } else {
                        max = (current, a, b);
                    }
                }
                _ => {}
            }

            current = 0;
        }
    }

    if current != 0 {
        match max {
            (a, b, c) if current > c => {
                if current < b {
                    max = (a, b, current);
                } else if current < a {
                    max = (a, current, b);
                } else {
                    max = (current, a, b);
                }
            }
            _ => {}
        }
    }
    println!("total: {}", max.0 + max.1 + max.2);
}

fn main() {
    let input = read_input("./inputs/1.test");

    part_one(&input);
    part_two(&input);
}
