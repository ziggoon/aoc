use utils::read_input;

fn part_one(input: &str) {
    let mut prev = 0;
    let mut total = 0;

    for item in input.lines() {
        let item = item.parse::<i32>().unwrap();
        if prev != 0 && item > prev {
            total += 1
        }
        prev = item;
    }

    println!("01 part1: {}", total);
}

fn part_two(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0;
    let mut prev_sum = 0;
    let mut total = 0;
    while i < lines.len() {
        if i + 2 >= lines.len() {
            break;
        }

        let v1 = lines[i].trim().parse::<i32>().unwrap();
        let v2 = lines[i + 1].trim().parse::<i32>().unwrap();
        let v3 = lines[i + 2].trim().parse::<i32>().unwrap();
        let sum = v1 + v2 + v3;

        if prev_sum != 0 && sum > prev_sum {
            total += 1;
        }

        prev_sum = sum;
        i += 1;
    }

    println!("01 part2: {}", total);
}

fn main() {
    let input = read_input("./inputs/1");

    part_one(&input);
    part_two(&input);
}
