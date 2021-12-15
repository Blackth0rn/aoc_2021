use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input, 40)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str], steps: usize) -> Result<u64, Box<dyn Error>> {
    let polymer = input[0];
    let mut mapping = HashMap::new();

    for formula in input[2..].iter() {
        let mut iter = formula.split(" -> ");
        let mut raw_pair = iter.next().unwrap().chars();
        let pair = (raw_pair.next().unwrap(), raw_pair.next().unwrap());

        let result = iter.next().unwrap().chars().next().unwrap();
        // convert to the new pairs?
        // ie (N, N) -> (N, C) + (C, N)
        let new_pairs = vec![(pair.0, result), (result, pair.1)];

        mapping.insert(pair, new_pairs);
    }

    let mut counts = HashMap::new();

    // convert input string to pairs
    let mut iter = polymer.chars();
    let mut last_value = iter.next().unwrap();

    // used later in the counting
    let first_char = last_value;

    for val in iter {
        let counter = counts.entry((last_value, val)).or_insert(0);
        *counter += 1;

        last_value = val;
    }

    for _ in 0..steps {
        // for each pair, make new pairs, increment counts
        let mut new_counts = HashMap::new();
        for (pair, pair_count) in &counts {
            // get new pairs from mapping
            if let Some(new_pairs) = mapping.get(pair) {
                for new_pair in new_pairs {
                    let counter = new_counts.entry(*new_pair).or_insert(0);
                    *counter += pair_count;
                }
            }
        }
        counts = new_counts;
    }

    println!("{:?}", counts);

    let mut char_counts = HashMap::new();
    char_counts.insert(first_char, 1);
    // only count the second value from each pair, we've added the first N already
    for (pair, count) in &counts {
        let counter = char_counts.entry(pair.1).or_insert(0);
        *counter += count;
    }
    println!("{:?}", char_counts);

    let mut min = u64::MAX;
    let mut max = 0;
    for (_, val) in char_counts.iter() {
        if val < &min {
            min = *val;
        }
        if val > &max {
            max = *val;
        }
    }

    Ok(max - min)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        assert_eq!(compute(&test_data, 10).unwrap(), 1588);
    }

    #[test]
    fn example2() {
        let test_data = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        assert_eq!(compute(&test_data, 40).unwrap(), 2188189693529);
    }

    #[test]
    fn example3() {
        let test_data = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];
        assert_eq!(compute(&test_data, 1).unwrap(), 1);
    }
}
