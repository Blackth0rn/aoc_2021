use std::error::Error;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input = raw_input.trim();

    let output = compute(input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut count_of_increases = 0;
    let depths: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    for (i, depth) in depths.iter().enumerate().skip(1) {
        let prev_depth = depths[i - 1];

        if prev_depth < *depth {
            count_of_increases += 1;
        }
    }
    Ok(count_of_increases)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            compute("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n").unwrap(),
            7
        )
    }
    #[test]
    fn basic_test() {
        assert_eq!(compute("199\n200\n").unwrap(), 1)
    }
    #[test]
    fn no_input() {
        assert_eq!(compute("").unwrap(), 0)
    }
}
