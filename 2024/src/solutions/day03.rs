use std::fs;
use anyhow::Result;

use regex::Regex;

pub fn part1() -> Result<i32> {
    //let input = fs::read_to_string("inputs/day03.txt")?;
    let input = fs::read_to_string("inputs/day03_test.txt")?;
    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)")?;
    let mut total = 0;

    re.captures_iter(&input).for_each(|cap| {
        total += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
    });

    Ok(total)
}

pub fn part2() -> Result<i32> {
    //let input = fs::read_to_string("inputs/day03.txt")?;
    let input = fs::read_to_string("inputs/day03_test.txt")?;

    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|(?:do|don't)\(\)")?;
    let mut total = 0;
    let mut flag = true;

    re.captures_iter(&input).for_each(|cap| {
        let string = cap[0].to_string();
        if string == "do()" {
            flag = true;
        } else if string == "don't()" {
            flag = false;
        } else {
            if flag {
                total += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
            }
        }
    });

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1().unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let result = part2().unwrap();
        assert_eq!(result, 48);
    }
}
