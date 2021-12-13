use std::{cell::RefCell, collections::HashSet, error::Error, fmt::Display, fs};

const INPUT_PATH: &str = "input.txt";

const STEPS: u32 = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input, STEPS, false)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

#[derive(Debug)]
struct OctopusMatrix {
    octopi: Vec<RefCell<Octopus>>,
    width: usize,
    height: usize,
}

impl OctopusMatrix {
    fn new(input: &[&str], width: usize, height: usize) -> Self {
        let mut octopi = Vec::with_capacity(width * height);

        for (i, line) in input.iter().enumerate() {
            for (j, energy_char) in line.chars().enumerate() {
                let octopus = Octopus::new(energy_char, i * width + j);
                octopi.push(RefCell::new(octopus));
            }
        }

        // set up neighbours
        for y in 0..height {
            for x in 0..width {
                let octopus = &mut octopi[y * height + x].borrow_mut();

                // tl neighbour if x and y > 0
                if x > 0 && y > 0 {
                    octopus.neighbours.push((y - 1) * height + x - 1);
                }
                // top neighbour if y > 0
                if y > 0 {
                    octopus.neighbours.push((y - 1) * height + x);
                }
                // tr neighbour if y > 0 and x < width - 1
                if x < width - 1 && y > 0 {
                    octopus.neighbours.push((y - 1) * height + x + 1);
                }

                // l neighbour if x > 0
                if x > 0 {
                    octopus.neighbours.push(y * height + x - 1);
                }
                // r neighbour if x < width - 1
                if x < width - 1 {
                    octopus.neighbours.push(y * height + x + 1);
                }

                // bl neighbour if x > 0 and y < height - 1
                if x > 0 && y < height - 1 {
                    octopus.neighbours.push((y + 1) * height + x - 1);
                }
                // bottom neighbour if y < height - 1
                if y < height - 1 {
                    octopus.neighbours.push((y + 1) * height + x);
                }
                // br neighbour if x < width - 1, y < height - 1
                if x < width - 1 && y < height - 1 {
                    octopus.neighbours.push((y + 1) * height + x + 1);
                }
            }
        }
        Self {
            octopi,
            width,
            height,
        }
    }

    fn next_step(&mut self, step_count: u32) -> u32 {
        // iterate the data to the next step
        let mut flash_count = 0;

        let mut flashing_octopi = HashSet::new();
        // iterate all octopi, increment energy, add flashers to flash set
        for octopus in &self.octopi {
            octopus.borrow_mut().increment_energy(step_count);
            if octopus.borrow().can_flash(step_count) {
                flashing_octopi.insert(octopus.borrow().index);
            }
        }

        while !flashing_octopi.is_empty() {
            let oct_idx = flashing_octopi.iter().next().cloned().unwrap();
            flashing_octopi.remove(&oct_idx);
            let octopus = &self.octopi[oct_idx];

            octopus.borrow_mut().flash(step_count);
            flash_count += 1;

            for neighbour_idx in &octopus.borrow().neighbours {
                let neighbour_octopus = &self.octopi[*neighbour_idx];
                neighbour_octopus.borrow_mut().increment_energy(step_count);
                // TODO we're double adding neighbours here, need to maybe track them a different
                // way (index list possibly?)
                if neighbour_octopus.borrow().can_flash(step_count)
                    && !flashing_octopi.contains(neighbour_idx)
                {
                    flashing_octopi.insert(*neighbour_idx);
                }
            }
        }
        flash_count
    }
}

impl Display for OctopusMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for each line, print the 10 values
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.octopi[y * self.width + x].borrow())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Octopus {
    index: usize,
    energy: u32,
    neighbours: Vec<usize>,
    last_flash_step: u32,
}

impl Octopus {
    fn new(input: char, index: usize) -> Self {
        let energy = input.to_digit(10).unwrap();

        Self {
            index,
            energy,
            neighbours: Vec::new(),
            last_flash_step: 0,
        }
    }

    fn increment_energy(&mut self, step_count: u32) {
        if self.last_flash_step < step_count {
            self.energy += 1;
        }
    }

    fn flash(&mut self, step_count: u32) {
        self.energy = 0;
        self.last_flash_step = step_count;
    }

    fn can_flash(&self, step_count: u32) -> bool {
        (self.energy > 9) && (self.last_flash_step < step_count)
    }
}

impl Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.energy)
    }
}

fn compute(input: &[&str], steps: u32, debug: bool) -> Result<u32, Box<dyn Error>> {
    // convert input to graph of octopi, with neighbours
    let width = input[0].len();
    let height = input.len();
    let mut oct_matrix = OctopusMatrix::new(input, width, height);

    for step_count in 1..=steps {
        if oct_matrix.next_step(step_count) == (width * height).try_into()? {
            return Ok(step_count);
        }
        if debug {
            println!("{}", oct_matrix);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];
        assert_eq!(compute(&test_data, STEPS, false).unwrap(), 195)
    }
}
