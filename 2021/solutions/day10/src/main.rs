use utils::read_input;

fn part_one(input: &str) {
    let mut total: i32 = 0;
    'linez: for line in input.lines() {
        // create a stack (vector)
        let mut stack = vec![];
        for char in line.chars() {
            if "([{<".contains(char) {
                stack.push(char);
            } else if let Some(&top) = stack.last() {
                if (char == ')' && top == '(')
                    || (char == ']' && top == '[')
                    || (char == '}' && top == '{')
                    || (char == '>' && top == '<')
                {
                    stack.pop();
                } else {
                    //println!("invalid char '{}' found in {}", char, line);
                    match char {
                        ')' => total += 3,
                        ']' => total += 57,
                        '}' => total += 1197,
                        '>' => total += 25137,
                        _ => println!("fucked"),
                    }
                    continue 'linez;
                }
            }
        }
    }

    println!("10 part1: {}", total);
}

fn part_two(input: &str) {
    let mut invalid_lines: Vec<Vec<char>> = vec![];
    let mut totals: Vec<u64> = vec![];
    'linez: for line in input.lines() {
        // create a stack (vector)
        let mut stack = vec![];
        for char in line.chars() {
            if "([{<".contains(char) {
                stack.push(char);
            } else if let Some(&top) = stack.last() {
                if (char == ')' && top == '(')
                    || (char == ']' && top == '[')
                    || (char == '}' && top == '{')
                    || (char == '>' && top == '<')
                {
                    stack.pop();
                } else {
                    continue 'linez;
                }
            }
        }

        if !stack.is_empty() {
            invalid_lines.push(stack);
        }
    }

    for mut line in invalid_lines {
        let mut total = 0;
        line.reverse();

        for char in line {
            match char {
                '(' => total = total * 5 + 1,
                '[' => total = total * 5 + 2,
                '{' => total = total * 5 + 3,
                '<' => total = total * 5 + 4,
                _ => println!("fucked"),
            }
        }

        totals.push(total);
    }

    totals.sort();
    let median = totals[totals.len() / 2];
    println!("10 part2: {:?}", median);
}

fn main() {
    let input = read_input("./inputs/10");
    //let input = read_input("./inputs/10.test");

    part_one(&input);
    part_two(&input);
}
