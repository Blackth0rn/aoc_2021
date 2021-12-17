use std::{cmp::Ordering, error::Error, fs, slice::Iter};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let target = Rect::new(
        Point(119, -84),
        Point(176, -84),
        Point(119, -141),
        Point(176, -141),
    );
    let output = compute(&input, &target)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(_input: &[&str], target: &Rect) -> Result<i32, Box<dyn Error>> {
    let mut best_max_y = 0;

    for y_vel in 0..200 {
        for x_vel in 0..target.tr.0 / 2 {
            let (hit, max_y) = fire(x_vel, y_vel, target);
            if hit && max_y > best_max_y {
                best_max_y = max_y;
            }
        }
    }
    Ok(best_max_y)
}

struct Point(i32, i32);

struct Rect {
    tl: Point,
    tr: Point,
    bl: Point,
    br: Point,
}

impl Rect {
    fn new(tl: Point, tr: Point, bl: Point, br: Point) -> Self {
        Self { tl, tr, bl, br }
    }

    fn hit(&self, loc: &Point) -> bool {
        loc.0 <= self.tr.0 && loc.0 >= self.tl.0 && loc.1 <= self.tl.1 && loc.1 >= self.bl.1
    }

    fn no_possible_hit(&self, loc: &Point) -> bool {
        loc.0 > self.tr.0 || loc.1 < self.bl.1
    }
}

fn fire(initial_x_vel: i32, initial_y_vel: i32, target: &Rect) -> (bool, i32) {
    let mut loc = Point(0, 0);
    let mut cur_x_vel = initial_x_vel;
    let mut cur_y_vel = initial_y_vel;

    let mut max_y = 0;
    let mut hit = false;
    while !target.no_possible_hit(&loc) && !hit {
        loc.0 += cur_x_vel;
        loc.1 += cur_y_vel;

        if loc.1 > max_y {
            max_y = loc.1;
        }

        match cur_x_vel.cmp(&0) {
            Ordering::Less => cur_x_vel += 1,
            Ordering::Equal => (),
            Ordering::Greater => cur_x_vel -= 1,
        }
        cur_y_vel -= 1;

        hit = target.hit(&loc);
    }

    (hit, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["target area: x=20..30, y=-10..-5"];
        let target = Rect::new(Point(20, -5), Point(30, -5), Point(20, -10), Point(30, -10));
        assert_eq!(compute(&test_data, &target).unwrap(), 45);
    }
}
