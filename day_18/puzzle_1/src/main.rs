use std::{error::Error, fmt::Display, fs, ops::Add, str::FromStr};

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
    let mut numbers = Vec::new();
    for line in input {
        numbers.push(Node::from_str(line)?);
    }

    // reduce numbers by adding
    let mut iter = numbers.into_iter();
    let mut sum = iter.next().unwrap();
    for node in iter {
        sum = sum + node;
    }
    Ok(sum.magnitude())
}

// each number is a tree
// leafs are u32, branches are pairs

// [[1,2],3]
// ==
//      root pair
//      /  \
//    pair  3
//    /  \
//   1    2

// trees are traditionally hard in rust
// due to memory safety and the borrow checker
//
// what if we store nodes as indexes into an array?
// this would seem to make removing nodes harder, we'd have to effectively just leave them as
// garbage or do a bulk update of the tree after a removal (updating all the nodes indices)
// we definitely need to remove nodes (explosion)

// each node can have a value or a pair of nodes (lhs, rhs)
// that would normally look like an enum (value(u32), pair(Rc<RefCell<node>>, node))

#[derive(Debug)]
enum Side {
    Lhs,
    Rhs,
}

#[derive(Debug)]
struct Node {
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    value: Option<u32>,
}

impl Node {
    fn new() -> Self {
        Self {
            lhs: None,
            rhs: None,
            value: None,
        }
    }

    fn leaf(value: u32) -> Self {
        Self {
            lhs: None,
            rhs: None,
            value: Some(value),
        }
    }

    fn magnitude(&self) -> u32 {
        match self.value {
            Some(n) => n,
            None => {
                self.lhs.as_ref().unwrap().magnitude() * 3 + self.rhs.as_ref().unwrap().magnitude()
            }
        }
    }

    fn increase(&mut self, side: Side, value: u32) {
        match self.value {
            None => match side {
                Side::Lhs => {
                    self.lhs.as_mut().unwrap().increase(side, value);
                }
                Side::Rhs => {
                    self.rhs.as_mut().unwrap().increase(side, value);
                }
            },
            Some(n) => self.value = Some(n + value),
        };
    }

    fn reduce(&mut self) {
        // need to go through and do the explodes and splits

        loop {
            let explosion = self.try_explode(1);

            let split = self.try_split();

            if explosion.is_none() && !split {
                break;
            }
        }
    }

    fn try_explode(&mut self, depth: u8) -> Option<(Option<u32>, Option<u32>)> {
        // if we're a pair at depth > 4 then we should explode
        //      set our value to 0, return our previous values for adding to our neighbour nodes
        match self.value {
            None => {
                if depth > 4 {
                    // explode!
                    let lhs = self.lhs.take();
                    let rhs = self.rhs.take();
                    self.value = Some(0);
                    if lhs.as_ref().unwrap().lhs.is_some() {
                        println!("{:?} {:?}", lhs, rhs);
                    }
                    Some((lhs.unwrap().value, rhs.unwrap().value))
                } else {
                    let mut left_over_explosion_values = (None, None);
                    // if the lhs of this pair exploded, add the rhs explosion value to our rhs's
                    // lhs value
                    let lhs_action = self.lhs.as_mut().unwrap().try_explode(depth + 1);
                    match lhs_action {
                        Some((Some(lhs), Some(rhs))) => {
                            // fresh explosion
                            self.rhs.as_mut().unwrap().increase(Side::Lhs, rhs);
                            left_over_explosion_values.0 = Some(lhs);
                        }
                        Some((Some(lhs), None)) => {
                            left_over_explosion_values.0 = Some(lhs);
                        }
                        Some((None, Some(rhs))) => {
                            self.rhs.as_mut().unwrap().increase(Side::Lhs, rhs);
                        }
                        None => {
                            // maybe we should explode here?
                            // because we've gone down the tree and are now working our way back
                            // up?
                            let rhs_action = self.rhs.as_mut().unwrap().try_explode(depth + 1);
                            match rhs_action {
                                Some((Some(lhs), Some(rhs))) => {
                                    // fresh explosion
                                    self.lhs.as_mut().unwrap().increase(Side::Rhs, lhs);
                                    left_over_explosion_values.1 = Some(rhs);
                                }
                                Some((Some(lhs), None)) => {
                                    self.lhs.as_mut().unwrap().increase(Side::Rhs, lhs);
                                }
                                Some((None, Some(rhs))) => {
                                    left_over_explosion_values.1 = Some(rhs);
                                }
                                None => (),
                                _ => panic!(
                                    "Got Some(None, None) for rhs action, shouldn't be possible"
                                ),
                            };
                        }
                        _ => panic!("Got Some(None, None) for lhs action, shouldn't be possible"),
                    }
                    match left_over_explosion_values {
                        (None, None) => None,
                        _ => Some(left_over_explosion_values),
                    }
                }
            }
            Some(_) => None,
        }
    }

    fn try_split(&mut self) -> bool {
        // if value > 10, split into a new node
        match self.value {
            None => {
                let lhs_action = self.lhs.as_mut().unwrap().try_split();
                match lhs_action {
                    false => self.rhs.as_mut().unwrap().try_split(),
                    true => true,
                }
            }
            Some(n) if n >= 10 => {
                // actually do a split
                // make a new node for each value
                // assign nodes to lhs and rhs
                let lhs_value = n / 2;
                let rhs_value = match n % 2 {
                    0 => n / 2,
                    1 => (n / 2) + 1,
                    _ => panic!("Mod 2 returned none 0 or 1 value"),
                };

                let lhs = Node::leaf(lhs_value);
                let rhs = Node::leaf(rhs_value);

                self.lhs = Some(Box::new(lhs));
                self.rhs = Some(Box::new(rhs));
                self.value = None;

                true
            }
            _ => false,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match on value, if None, write a [ then recurse into the lhs,
        match self.value {
            None => {
                write!(f, "[")?;
                self.lhs.as_ref().unwrap().fmt(f)?;
                write!(f, ",")?;
                self.rhs.as_ref().unwrap().fmt(f)?;
                write!(f, "]")?;
            }
            Some(n) => {
                write!(f, "{}", n)?;
            }
        }
        Ok(())
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_node = Node::new();
        new_node.lhs = Some(Box::new(self));
        new_node.rhs = Some(Box::new(rhs));

        new_node.reduce();

        new_node
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // for each [ make a new Node
        // for each number make a new node with value
        let mut root_node = None;

        let mut stack = Vec::new();
        for val in s.chars() {
            match val {
                // push a new blank pair onto the stack
                '[' => {
                    stack.push(Node::new());
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let value = val
                        .to_digit(10)
                        .ok_or_else(|| format!("Error converting char {} to digit", val))?;

                    let mut current_node =
                        stack.pop().expect("Found a digit but the stack was empty");

                    if current_node.lhs.is_none() {
                        current_node.lhs = Some(Box::new(Node::leaf(value)));
                    } else {
                        current_node.rhs = Some(Box::new(Node::leaf(value)));
                    }

                    stack.push(current_node);
                }
                ']' => {
                    // we need to link this pair to the parent pair
                    // it's not the last item on the stack though, that could be a sibling pair
                    // ie. [[1,2],[3,4]]
                    //                ^- at this point, the 1,2 pair is the last on the stack and
                    //                shouldn't be added to
                    // unless we don't add this to the stack once done, and leave it linked only
                    // via the tree
                    let current_node = stack.pop().expect("Malformed number");
                    match stack.pop() {
                        None => {
                            // stack is empty, the current node is the only node
                            // can add the current node to the number as the root
                            root_node = Some(current_node);
                        }
                        Some(mut node) => {
                            if node.lhs.is_none() {
                                node.lhs = Some(Box::new(current_node));
                            } else if node.rhs.is_none() {
                                node.rhs = Some(Box::new(current_node));
                            }
                            stack.push(node);
                        }
                    }
                }

                _ => (),
            }
        }
        Ok(root_node.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let test_data = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        assert_eq!(compute(&test_data)?, 4140);
        Ok(())
    }
    #[test]
    fn test_parse_basic() -> Result<()> {
        let test_data = "[1,2]";

        assert_eq!(format!("{}", Node::from_str(test_data)?), test_data);
        Ok(())
    }

    #[test]
    fn test_parse_nested() -> Result<()> {
        let test_data = "[[1,2],3]";

        assert_eq!(format!("{}", Node::from_str(test_data)?), test_data);
        Ok(())
    }

    #[test]
    fn test_parse_sibling() -> Result<()> {
        let test_data = "[[1,2],[3,4]]";

        assert_eq!(format!("{}", Node::from_str(test_data)?), test_data);
        Ok(())
    }

    #[test]
    fn test_addition_basic() -> Result<()> {
        let lhs = Node::from_str("[1,2]")?;
        let rhs = Node::from_str("[3,4]")?;

        assert_eq!(format!("{}", lhs + rhs), "[[1,2],[3,4]]");
        Ok(())
    }

    #[test]
    fn test_addition_explode() -> Result<()> {
        let lhs = Node::from_str("[1,1]")?;
        let rhs = Node::from_str("[[[[4,4],3,],2],1]")?;

        assert_eq!(format!("{}", lhs + rhs), "[[1,5],[[[0,7],2],1]]");
        Ok(())
    }

    #[test]
    fn test_explode_1() -> Result<()> {
        let mut node = Node::from_str("[[[[[9,8],1],2],3],4]")?;

        node.try_explode(1);
        assert_eq!(format!("{}", node), "[[[[0,9],2],3],4]");
        Ok(())
    }

    #[test]
    fn test_explode_2() -> Result<()> {
        let mut node = Node::from_str("[7,[6,[5,[4,[3,2]]]]]")?;

        node.try_explode(1);
        assert_eq!(format!("{}", node), "[7,[6,[5,[7,0]]]]");
        Ok(())
    }

    #[test]
    fn test_explode_3() -> Result<()> {
        let mut node = Node::from_str("[[6,[5,[4,[3,2]]]],1]")?;

        node.try_explode(1);
        assert_eq!(format!("{}", node), "[[6,[5,[7,0]]],3]");
        Ok(())
    }

    #[test]
    fn test_addition_explode_split() -> Result<()> {
        let lhs = Node::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")?;
        let rhs = Node::from_str("[1,1]")?;

        assert_eq!(
            format!("{}", lhs + rhs),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
        Ok(())
    }
}
