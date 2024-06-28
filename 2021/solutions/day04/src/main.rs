use utils::read_input;

fn check_rows(board: &mut Vec<Vec<u8>>) -> bool {
    board.iter().any(|row| row.iter().all(|&val| val == 255))
}

fn check_cols(board: &mut Vec<Vec<u8>>) -> bool {
    board.iter().any(|col| col.iter().all(|&val| val == 255))
}

fn calc_final(board: Vec<Vec<u8>>, draw: u8) -> i32 {
    let mut total: u32 = 0;
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] != 255 {
                total += board[row][col] as u32;
            }
        }
    }

    total *= draw as u32;
    total as i32
}

fn part_one(input: &str) {
    let mut lines = input.lines();
    let draws: Vec<&str> = lines.next().unwrap().split(',').collect();

    // skip blank
    lines.next().unwrap();

    // is this legal?
    let mut boards: Vec<Vec<Vec<u8>>> = vec![];
    let mut to_add: Vec<Vec<u8>> = vec![];
    for line in lines {
        if line.is_empty() {
            boards.push(to_add);
            to_add = vec![];
        } else {
            let row: Vec<u8> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            to_add.push(row);
        }
    }

    let mut winner: Vec<Vec<u8>> = vec![];
    for draw in draws {
        for board in &mut boards {
            for row in 0..board.len() {
                for col in 0..board[0].len() {
                    if board[row][col] == draw.parse::<u8>().unwrap() {
                        board[row][col] = 255;
                    }
                }
            }

            let rows = check_rows(board);
            let cols = check_cols(board);

            if rows || cols {
                winner = board.clone();
                break;
            }
        }

        if !winner.is_empty() {
            let res = calc_final(winner, draw.parse::<u8>().unwrap());
            println!("04 part1: {}", res);
            break;
        }
    }
}

fn part_two(input: &str) {
    let mut lines = input.lines();
    let draws: Vec<&str> = lines.next().unwrap().split(',').collect();

    // meow!
    lines.next().unwrap();

    // is this legal?
    let mut boards: Vec<Vec<Vec<u8>>> = vec![];
    let mut to_add: Vec<Vec<u8>> = vec![];
    for line in lines {
        if line.is_empty() {
            boards.push(to_add);
            to_add = vec![];
        } else {
            let row: Vec<u8> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            to_add.push(row);
        }
    }

    let mut to_remove: Vec<Vec<Vec<u8>>> = vec![];
    for draw in draws {
        for board in &mut boards {
            for row in 0..board.len() {
                for col in 0..board[0].len() {
                    if board[row][col] == draw.parse::<u8>().unwrap() {
                        board[row][col] = 255;
                    }
                }
            }

            let rows = check_rows(board);
            let cols = check_cols(board);

            if rows || cols {
                to_remove.push(board.clone());
            }
        }

        if boards.len() == 1 {
            let res = calc_final(boards[0].clone(), draw.parse::<u8>().unwrap());
            println!("{:?}", boards);
            println!("{}", draw);
            println!("04 part2: {}", res);
            break;
        }

        boards.retain(|b| !to_remove.contains(b));
        to_remove.clear();
    }
}

fn main() {
    let input = read_input("./inputs/4");

    part_one(&input);
    part_two(&input);
}
