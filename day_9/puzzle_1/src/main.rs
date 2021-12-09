use std::{error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

struct HeightMap {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len();
        let height = input.len();

        // convert from str slice to vec of vecs of ints
        let height_map = input
            .iter()
            .map(|val| {
                val.chars()
                    .map(|height| height.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        Self {
            height,
            width,
            map: height_map,
        }
    }

    fn item_at(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.map[y][x])
    }
}

fn compute(input: &[&str]) -> Result<u32, Box<dyn Error>> {
    let map = HeightMap::new(input);

    let mut sum_of_low_points = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            // get surrounding items, compare to current
            let mut is_a_hole = true;

            let current = map
                .item_at(x, y)
                .ok_or_else(|| format!("Bad coords: {} {}", x, y))?;

            if y > 0 {
                if let Some(up) = map.item_at(x, y - 1) {
                    if up <= current {
                        is_a_hole = false;
                    }
                }
            }

            if let Some(right) = map.item_at(x + 1, y) {
                if right <= current {
                    is_a_hole = false;
                }
            }

            if let Some(down) = map.item_at(x, y + 1) {
                if down <= current {
                    is_a_hole = false;
                }
            }

            if x > 0 {
                if let Some(left) = map.item_at(x - 1, y) {
                    if left <= current {
                        is_a_hole = false;
                    }
                }
            }

            if is_a_hole {
                sum_of_low_points += 1 + current;
            }
        }
    }

    Ok(sum_of_low_points)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        assert_eq!(compute(&test_data).unwrap(), 15)
    }

    #[test]
    fn example2() {
        let test_data = vec!["210", "921", "892"];
        assert_eq!(compute(&test_data).unwrap(), 10)
    }

    #[test]
    fn example3() {
        let test_data = vec![
            "1210", // 1, 0
            "6921", "2892", // 2
            "4321", "1881", // 1
        ];
        assert_eq!(compute(&test_data).unwrap(), 8)
    }
}
