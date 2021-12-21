use std::{collections::HashMap, error::Error, fs, str::FromStr};

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
    let scanners = create_scanners(input)?;
    println!("Number of scanners: {}", scanners.len());

    // all of our axis rotations
    let rotations = vec![
        Rotation {
            x_rot: Sign::Pos,
            y_rot: Sign::Pos,
            z_rot: Sign::Pos,
        },
        Rotation {
            x_rot: Sign::Neg,
            y_rot: Sign::Pos,
            z_rot: Sign::Pos,
        },
        Rotation {
            x_rot: Sign::Pos,
            y_rot: Sign::Neg,
            z_rot: Sign::Pos,
        },
        Rotation {
            x_rot: Sign::Pos,
            y_rot: Sign::Pos,
            z_rot: Sign::Neg,
        },
        Rotation {
            x_rot: Sign::Neg,
            y_rot: Sign::Neg,
            z_rot: Sign::Pos,
        },
        Rotation {
            x_rot: Sign::Pos,
            y_rot: Sign::Neg,
            z_rot: Sign::Neg,
        },
        Rotation {
            x_rot: Sign::Neg,
            y_rot: Sign::Pos,
            z_rot: Sign::Neg,
        },
        Rotation {
            x_rot: Sign::Neg,
            y_rot: Sign::Neg,
            z_rot: Sign::Neg,
        },
    ];

    // for each scanner we need to map to world coords (which is scanner 0's coords)
    // this involves finding the rotation and translation that moves all the beacons in a scanner
    // into world coords
    // These are found by comparing each rotated beacon set with the base one and seeing the beacon
    // offsets match up

    Ok(0)
}

fn create_scanners(input: &[&str]) -> Result<Vec<Scanner>> {
    let mut scanners = Vec::new();
    let mut tmp_beacons = Vec::new();
    let mut label = String::new();

    for line in input {
        if line.is_empty() {
            scanners.push(Scanner::with_beacons(label.clone(), tmp_beacons));
            tmp_beacons = Vec::new();
            label.clear();
        } else if line.starts_with("---") {
            // scanner line
            label = line.to_string();
        } else {
            // beacon
            tmp_beacons.push(Beacon::from_str(line)?);
        }
    }

    // catch if we have beacons left over
    if !tmp_beacons.is_empty() {
        scanners.push(Scanner::with_beacons(label, tmp_beacons));
    }

    Ok(scanners)
}

#[derive(Debug, Clone)]
struct Scanner {
    label: String,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(label: String) -> Self {
        Self {
            label,
            beacons: vec![],
        }
    }

    fn with_beacons(label: String, beacons: Vec<Beacon>) -> Self {
        Self { label, beacons }
    }
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn apply_rotation(&self, rot: Rotation) -> Point {
        let new_x = rot.x_rot.apply(self.x);
        let new_y = rot.y_rot.apply(self.y);
        let new_z = rot.z_rot.apply(self.z);

        Point::new(new_x, new_y, new_z)
    }
}

enum Sign {
    Pos,
    Neg,
}

impl Sign {
    fn apply(&self, value: i32) -> i32 {
        match self {
            Sign::Pos => value,
            Sign::Neg => -value,
        }
    }
}
struct Rotation {
    x_rot: Sign,
    y_rot: Sign,
    z_rot: Sign,
}

#[derive(Debug, Clone)]
struct Beacon {
    local_loc: Point,
    world_loc: Option<Point>,
}

impl Beacon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            local_loc: Point::new(x, y, z),
            world_loc: None,
        }
    }
}

impl FromStr for Beacon {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // x,y,z
        let coords = s
            .split(',')
            .map(|val| val.parse::<i32>().map_err(|err| err.to_string()))
            .collect::<std::result::Result<Vec<i32>, _>>()?;

        Ok(Beacon::new(coords[0], coords[1], coords[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let test_data = vec![
            "--- scanner 0 ---",
            "404,-588,-901",
            "528,-643,409",
            "-838,591,734",
            "390,-675,-793",
            "-537,-823,-458",
            "-485,-357,347",
            "-345,-311,381",
            "-661,-816,-575",
            "-876,649,763",
            "-618,-824,-621",
            "553,345,-567",
            "474,580,667",
            "-447,-329,318",
            "-584,868,-557",
            "544,-627,-890",
            "564,392,-477",
            "455,729,728",
            "-892,524,684",
            "-689,845,-530",
            "423,-701,434",
            "7,-33,-71",
            "630,319,-379",
            "443,580,662",
            "-789,900,-551",
            "459,-707,401",
            "",
            "--- scanner 1 ---",
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390",
            "",
            "--- scanner 2 ---",
            "649,640,665",
            "682,-795,504",
            "-784,533,-524",
            "-644,584,-595",
            "-588,-843,648",
            "-30,6,44",
            "-674,560,763",
            "500,723,-460",
            "609,671,-379",
            "-555,-800,653",
            "-675,-892,-343",
            "697,-426,-610",
            "578,704,681",
            "493,664,-388",
            "-671,-858,530",
            "-667,343,800",
            "571,-461,-707",
            "-138,-166,112",
            "-889,563,-600",
            "646,-828,498",
            "640,759,510",
            "-630,509,768",
            "-681,-892,-333",
            "673,-379,-804",
            "-742,-814,-386",
            "577,-820,562",
            "",
            "--- scanner 3 ---",
            "-589,542,597",
            "605,-692,669",
            "-500,565,-823",
            "-660,373,557",
            "-458,-679,-417",
            "-488,449,543",
            "-626,468,-788",
            "338,-750,-386",
            "528,-832,-391",
            "562,-778,733",
            "-938,-730,414",
            "543,643,-506",
            "-524,371,-870",
            "407,773,750",
            "-104,29,83",
            "378,-903,-323",
            "-778,-728,485",
            "426,699,580",
            "-438,-605,-362",
            "-469,-447,-387",
            "509,732,623",
            "647,635,-688",
            "-868,-804,481",
            "614,-800,639",
            "595,780,-596",
            "",
            "--- scanner 4 ---",
            "727,592,562",
            "-293,-554,779",
            "441,611,-461",
            "-714,465,-776",
            "-743,427,-804",
            "-660,-479,-426",
            "832,-632,460",
            "927,-485,-438",
            "408,393,-506",
            "466,436,-512",
            "110,16,151",
            "-258,-428,682",
            "-393,719,612",
            "-211,-452,876",
            "808,-476,-593",
            "-575,615,604",
            "-485,667,467",
            "-680,325,-822",
            "-627,-443,-432",
            "872,-547,-609",
            "833,512,582",
            "807,604,487",
            "839,-516,451",
            "891,-625,532",
            "-652,-548,-490",
            "30,-46,-14",
        ];

        assert_eq!(compute(&test_data)?, 1);
        Ok(())
    }
}
