use std::{
    collections::{hash_map::RandomState, HashSet},
    error::Error,
    fs,
    str::FromStr,
};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<usize, Box<dyn Error>> {
    // need to parse dots
    // need to parse instructions

    let mut dots = Vec::new();
    let mut instructions = Vec::new();

    for line in input {
        if line == &"" {
            continue;
        }

        // fold
        if line.starts_with('f') {
            instructions.push(Fold::from_str(line)?);
        } else {
            // dot
            dots.push(Dot::from_str(line)?);
        }
    }

    // all folds
    for instruction in instructions {
        for dot in &mut dots.iter_mut() {
            dot.fold(&instruction);
        }
    }

    let dot_set: HashSet<Dot, RandomState> =
        HashSet::from_iter(dots.into_iter().filter(|val| !val.culled));

    // need to print it somehow
    let mut max_x = 0;
    let mut max_y = 0;

    for dot in dot_set.iter() {
        if dot.x > max_x {
            max_x = dot.x;
        }
        if dot.y > max_y {
            max_y = dot.y;
        }
    }

    let mut plot_area = vec![vec!['.'; max_x + 1]; max_y + 1];

    for dot in dot_set.iter() {
        plot_area[dot.y][dot.x] = '#';
    }

    (0..max_y + 1).for_each(|y| {
        for x in 0..max_x + 1 {
            print!("{}", plot_area[y][x]);
        }
        println!();
    });

    Ok(0)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dot {
    x: usize,
    y: usize,
    culled: bool,
}

impl Dot {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            culled: false,
        }
    }

    fn fold(&mut self, instruction: &Fold) {
        match instruction.direction {
            Direction::X => {
                // if we're folding along x = 5, then values > 5 will get moved to the difference
                // between value and x subtracted from x
                match self.x.cmp(&instruction.value) {
                    std::cmp::Ordering::Greater => {
                        self.x = instruction.value - (self.x - instruction.value);
                    }
                    std::cmp::Ordering::Equal => self.culled = true,
                    _ => (),
                }
            }
            Direction::Y => {
                // if we're folding along x = 5, then values > 5 will get moved to the difference
                // between value and x subtracted from x
                match self.y.cmp(&instruction.value) {
                    std::cmp::Ordering::Greater => {
                        self.y = instruction.value - (self.y - instruction.value);
                    }
                    std::cmp::Ordering::Equal => self.culled = true,
                    _ => (),
                }
            }
        }
    }
}

impl FromStr for Dot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|val| {
                val.parse::<usize>()
                    .map_err(|_| format!("Bad digit: {}", val))
            })
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self::new(parts[0], parts[1]))
    }
}

enum Direction {
    X,
    Y,
}

struct Fold {
    direction: Direction,
    value: usize,
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fold along x=5
        // need to separate out the x=5 part
        let mut iter = s.split_whitespace().nth(2).unwrap().chars();
        let direction = match iter.next() {
            Some('x') => Direction::X,
            Some('y') => Direction::Y,
            _ => return Err(String::from("unknown direction")),
        };
        iter.next(); // = sign
        let value = iter
            .as_str()
            .parse::<usize>()
            .map_err(|err| err.to_string())?;

        Ok(Self { direction, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];
        assert_eq!(compute(&test_data).unwrap(), 17);
    }
}
