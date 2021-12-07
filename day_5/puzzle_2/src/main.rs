use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split(',')
            .map(|val| val.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Self {
            0: data[0],
            1: data[1],
        })
    }
}

#[derive(Debug)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        // need to sort points
        // iterate along them, filling in the details
        // points will be horizontal or vertical

        // find common value
        let mut points = Vec::new();
        if p1.0 == p2.0 {
            //x is common
            for y in std::cmp::min(p1.1, p2.1)..=std::cmp::max(p1.1, p2.1) {
                points.push(Point(p1.0, y));
            }
        } else if p1.1 == p2.1 {
            //y is common
            for x in std::cmp::min(p1.0, p2.0)..=std::cmp::max(p1.0, p2.0) {
                points.push(Point(x, p1.1));
            }
        } else {
            // diagonal line
            let delta_x = match p1.0 - p2.0 {
                x if x > 0 => -1,
                x if x < 0 => 1,
                _ => 0,
            };

            let delta_y = match p1.1 - p2.1 {
                y if y > 0 => -1,
                y if y < 0 => 1,
                _ => 0,
            };

            // iterate from start x to end x and add delta to each point
            let mut current_x = p1.0;
            let mut current_y = p1.1;
            // start point
            points.push(Point(current_x, current_y));
            while current_x != p2.0 {
                current_x += delta_x;
                current_y += delta_y;
                points.push(Point(current_x, current_y));
            }
        }

        Self { points }
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let start = Point::from_str(iter.next().unwrap())?;
        iter.next();
        let end = Point::from_str(iter.next().unwrap())?;
        Ok(Self::new(start, end))
    }
}

fn compute(input: &[&str]) -> Result<i32, Box<dyn Error>> {
    // parse data
    let lines = input
        .iter()
        .map(|val| Line::from_str(val))
        .collect::<Result<Vec<Line>, _>>()?;
    // for line in lines
    // for point in line.points
    // build up hashmap of points
    let mut map: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        for point in line.points {
            let value = map.entry(point).or_insert(0);
            *value += 1;
        }
    }

    // count number of points with value >= 2
    let mut count = 0;
    for (_, val) in map.iter() {
        if *val >= 2 {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        assert_eq!(compute(&test_data).unwrap(), 12)
    }

    #[test]
    fn example2() {
        let test_data = vec!["0,0 -> 0,4", "0,0 -> 0,1"];

        assert_eq!(compute(&test_data).unwrap(), 2)
    }

    #[test]
    fn example3() {
        let test_data = vec!["0,0 -> 0,4", "1,0 -> 1,1"];

        assert_eq!(compute(&test_data).unwrap(), 0)
    }

    #[test]
    fn example4() {
        let test_data = vec!["0,0 -> 0,4", "0,0 -> 0,1", "0,4 -> 2,4"];

        assert_eq!(compute(&test_data).unwrap(), 3)
    }

    #[test]
    fn line_vertical() {
        let start = Point(0, 0);
        let end = Point(0, 4);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(0, 0),
                Point(0, 1),
                Point(0, 2),
                Point(0, 3),
                Point(0, 4)
            ]
        )
    }

    #[test]
    fn line_horizontal() {
        let start = Point(0, 0);
        let end = Point(4, 0);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(2, 0),
                Point(3, 0),
                Point(4, 0)
            ]
        )
    }

    #[test]
    fn line_diagonal_down_right() {
        let start = Point(0, 0);
        let end = Point(4, 4);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(0, 0),
                Point(1, 1),
                Point(2, 2),
                Point(3, 3),
                Point(4, 4)
            ]
        )
    }
    #[test]
    fn line_diagonal_down_left() {
        let start = Point(4, 0);
        let end = Point(0, 4);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(4, 0),
                Point(3, 1),
                Point(2, 2),
                Point(1, 3),
                Point(0, 4)
            ]
        )
    }
    #[test]
    fn line_diagonal_up_left() {
        let start = Point(4, 4);
        let end = Point(0, 0);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(4, 4),
                Point(3, 3),
                Point(2, 2),
                Point(1, 1),
                Point(0, 0)
            ]
        )
    }
    #[test]
    fn line_diagonal_up_right() {
        let start = Point(0, 4);
        let end = Point(4, 0);

        assert_eq!(
            Line::new(start, end).points,
            vec![
                Point(0, 4),
                Point(1, 3),
                Point(2, 2),
                Point(3, 1),
                Point(4, 0)
            ]
        )
    }
}
