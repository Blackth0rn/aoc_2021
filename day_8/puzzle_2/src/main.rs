use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<i32, Box<dyn Error>> {
    let mut signal_patterns: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut output_values: Vec<Vec<HashSet<char>>> = Vec::new();

    for line in input {
        let mut iter = line.split('|');
        let raw_patterns = iter.next().ok_or_else(|| String::from("Invalid input"))?;
        let raw_patterns = raw_patterns.split_whitespace().collect::<Vec<&str>>();
        let pattern_sets = raw_patterns
            .iter()
            .map(|val| val.chars().collect::<HashSet<char>>())
            .collect();
        signal_patterns.push(pattern_sets);

        let raw_output = iter.next().ok_or_else(|| String::from("Invalid input"))?;
        let raw_output = raw_output.split_whitespace().collect::<Vec<&str>>();
        let output_sets = raw_output
            .iter()
            .map(|val| val.chars().collect::<HashSet<char>>())
            .collect();
        output_values.push(output_sets);
    }

    let mut count: usize = 0;
    for i in 0..signal_patterns.len() {
        let number_mappings = build_char_set_number_mapping(&signal_patterns[i]);

        // convert output sets to numbers, then parse as an int, then sum to the total

        let output_sets = &output_values[i];
        for (exp, output_set) in output_sets.iter().enumerate() {
            for (i, number_mapping) in &number_mappings {
                if output_set == *number_mapping {
                    count += i * 10_usize.pow((3_usize - exp).try_into().unwrap());
                }
            }
        }
    }

    Ok(count.try_into().unwrap())
}

// this function takes a list of patterns and works out which segment lines up with which wire
// returned is a mapping of numbers to sets of characters
fn build_char_set_number_mapping(
    signal_patterns: &[HashSet<char>],
) -> HashMap<usize, &HashSet<char>> {
    // TODO
    // numbers to do 3
    // segments to do: n/a
    let mut numbers = HashMap::new();
    // numbers done: 0, 1, 2, 4, 5, 6, 7, 8, 9
    // segments done: 0, 1, 2, 3, 4, 6
    // segments done: a, b, c, d, e, g

    // a,b,c,d,e,f,g == indices 0,1,2,3,4,5,6
    // so the character at index 0 is display segment 0
    let mut segments = HashMap::new();

    for pattern in signal_patterns {
        match pattern.len() {
            2 => numbers.insert(1, pattern),
            3 => numbers.insert(7, pattern),
            4 => numbers.insert(4, pattern),
            7 => numbers.insert(8, pattern),
            _ => None,
        };
    }

    // can work out segment 0 from number 7 - 4
    segments.insert(0, *numbers[&7].difference(numbers[&4]).next().unwrap());

    // number 9 == 6 length and intersects with 4 completely
    // number 0 == length 6, not 9, intersects with 1
    // number 6 == 6 length and doesn't intersect with 4 completely
    for pattern in signal_patterns {
        if pattern.len() == 6 {
            if numbers[&4].is_subset(pattern) {
                numbers.insert(9, pattern);
            } else if numbers[&1].is_subset(pattern) {
                numbers.insert(0, pattern);
            } else {
                numbers.insert(6, pattern);
            }
        }
    }

    // segment 6 == 9 - 4 - 7
    let segments_1_2_3_4_6 = numbers[&4]
        .union(numbers[&7])
        .cloned()
        .collect::<HashSet<char>>();
    segments.insert(
        6,
        *numbers[&9].difference(&segments_1_2_3_4_6).next().unwrap(),
    );

    // segment 4 == 9 - 8
    segments.insert(4, *numbers[&8].difference(numbers[&9]).next().unwrap());

    // segment 3 == difference of 8 and 0
    segments.insert(3, *numbers[&8].difference(numbers[&0]).next().unwrap());

    // segment 1 == number 4 - number 1 and segment d/3
    segments.insert(
        1,
        **numbers[&4]
            .difference(numbers[&1])
            .collect::<HashSet<&char>>()
            .difference(&[segments[&3]].iter().collect())
            .next()
            .unwrap(),
    );

    // number 2 == pattern with len == 5 and segments a,d,e,g
    // number 5 == pattern with len == 5, and segment b
    for pattern in signal_patterns {
        if pattern.len() == 5 {
            if pattern.is_superset(
                &[segments[&0], segments[&3], segments[&4], segments[&6]]
                    .iter()
                    .cloned()
                    .collect(),
            ) {
                numbers.insert(2, pattern);
            } else if pattern.is_superset(&[segments[&1]].iter().cloned().collect()) {
                numbers.insert(5, pattern);
            }
        }
    }

    // segment 2 = 2 - adge
    segments.insert(
        2,
        *numbers[&2]
            .difference(
                &[segments[&0], segments[&3], segments[&4], segments[&6]]
                    .iter()
                    .cloned()
                    .collect(),
            )
            .next()
            .unwrap(),
    );

    // segments 5 is 1 - segment 2
    segments.insert(
        5,
        *numbers[&1]
            .difference(&[segments[&2]].iter().cloned().collect())
            .next()
            .unwrap(),
    );

    // number 3
    for pattern in signal_patterns {
        if pattern.len() == 5 && pattern.is_superset(numbers[&1]) {
            numbers.insert(3, pattern);
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];

        assert_eq!(compute(&test_data).unwrap(), 61229)
    }
}
