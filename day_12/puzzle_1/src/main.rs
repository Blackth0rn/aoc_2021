use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    hash::{Hash, Hasher},
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
    let caves = build_caves_map(input)?;

    // we have a map of caves
    // now we need to get all the possible ways to get from start to end without using a lowercase
    // cave more than once
    let mut in_progress_routes = Vec::new();
    let mut finished_routes = HashSet::new();

    // set up initial conditions
    in_progress_routes.push(Route::new("start"));

    // loop over routes
    while !in_progress_routes.is_empty() {
        // pop a route
        let route = in_progress_routes.pop().unwrap();
        // look up the connections for it's end
        let connections = caves
            .get(&route.current_end)
            .ok_or(format!("Bad cave: {}", &route.current_end))?;

        for connection in connections {
            if route.can_add_cave(connection) {
                let mut new_route = Route::from_route(&route);

                new_route.add_cave(connection);

                if new_route.is_done() {
                    finished_routes.insert(new_route);
                } else {
                    in_progress_routes.push(new_route);
                }
            }
        }
    }

    Ok(finished_routes.len())
}

fn build_caves_map<'a>(
    input: &'a [&str],
) -> Result<HashMap<&'a str, Vec<&'a str>>, Box<dyn Error>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input {
        let mut cave_iter = line.split('-');
        let start = cave_iter.next().unwrap();
        let end = cave_iter.next().unwrap();

        if !caves.contains_key(start) {
            caves.insert(start, Vec::new());
        }

        if !caves.contains_key(end) {
            caves.insert(end, Vec::new());
        }

        caves
            .get_mut(start)
            .ok_or_else(|| format!("Invalid key: {}", start))?
            .push(end);
        caves
            .get_mut(end)
            .ok_or_else(|| format!("Invalid key: {}", end))?
            .push(start);
    }

    Ok(caves)
}

#[derive(Debug, Clone)]
struct Route<'a> {
    so_far: Vec<&'a str>,
    current_end: &'a str,
    used_little_caves: HashSet<&'a str>,
}

impl<'a> Route<'a> {
    fn new(start: &'a str) -> Self {
        let mut hs = HashSet::new();
        hs.insert(start);
        Self {
            so_far: vec![start],
            current_end: start,
            used_little_caves: hs,
        }
    }

    fn from_route(base: &Self) -> Self {
        base.clone()
    }

    fn can_add_cave(&self, cave: &str) -> bool {
        !self.used_little_caves.contains(cave)
    }

    fn is_done(&self) -> bool {
        self.current_end == "end"
    }

    fn add_cave(&mut self, cave: &'a str) {
        self.so_far.push(cave);
        self.current_end = cave;
        // if string is lower case (cave is little) the add to used little caves
        if cave.chars().all(|val| val.is_lowercase()) {
            self.used_little_caves.insert(cave);
        }
    }
}

// only care about the route for comparison in hash sets
impl<'a> Hash for Route<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.so_far.hash(state);
    }
}

impl<'a> PartialEq for Route<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.so_far == other.so_far
    }
}

impl<'a> Eq for Route<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        assert_eq!(compute(&test_data).unwrap(), 10);
    }

    #[test]
    fn example2() {
        let test_data = vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ];
        assert_eq!(compute(&test_data).unwrap(), 19)
    }

    #[test]
    fn example3() {
        let test_data = vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ];
        assert_eq!(compute(&test_data).unwrap(), 226)
    }
}
