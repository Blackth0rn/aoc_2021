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

fn compute(input: &str) -> Result<i32, Box<dyn Error>> {
    // iterate each line and add to a count for each bit, either 0 or 1
    // so a a pair of vecs, index is bit position, value is count of either 0 or 1s

    let report_number_length = if let Some(line) = input.lines().next() {
        line.len()
    } else {
        return Err("Empty input".into());
    };

    let mut zeros: Vec<i32> = vec![0; report_number_length];
    let mut ones: Vec<i32> = vec![0; report_number_length];

    for line in input.lines() {
        for (i, bit) in line.trim().chars().enumerate() {
            match bit {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => (),
            }
        }
    }

    // now we have our bits so we can make up a binary value for gamma and epsilon
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, zero_count) in zeros.iter().enumerate() {
        let ones_count = ones[i];

        let exponent = report_number_length as u32 - 1 - i as u32;

        if ones_count > *zero_count {
            gamma += 2_i32.pow(exponent);
        } else {
            epsilon += 2_i32.pow(exponent);
        }
    }
    Ok(gamma * epsilon)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            compute(
                "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
            "
            )
            .unwrap(),
            198
        )
    }
}
