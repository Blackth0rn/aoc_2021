use std::cmp::Ordering;
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

    // pseudo code
    // get ones, zeros for all numbers
    // find most common bit in idx 0
    // filter numbers by that bit
    // repeat until length of numbers = 0
    let oxygen_number = find_and_filter(
        input.lines().map(|line| line.trim()).collect(),
        report_number_length,
        0,
        oxygen_comparison,
    );
    let co2_number = find_and_filter(
        input.lines().map(|line| line.trim()).collect(),
        report_number_length,
        0,
        co2_comparison,
    );

    // parse both numbers to decimal and multiply
    let oxygen_number = i32::from_str_radix(oxygen_number, 2)?;
    let co2_number = i32::from_str_radix(co2_number, 2)?;

    Ok(oxygen_number * co2_number)
}

fn oxygen_comparison(z: i32, o: i32) -> char {
    match z.cmp(&o) {
        Ordering::Greater => '0',
        Ordering::Equal => '1',
        Ordering::Less => '1',
    }
}

fn co2_comparison(z: i32, o: i32) -> char {
    match z.cmp(&o) {
        Ordering::Greater => '1',
        Ordering::Equal => '0',
        Ordering::Less => '0',
    }
}

fn find_and_filter<F>(
    lines: Vec<&str>,
    report_number_length: usize,
    depth: usize,
    bit_comparison: F,
) -> &str
where
    F: Fn(i32, i32) -> char,
{
    let (zeros, ones) = find_zeros_and_ones(&lines, report_number_length);

    let most_common_bit = bit_comparison(zeros[depth], ones[depth]);

    let mut new_lines: Vec<&str> = Vec::new();
    // filter lines
    for line in lines {
        if line.chars().nth(depth) == Some(most_common_bit) {
            new_lines.push(line);
        }
    }

    if new_lines.len() == 1 {
        new_lines[0]
    } else {
        find_and_filter(new_lines, report_number_length, depth + 1, bit_comparison)
    }
}

fn find_zeros_and_ones(lines: &[&str], report_number_length: usize) -> (Vec<i32>, Vec<i32>) {
    let mut zeros: Vec<i32> = vec![0; report_number_length];
    let mut ones: Vec<i32> = vec![0; report_number_length];

    for line in lines.iter() {
        for (i, bit) in line.trim().chars().enumerate() {
            match bit {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => (),
            }
        }
    }

    (zeros, ones)
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
            230
        )
    }

    #[test]
    fn basic_find_zeros_and_ones() {
        assert_eq!(
            find_zeros_and_ones(&["000", "010", "101"], 3),
            (vec![2, 2, 2], vec![1, 1, 1])
        )
    }
}
