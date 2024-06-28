use utils::read_input;

fn part_one(input: &str) {
    let mut x = 0;
    let mut y = 0;

    for item in input.lines() {
        let mut item = item.split_whitespace();
        let action = item.next().unwrap();
        let val = item.next().unwrap().parse::<i32>().unwrap();

        match action {
            "forward" => {
                x += val;
            }
            "up" => {
                y -= val;
            }
            "down" => {
                y += val;
            }
            _ => println!("fucked"),
        };
    }

    let res = x * y;
    println!("02 part1: {}", res);
}

fn part_two(input: &str) {
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;

    for item in input.lines() {
        let mut item = item.split_whitespace();
        let action = item.next().unwrap();
        let val = item.next().unwrap().parse::<i32>().unwrap();

        match action {
            "forward" => {
                x += val;
                y += a * val;
            }
            "up" => {
                a -= val;
            }
            "down" => {
                a += val;
            }
            _ => println!("fucked"),
        };
    }

    let res = x * y;
    println!("02 part2: {}", res);
}

fn main() {
    let input = read_input("./inputs/2");

    part_one(&input);
    part_two(&input);
}
