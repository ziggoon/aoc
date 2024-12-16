use std::fs;
use anyhow::Result;

pub fn part1() -> Result<String> {
    //let input = fs::read_to_string("inputs/dayXX.txt")?;
    let input = fs::read_to_string("inputs/day02_test.txt")?;
    Ok("Not implemented yet".to_string())
}

pub fn part2() -> Result<String> {
    //let input = fs::read_to_string("inputs/dayXX.txt")?;
    let input = fs::read_to_string("inputs/day02_test.txt")?;
    Ok("Not implemented yet".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1().unwrap();
        assert_eq!(result, "Not implemented yet");
    }

    #[test]
    fn test_part2() {
        let result = part2().unwrap();
        assert_eq!(result, "Not implemented yet");
    }
}
