use std::{collections::HashMap, error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input, 10)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str], steps: usize) -> Result<u32, Box<dyn Error>> {
    // parse initial state
    // parse mapping

    let mut polymer = String::from(input[0]);
    let mut mapping = HashMap::new();

    for formula in input[2..].iter() {
        let mut iter = formula.split(" -> ");
        let pair = iter.next().unwrap();
        let result = iter.next().unwrap();

        mapping.insert(pair, result);
    }

    for _ in 0..steps {
        let mut polymer_iter = polymer.chars();
        let mut last_item = polymer_iter.next().unwrap();

        let mut new_str = String::from(last_item);

        for item in polymer_iter {
            // make a pair from last_item + item
            let pair = vec![last_item, item].iter().collect::<String>();
            // look up mapping
            let new_item = mapping.get(pair.as_str()).unwrap();
            // insert val from lookup
            new_str.push_str(new_item);
            // insert item
            new_str.push(item);
            // set last_item = item
            last_item = item;
        }

        polymer = new_str;
    }

    let mut counts = HashMap::new();
    for val in polymer.chars() {
        let counter = counts.entry(val).or_insert(0);
        *counter += 1;
    }

    let mut min = u32::MAX;
    let mut max = 0;
    for (_, val) in counts.iter() {
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
}
