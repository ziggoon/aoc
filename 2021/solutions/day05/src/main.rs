use std::collections::HashMap;
use utils::read_input;

fn part_one(input: &str) {
    // computer science?
    let mut grid = HashMap::new();

    for line in input.lines() {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = coords[0].split(',').map(|x| x.parse().unwrap()).collect();
        let end: Vec<i32> = coords[1].split(',').map(|x| x.parse().unwrap()).collect();

        let (x1, y1) = (start[0], start[1]);
        let (x2, y2) = (end[0], end[1]);

        if x1 == x2 {
            // iterate through each line, adding coord and intersect count to hashmap
            let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in y_min..=y_max {
                *grid.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in x_min..=x_max {
                *grid.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    let res = grid.values().filter(|&&count| count >= 2).count();
    println!("05 part1: {}", res);
}

fn part_two(input: &str) {
    // computer science?
    let mut grid = HashMap::new();

    for line in input.lines() {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = coords[0].split(',').map(|x| x.parse().unwrap()).collect();
        let end: Vec<i32> = coords[1].split(',').map(|x| x.parse().unwrap()).collect();

        let (x1, y1) = (start[0], start[1]);
        let (x2, y2) = (end[0], end[1]);

        if x1 == x2 {
            // iterate through each line, adding coord and intersect count to hashmap
            let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in y_min..=y_max {
                *grid.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in x_min..=x_max {
                *grid.entry((x, y1)).or_insert(0) += 1;
            }
        } else {
            // if slope = 1 OR -1 do shit
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            loop {
                *grid.entry((x, y)).or_insert(0) += 1;
                if x == x2 {
                    break;
                }
                x += x_step;
                y += y_step;
            }
        }
    }

    let res = grid.values().filter(|&&count| count >= 2).count();
    println!("05 part2: {}", res);
}

fn main() {
    let input = read_input("./inputs/5");

    part_one(&input);
    part_two(&input);
}
