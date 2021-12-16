use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Node {
    x: usize,
    y: usize,
    cost: u32,
}

struct Graph {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
}

impl Graph {
    fn new(nodes: Vec<Node>, width: usize, height: usize) -> Self {
        Self {
            nodes,
            width,
            height,
        }
    }

    fn get_node(&self, x: i32, y: i32) -> Option<&Node> {
        if y < 0
            || y == self.height.try_into().unwrap()
            || x < 0
            || x == self.width.try_into().unwrap()
        {
            None
        } else {
            Some(&self.nodes[y as usize * self.width + x as usize])
        }
    }
}

fn compute(input: &[&str]) -> Result<u32, Box<dyn Error>> {
    // parse input to nodes
    let mut nodes = Vec::new();
    let height = input.len();
    let width = input[0].len();

    for y_tile in 0..5 {
        for (y, line) in input.iter().enumerate() {
            for x_tile in 0..5 {
                for (x, val) in line.chars().enumerate() {
                    let mut cost = val.to_digit(10).unwrap() + y_tile + x_tile;
                    if cost > 9 {
                        cost %= 9;
                    }
                    nodes.push(Node {
                        x: x + (x_tile as usize * width),
                        y: y + (y_tile as usize * height),
                        cost,
                    });
                }
            }
        }
    }

    println!("Map built, size: {}", nodes.len());

    let mut graph = Graph::new(nodes, width * 5, height * 5);
    // run a* on nodes
    let mut best_path = a_star(&mut graph);

    // skip the start node
    best_path.pop();
    let mut cost = 0;
    for node in best_path {
        cost += node.cost;
    }
    Ok(cost)
}

fn estimate_cost(node: &Node, width: usize, height: usize) -> u32 {
    let distance: u32 = ((width - 1 - node.x) + (height - 1 - node.y))
        .try_into()
        .unwrap();
    distance
}

fn a_star(graph: &mut Graph) -> Vec<&Node> {
    let start = &graph.nodes[0];
    let goal = &graph.nodes[graph.nodes.len() - 1];

    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut actual_costs = HashMap::new();
    actual_costs.insert(start, 0);

    let mut estimated_costs: HashMap<&Node, u32> = HashMap::new();
    estimated_costs.insert(start, estimate_cost(start, graph.width, graph.height));

    while !open_set.is_empty() {
        println!("Iterating, open_set size: {}", open_set.len());
        // iterate open_set, finding lowest estimated cost
        let mut lowest_cost: Option<&Node> = None;
        for node in &open_set {
            match lowest_cost {
                None => lowest_cost = Some(node),
                Some(low_cost_node) => {
                    if estimated_costs[node] < estimated_costs[low_cost_node] {
                        lowest_cost = Some(graph.get_node(node.x as i32, node.y as i32).unwrap())
                    }
                }
            }
        }
        let current = lowest_cost.unwrap();

        if current == goal {
            return reconstruct_path(&came_from, current);
        }

        open_set.remove(&current);

        let mut tmp_set = Vec::new();

        let neighbours = vec![
            graph.get_node(current.x as i32 - 1, current.y.try_into().unwrap()), // left
            graph.get_node(current.x.try_into().unwrap(), current.y as i32 + 1), // bottom
            graph.get_node(current.x as i32 + 1, current.y.try_into().unwrap()), // right
            graph.get_node(current.x.try_into().unwrap(), current.y as i32 - 1), // top
        ];

        for neighbour_node in neighbours.into_iter().flatten() {
            let tentative_cost = actual_costs[&current] + neighbour_node.cost;

            let actual_cost = actual_costs.entry(neighbour_node).or_insert(u32::MAX);
            if tentative_cost < *actual_cost {
                *actual_cost = tentative_cost;

                came_from.insert(neighbour_node, current);

                estimated_costs.insert(
                    neighbour_node,
                    tentative_cost + estimate_cost(neighbour_node, graph.width, graph.height),
                );
                if !open_set.contains(neighbour_node) {
                    tmp_set.push(neighbour_node);
                }
            }
        }
        open_set.extend(tmp_set);
    }

    vec![]
}

fn reconstruct_path<'a, 'b>(
    came_from: &'b HashMap<&'a Node, &'a Node>,
    current: &'a Node,
) -> Vec<&'a Node> {
    let mut path = vec![current];
    let mut tmp_current = current;
    while came_from.contains_key(tmp_current) {
        tmp_current = came_from[tmp_current];
        path.push(tmp_current);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];
        assert_eq!(compute(&test_data).unwrap(), 315);
    }
}
