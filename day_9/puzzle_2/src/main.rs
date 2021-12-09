use std::{collections::HashSet, error::Error, fs};

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

    let mut coords_seen: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<u32> = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            // if we've already been here in a basin, move along
            if coords_seen.contains(&(x, y)) {
                continue;
            }

            if map
                .item_at(x, y)
                .ok_or_else(|| format!("Bad coords: {} {}", x, y))?
                == 9
            {
                continue;
            }

            // we're not on a 9, we haven't already been here, so we're starting a new basin
            let mut current_basin_size = 1;
            let mut current_basin_edge = HashSet::new();
            current_basin_edge.insert((x, y));
            coords_seen.insert((x, y));

            // do bfs from here.
            while !current_basin_edge.is_empty() {
                let mut tmp_additions = HashSet::new();
                for (curr_x, curr_y) in &current_basin_edge {
                    // get neighbours that:
                    //      aren't 9
                    //      aren't already seen
                    //      push to already seen
                    //      push to current_basin_edge
                    //      increment size

                    if *curr_y > 0 {
                        if let Some(up) = map.item_at(*curr_x, *curr_y - 1) {
                            let up_point = (*curr_x, *curr_y - 1);

                            if up != 9 && !coords_seen.contains(&up_point) {
                                coords_seen.insert(up_point);
                                tmp_additions.insert(up_point);
                                current_basin_size += 1;
                            }
                        }
                    }

                    if let Some(right) = map.item_at(*curr_x + 1, *curr_y) {
                        let right_point = (*curr_x + 1, *curr_y);

                        if right != 9 && !coords_seen.contains(&right_point) {
                            coords_seen.insert(right_point);
                            tmp_additions.insert(right_point);
                            current_basin_size += 1;
                        }
                    }

                    if let Some(down) = map.item_at(*curr_x, *curr_y + 1) {
                        let down_point = (*curr_x, *curr_y + 1);

                        if down != 9 && !coords_seen.contains(&down_point) {
                            coords_seen.insert(down_point);
                            tmp_additions.insert(down_point);
                            current_basin_size += 1;
                        }
                    }

                    if *curr_x > 0 {
                        if let Some(left) = map.item_at(*curr_x - 1, *curr_y) {
                            let left_point = (*curr_x - 1, *curr_y);

                            if left != 9 && !coords_seen.contains(&left_point) {
                                coords_seen.insert(left_point);
                                tmp_additions.insert(left_point);
                                current_basin_size += 1;
                            }
                        }
                    }
                }
                current_basin_edge.clear(); // remove all the things we just iterated over
                current_basin_edge.extend(tmp_additions); // extend with the new things
            }

            basins.push(current_basin_size);
        }
    }

    // find max 3 of basins and multiply
    basins.sort_unstable();
    basins.reverse();

    Ok(basins[0] * basins[1] * basins[2])
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
        assert_eq!(compute(&test_data).unwrap(), 1134)
    }
}
