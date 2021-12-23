use std::{collections::HashSet, error::Error, fs, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<u32> {
    let mut cubes_on = HashSet::new();

    let mut steps = Vec::new();
    let clamp_range = CoordRange(-50, 50);
    for line in input {
        let mut step = RebootStep::from_str(line)?;
        // clamp steps outside of -50, 50

        if step.clamp(&clamp_range, &clamp_range, &clamp_range) {
            steps.push(step);
        }
    }

    for step in steps {
        for x in step.x_range.to_range() {
            for y in step.y_range.to_range() {
                for z in step.z_range.to_range() {
                    let cube = Cube(x, y, z);

                    match step.action {
                        OnOff::On => {
                            cubes_on.insert(cube);
                        }
                        OnOff::Off => {
                            cubes_on.remove(&cube);
                        }
                    }
                }
            }
        }
    }
    Ok(cubes_on.len().try_into()?)
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Cube(i32, i32, i32);

enum OnOff {
    On,
    Off,
}

impl FromStr for OnOff {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOff::On),
            "off" => Ok(OnOff::Off),
            _ => Err(format!("Unknown string {}", s)),
        }
    }
}

struct CoordRange(i32, i32);

impl CoordRange {
    fn overlaps(&self, other: &Self) -> bool {
        // rules for overlaps
        // 5....10
        //  6.8
        if (self.1 < other.0) || (self.0 > other.1) {
            return false;
        }
        true
    }

    fn clamp(&mut self, other: &Self) {
        if self.0 < other.0 {
            self.0 = other.0
        }

        if self.1 > other.1 {
            self.1 = other.1;
        }
    }

    fn to_range(&self) -> std::ops::RangeInclusive<i32> {
        std::ops::RangeInclusive::new(self.0, self.1)
    }
}

impl FromStr for CoordRange {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // split at =
        // use part[1]
        // split at '..',
        // use parts 0,1 for start,end
        let mut iter = s.split('=');
        iter.next(); // drop the axis specifier

        let mut num_iter = iter.next().unwrap().split("..");

        let start = num_iter
            .next()
            .unwrap()
            .parse::<i32>()
            .map_err(|err| err.to_string())?;
        let end = num_iter
            .next()
            .unwrap()
            .parse::<i32>()
            .map_err(|err| err.to_string())?;

        Ok(Self(start, end))
    }
}

struct RebootStep {
    x_range: CoordRange,
    y_range: CoordRange,
    z_range: CoordRange,
    action: OnOff,
}

impl RebootStep {
    fn new(x_range: CoordRange, y_range: CoordRange, z_range: CoordRange, action: OnOff) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
            action,
        }
    }

    fn clamp(&mut self, x_range: &CoordRange, y_range: &CoordRange, z_range: &CoordRange) -> bool {
        // need to clamp to the given ranges if they overlap
        if x_range.overlaps(&self.x_range)
            && y_range.overlaps(&self.y_range)
            && z_range.overlaps(&self.z_range)
        {
            // clamp the ranges
            self.x_range.clamp(x_range);
            self.y_range.clamp(y_range);
            self.z_range.clamp(z_range);
            return true;
        }
        false
    }
}

impl FromStr for RebootStep {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let action = OnOff::from_str(iter.next().unwrap())?;

        let mut ranges = iter.next().unwrap().split(',');

        let x_range = CoordRange::from_str(ranges.next().unwrap())?;
        let y_range = CoordRange::from_str(ranges.next().unwrap())?;
        let z_range = CoordRange::from_str(ranges.next().unwrap())?;

        Ok(Self::new(x_range, y_range, z_range, action))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let test_data = vec![
            "on x=10..12,y=10..12,z=10..12",
            "on x=11..13,y=11..13,z=11..13",
            "off x=9..11,y=9..11,z=9..11",
            "on x=10..10,y=10..10,z=10..10",
        ];

        assert_eq!(compute(&test_data)?, 39);
        Ok(())
    }

    #[test]
    fn example2() -> Result<()> {
        let test_data = vec![
            "on x=-20..26,y=-36..17,z=-47..7",
            "on x=-20..33,y=-21..23,z=-26..28",
            "on x=-22..28,y=-29..23,z=-38..16",
            "on x=-46..7,y=-6..46,z=-50..-1",
            "on x=-49..1,y=-3..46,z=-24..28",
            "on x=2..47,y=-22..22,z=-23..27",
            "on x=-27..23,y=-28..26,z=-21..29",
            "on x=-39..5,y=-6..47,z=-3..44",
            "on x=-30..21,y=-8..43,z=-13..34",
            "on x=-22..26,y=-27..20,z=-29..19",
            "off x=-48..-32,y=26..41,z=-47..-37",
            "on x=-12..35,y=6..50,z=-50..-2",
            "off x=-48..-32,y=-32..-16,z=-15..-5",
            "on x=-18..26,y=-33..15,z=-7..46",
            "off x=-40..-22,y=-38..-28,z=23..41",
            "on x=-16..35,y=-41..10,z=-47..6",
            "off x=-32..-23,y=11..30,z=-14..3",
            "on x=-49..-5,y=-3..45,z=-29..18",
            "off x=18..30,y=-20..-8,z=-3..13",
            "on x=-41..9,y=-7..43,z=-33..15",
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
            "on x=967..23432,y=45373..81175,z=27513..53682",
        ];

        assert_eq!(compute(&test_data)?, 590784);
        Ok(())
    }
}
