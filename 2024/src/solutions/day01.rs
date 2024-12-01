use std::collections::HashMap;
use std::fs;
use anyhow::Result;

// 1. pair smallest in each list incrementally
// 2. calculate difference between all pairs
// 3. sum all differences
pub fn part1() -> Result<i32> {
    let input = fs::read_to_string("inputs/day01.txt")?;
    //let input = fs::read_to_string("inputs/day01_test.txt")?;
    let mut total = 0;
    let mut right = vec![];
    let mut left = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        right.push(parts[1].parse::<i32>()?);
        left.push(parts[0].parse::<i32>()?);
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        if left[i] > right[i] {
            total += left[i] - right[i];
        } else {
            total += right[i] - left[i];
        }
    }

    Ok(total)
}

// 1. find how often a number from the left list appears in the right list
// 2. calculate similarity score by summing (key * value)
pub fn part2() -> Result<i32> {
    let input = fs::read_to_string("inputs/day01.txt")?;
    //let input = fs::read_to_string("inputs/day01_test.txt")?;
    let mut total = 0;
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    let mut left = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>()?);
        let right = parts[1].parse::<i32>()?;

        if occurrences.contains_key(&right) {
            let value = occurrences.get(&right).unwrap();
            occurrences.insert(right, value + 1);
        } else {
            occurrences.insert(right, 1);
        }
    }

    left.sort();

    for i in 0..left.len() {
        if occurrences.contains_key(&left[i]) {
            total += &left[i] * occurrences.get(&left[i]).unwrap();
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1().unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let result = part2().unwrap();
        assert_eq!(result, 31);
    }
}