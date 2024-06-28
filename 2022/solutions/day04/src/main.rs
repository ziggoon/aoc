use utils::read_input;

fn part_one(input: &str) {
    let mut total = 0;

    for line in input.lines() {
        let ranges: Vec<&str> = line.split(',').collect();

        let range1: Vec<u32> = ranges[0].split('-').map(|x| x.parse().unwrap()).collect();
        let range2: Vec<u32> = ranges[1].split('-').map(|x| x.parse().unwrap()).collect();

        if (range1[0] >= range2[0] && range1[1] <= range2[1])
            || (range2[0] >= range1[0] && range2[1] <= range1[1])
        {
            total += 1;
        }
    }
    println!("04 part1: {}", total);
}

fn part_two(input: &str) {
    let mut total: u32 = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<u32> = ranges[0].split('-').map(|x| x.parse().unwrap()).collect();
        let range2: Vec<u32> = ranges[1].split('-').map(|x| x.parse().unwrap()).collect();

        if range1[1] >= range2[0] && range1[0] <= range2[1] {
            total += 1;
        }
    }

    println!("{}", total);
}

fn main() {
    let input = read_input("./inputs/4");

    part_one(&input);
    part_two(&input);
}
