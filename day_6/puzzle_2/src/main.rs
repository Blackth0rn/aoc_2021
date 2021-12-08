use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "input.txt";
const DAYS: i32 = 256;
// const DAYS: i32 = 80;
// const DAYS: i32 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input, DAYS)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str], simulation_length_in_days: i32) -> Result<i64, Box<dyn Error>> {
    let fish_school = input[0]
        .split(',')
        .map(|val| val.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let days_from_end_born_at = fish_school
        .iter()
        .map(|fish| 8 - fish + simulation_length_in_days) // 8 is the maximum days ago that a fish could have been born
        .collect::<Vec<i32>>();

    let fish_reproduction_map = compute_reproduction_map(simulation_length_in_days);

    let mut count = 0;
    for fish in days_from_end_born_at {
        count += 1;
        count += fish_reproduction_map
            .get(&fish)
            .unwrap_or_else(|| panic!("Map doesn't contain reproduction data for fish: {}", fish));
    }

    Ok(count)
}

fn compute_reproduction_map(simulation_length_in_days: i32) -> HashMap<i32, i64> {
    let mut map = HashMap::new();

    let mut born_at_days_from_end = 0;
    while born_at_days_from_end <= simulation_length_in_days + 9 {
        map.insert(
            born_at_days_from_end,
            compute_number_of_offspring(born_at_days_from_end, &map),
        );
        born_at_days_from_end += 1;
    }

    map
}

fn compute_number_of_offspring(born_at_days_from_end: i32, map: &HashMap<i32, i64>) -> i64 {
    // if a fish is born less than 9 days from the end than it can't reproduce
    if born_at_days_from_end < 9 {
        return 0;
    }

    // count of how many fish this fish directly produced
    let mut offspring_count: i64 = ((i64::from(born_at_days_from_end) - 9) / 7) + 1;

    // fish we indirectly reproduced = count from map at each birth date
    let mut child_birth_date = born_at_days_from_end - 9;
    while child_birth_date >= 0 {
        offspring_count += map.get(&child_birth_date).unwrap_or_else(|| {
            panic!(
                "Map doesn't contain key for birth date: {}",
                child_birth_date
            )
        });
        child_birth_date -= 7;
    }

    offspring_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_18_days() {
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(compute(&test_data, 18).unwrap(), 26)
    }

    #[test]
    fn example_80_days() {
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(compute(&test_data, 80).unwrap(), 5934)
    }

    #[test]
    fn example_256_days() {
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(compute(&test_data, 256).unwrap(), 26984457539)
    }
}
