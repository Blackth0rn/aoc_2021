use std::{error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<i32, Box<dyn Error>> {
    let mut fish_school = input[0]
        .split(',')
        .map(|val| val.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let days = 80;
    let mut fish_school_buffer = Vec::new();

    for _i in 0..days {
        for fish in &fish_school {
            match fish {
                0 => {
                    fish_school_buffer.push(6); // reset fish
                    fish_school_buffer.push(8); // add new fish
                }
                age @ 1..=8 => {
                    fish_school_buffer.push(*age - 1);
                }
                _ => (),
            }
        }
        fish_school.clear();
        fish_school.append(&mut fish_school_buffer);
    }
    Ok(fish_school.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(compute(&test_data).unwrap(), 5934)
    }
}
