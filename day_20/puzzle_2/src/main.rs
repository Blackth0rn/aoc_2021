use std::{error::Error, fmt::Display, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<i32> {
    let mut iter = input.iter();

    let mut algorithm = Enhancement { data: [0; 512] };
    for (i, val) in iter
        .next()
        .ok_or_else(|| String::from("Empty input"))?
        .chars()
        .enumerate()
    {
        if val == '#' {
            algorithm.data[i] = 1
        }
    }

    iter.next(); // skip blank line

    let mut data = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for line in iter {
        height += 1;
        width = line.len() as i32;
        for val in line.chars() {
            match val {
                '#' => data.push(1),
                '.' => data.push(0),
                _ => (),
            }
        }
    }

    let mut input_image = InputImage::new(width, height, data, 0);
    for i in 0..50 {
        println!("Starting iteration {}", i);
        input_image = input_image.new_image_from_algorithm(&algorithm)?;
    }

    Ok(input_image.count_lit_pixels())
}

struct Enhancement {
    data: [i32; 512],
}

struct InputImage {
    width: i32,
    height: i32,
    data: Vec<i32>,
    infinite_value: i32,
}

// coords, (0,0) is top left
impl InputImage {
    fn new(width: i32, height: i32, data: Vec<i32>, infinite_value: i32) -> Self {
        Self {
            width,
            height,
            data,
            infinite_value,
        }
    }

    fn count_lit_pixels(&self) -> i32 {
        let mut count = 0;

        for px in &self.data {
            count += *px;
        }

        count
    }

    fn get_pixel_value(&self, x: i32, y: i32) -> Result<u16> {
        // get the 9 values from the data
        // convert to a binary number
        // Note: if the value is out of range, assume 0

        // getting (0,0), we need (-1, -1) for top left
        let px_vals = vec![
            self.get_data_val_with_offsets(x, y, -1, -1)?, // top-left
            self.get_data_val_with_offsets(x, y, 0, -1)?,  // top-mid
            self.get_data_val_with_offsets(x, y, 1, -1)?,  // top-right
            self.get_data_val_with_offsets(x, y, -1, 0)?,  // mid-left
            self.get_data_val_with_offsets(x, y, 0, 0)?,   // mid-mid
            self.get_data_val_with_offsets(x, y, 1, 0)?,   // mid-right
            self.get_data_val_with_offsets(x, y, -1, 1)?,  // bot-left
            self.get_data_val_with_offsets(x, y, 0, 1)?,   // bot-mid
            self.get_data_val_with_offsets(x, y, 1, 1)?,   // bot-right
        ];

        self.convert_vec_to_binary(&px_vals)
    }
    fn convert_vec_to_binary(&self, vec: &[i32]) -> Result<u16> {
        // convert to a binary digit
        let mut val = 0;
        for px_val in vec {
            match px_val {
                0 => val <<= 1,
                1 => {
                    val <<= 1;
                    val += 1;
                }
                _ => (),
            }
        }
        Ok(val)
    }

    fn get_data_val_with_offsets(&self, x: i32, y: i32, x_off: i32, y_off: i32) -> Result<i32> {
        // doesn't work with underflow
        if (x + x_off < 0) || (x + x_off >= self.width) {
            return Ok(self.infinite_value);
        }
        if (y + y_off < 0) || (y + y_off >= self.height) {
            return Ok(self.infinite_value);
        }
        match self
            .data
            .get(((y + y_off) * self.width + (x + x_off)) as usize)
        {
            None => Ok(0),
            Some(val) => Ok(*val),
        }
    }

    fn new_image_from_algorithm(&self, algo: &Enhancement) -> Result<Self> {
        let width = self.width + 2;
        let height = self.height + 2;

        let mut data = Vec::new();

        for y in -1..=self.height {
            for x in -1..=self.width {
                let px_value = self.get_pixel_value(x, y)?;

                let new_px_value = algo.data[px_value as usize];

                data.push(new_px_value);
            }
        }

        let new_infinite =
            algo.data[self.convert_vec_to_binary(&[self.infinite_value; 9])? as usize];

        Ok(Self::new(width, height, data, new_infinite))
    }
}

impl Display for InputImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.data.get((y * self.width + x) as usize) {
                    Some(0) => write!(f, ".")?,
                    Some(1) => write!(f, "#")?,
                    _ => (),
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let test_data = vec![
"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
"",
"#..#.",
"#....",
"##..#",
"..#..",
"..###",
        ];

        assert_eq!(compute(&test_data)?, 3351);
        Ok(())
    }

    #[test]
    fn get_algorithm_index_1() -> Result<()> {
        let sut = InputImage::new(3, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1], 0);

        assert_eq!(sut.get_pixel_value(1, 1)?, 511);
        Ok(())
    }

    #[test]
    fn get_algorithm_index_2() -> Result<()> {
        let sut = InputImage::new(3, 3, vec![1, 0, 0, 0, 0, 0, 0, 0, 1], 0);

        assert_eq!(sut.get_pixel_value(1, 1)?, 257);
        Ok(())
    }

    #[test]
    fn get_algorithm_index_3() -> Result<()> {
        let sut = InputImage::new(3, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1], 0);

        assert_eq!(sut.get_pixel_value(0, 0)?, 27);
        Ok(())
    }

    #[test]
    fn get_algorithm_index_4() -> Result<()> {
        let sut = InputImage::new(3, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1], 0);

        assert_eq!(sut.get_pixel_value(2, 2)?, 432);
        Ok(())
    }
}
