use utils::read_input;

fn part_one(input: &str) {
    let matrix: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // this is cool. in rust you can modify individual bits
    let mut gamma: u16 = 0b0000_0000_0000;
    let mut epsilon: u16 = 0b0000_0000_0000;

    let rows = matrix.len();
    let cols = matrix[0].len();

    for col in 0..cols {
        let mut c = 0;
        for row in 0..rows {
            match matrix[row][col] {
                0 => c -= 1,
                1 => c += 1,
                _ => println!("fucked"),
            };
        }

        // append a 0 or 1
        if c > 0 {
            gamma = gamma << 1 | 0b0000_0000_0001;
            epsilon = epsilon << 1;
        } else {
            gamma = gamma << 1;
            epsilon = epsilon << 1 | 0b0000_0000_0001;
        }
    }

    let res: i32 = gamma as i32 * epsilon as i32;
    println!("03 part1: {}", res);
}

fn calc_o2(matrix: &mut Vec<Vec<u8>>) -> u32 {
    let mut rows = matrix.len();
    let cols = matrix[0].len();

    for col in 0..cols {
        let mut c = 0;
        let mut to_remove = vec![];
        for row in 0..rows {
            match matrix[row][col] {
                0 => c -= 1,
                1 => c += 1,
                _ => println!("fucked"),
            };
        }

        for row in 0..rows {
            match matrix[row][col] {
                0 => {
                    if c < 0 {
                        to_remove.push(row)
                    } else {
                        continue;
                    }
                }
                1 => {
                    if c > 0 {
                        to_remove.push(row)
                    } else {
                        continue;
                    }
                }
                _ => println!("fucked"),
            }
        }

        matrix.retain(|row| row[col] == if c >= 0 { 1 } else { 0 });

        // update the total rows bc we removed some!!!!
        // def didn't spend 2 hours debugging this hahahahhahahahahahah fuck you
        rows = matrix.len();

        if rows == 0 {
            break;
        }
    }

    let o2_rating = matrix[0]
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | bit as u32);

    o2_rating
}

fn calc_co2(matrix: &mut Vec<Vec<u8>>) -> u32 {
    let mut rows = matrix.len();
    let cols = matrix[0].len();

    for col in 0..cols {
        let mut c = 0;
        let mut to_remove = vec![];
        for row in 0..rows {
            match matrix[row][col] {
                0 => c -= 1,
                1 => c += 1,
                _ => println!("fucked"),
            };
        }

        for row in 0..rows {
            match matrix[row][col] {
                0 => {
                    if c > 0 {
                        to_remove.push(row)
                    } else {
                        continue;
                    }
                }
                1 => {
                    if c < 0 {
                        to_remove.push(row)
                    } else {
                        continue;
                    }
                }
                _ => println!("fucked"),
            }
        }

        matrix.retain(|row| row[col] == if c >= 0 { 0 } else { 1 });

        // update the total rows bc we removed some!!!!
        // def didn't spend 2 hours debugging this hahahahhahahahahahah fuck you
        rows = matrix.len();

        if rows == 1 {
            break;
        }
    }

    let co2_rating = matrix[0]
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | bit as u32);

    co2_rating
}

// tbh this whole function seems a bit GOOFY
fn part_two(input: &str) {
    let matrix: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let o2 = calc_o2(&mut matrix.clone());
    let co2 = calc_co2(&mut matrix.clone());

    let res = o2 * co2;
    println!("03 part2: {}", res);
}

fn main() {
    let input = read_input("./inputs/3");

    part_one(&input);
    part_two(&input);
}
