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
    let mut positions = input[0]
        .split(',')
        .map(|val| val.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    positions.sort_unstable();

    let min = positions[0];
    let max = positions[positions.len() - 1];

    // iterate from min to max, calculate fuel usage to get all crabs here
    //
    // alternatively, start at the middle of the sorted list, calculate for all, move to 1/4 then
    // 3/4 and do the same?
    //
    // is this some form of binary search?
    let mut current_lowest_fuel = i32::MAX;
    for selected_position in min..=max {
        let mut fuel_usage = 0;
        for crab_pos in &positions {
            fuel_usage += fuel_calculation((crab_pos - selected_position).abs());
        }

        if fuel_usage < current_lowest_fuel {
            current_lowest_fuel = fuel_usage;
        }
    }

    Ok(current_lowest_fuel)
}

fn fuel_calculation(distance: i32) -> i32 {
    let mut fuel_total = 0;
    for i in 0..=distance {
        fuel_total += i;
    }
    fuel_total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["16,1,2,0,4,2,7,1,2,14"];

        assert_eq!(compute(&test_data).unwrap(), 168)
    }

    #[test]
    fn fuel_calc() {
        assert_eq!(fuel_calculation(4), 10)
    }
}
