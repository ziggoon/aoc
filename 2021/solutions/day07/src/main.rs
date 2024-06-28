use utils::read_input;

fn part_one(input: &str) {
    let mut crabs: Vec<i32> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    crabs.sort();

    let median = crabs[crabs.len() / 2];
    let res: i32 = crabs.iter().map(|&x| (x - median).abs()).sum();

    println!("07 part1: {}", res);
}

fn part_two(input: &str) {
    let crabs: Vec<i32> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    let sum: i32 = crabs.iter().sum();

    // i was very confused why the mean was working for test case, but not the actual input..
    // apparently rounding the mean is chill in this case. not sure why.
    //let mean = (sum as f64 / crabs.len() as f64).round() as i32;
    let mean = sum / crabs.len() as i32;

    let avg_cost: i32 = crabs
        .iter()
        .map(|&x| {
            let diff = (x - mean).abs();
            (diff * (diff + 1)) / 2
        })
        .sum();

    println!("07 part2: {}", avg_cost);
}

fn main() {
    let input = read_input("./inputs/7");

    part_one(&input);
    part_two(&input);
}
