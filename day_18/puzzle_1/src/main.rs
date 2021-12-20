use std::{
    cell::RefCell,
    error::Error,
    fs,
    ops::Add,
    rc::{Rc, Weak},
    str::FromStr,
};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(_input: &[&str]) -> Result<i32, Box<dyn Error>> {
    Ok(0)
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

#[derive(Debug)]
struct Number {
    root: RefCell<Rc<Node>>,
}

impl Number {
    fn new() -> Self {
        Self {
            root: RefCell::new(Rc::new(Node::new())),
        }
    }

    fn append(parent: &Rc<Node>, child: &Rc<Node>, side: Side) {
        match side {
            Side::Lhs => {
                if parent.lhs.borrow().is_none() {
                    *parent.lhs.borrow_mut() = Rc::clone(child);
                }
            }
            Side::Rhs => {
                if parent.rhs.borrow().is_none() {
                    *parent.rhs.borrow_mut() = Rc::clone(child);
                }
            }
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        // make a new number, root is a new node, lhs is self.root, rhs is rhs.root
        let new_number = Self::new();
        let new_root_node = Rc::new(Node::new());

        *new_number.root.borrow_mut() = Rc::clone(&new_root_node);

        Number::append(&new_root_node, &self.root.borrow(), Side::Lhs);
        Number::append(&new_root_node, &rhs.root.borrow(), Side::Rhs);

        // TODO need to add a reduction step in here

        new_number
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // make a new number
        let mut number = Number::new();
        // for each [ make a new Node
        // for each number make a new node with value
        let mut stack = Vec::new();
        for val in s.chars() {
            match val {
                // push a new blank pair onto the stack
                '[' => {
                    stack.push(Rc::new(Node::new()));
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let value = val
                        .to_digit(10)
                        .ok_or_else(|| format!("Error converting char {} to digit", val))?;

                    let current_node = stack.pop().expect("Found a digit but the stack was empty");

                    if current_node.lhs.borrow().is_none() {
                        Node::leaf(value, Side::Lhs, &current_node);
                    } else {
                        Node::leaf(value, Side::Rhs, &current_node);
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
                            number.root = RefCell::new(Rc::clone(&current_node));
                        }
                        Some(node) => {
                            if node.lhs.borrow().is_none() {
                                Number::append(&node, &current_node, Side::Lhs);
                            } else if node.rhs.borrow().is_none() {
                                Number::append(&node, &current_node, Side::Rhs);
                            }
                            stack.push(node);
                        }
                    }
                }

                _ => (),
            }
        }
        Ok(number)
    }
}

#[derive(Debug)]
enum Side {
    Lhs,
    Rhs,
}

#[derive(Debug)]
struct Node {
    lhs: RefCell<Rc<Option<Node>>>,
    rhs: RefCell<Rc<Option<Node>>>,
    parent: RefCell<Weak<Node>>,
    value: Option<u32>,
}

impl Node {
    fn new() -> Self {
        Self {
            lhs: RefCell::new(Rc::new(None)),
            rhs: RefCell::new(Rc::new(None)),
            parent: RefCell::new(Default::default()),
            value: None,
        }
    }

    fn leaf(value: u32, side: Side, parent: &Rc<Self>) {
        let node = Self {
            lhs: RefCell::new(Rc::new(None)),
            rhs: RefCell::new(Rc::new(None)),
            parent: RefCell::new(Rc::downgrade(parent)),
            value: Some(value),
        };

        match side {
            Side::Lhs => *parent.lhs.borrow_mut() = Rc::new(Some(node)),
            Side::Rhs => *parent.rhs.borrow_mut() = Rc::new(Some(node)),
        }
    }

    fn try_split(node: &mut Rc<Node>) -> bool {
        // if it should split then split
        // split turns the node from a value node into a pair node
        match node.value {
            Some(n) if n > 10 => {
                // make some new nodes
                // append them to this node
                // set this node's value to None
                let lhs = Node::leaf(n / 2, Side::Lhs, node);
                let rhs = match n.cmp(&10) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        Node::leaf(n / 2, Side::Rhs, node)
                    }
                    std::cmp::Ordering::Greater => Node::leaf(n / 2 + 1, Side::Rhs, node),
                };

                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_basic() -> Result<(), Box<dyn Error>> {
        let test_data = "[1,2]";

        let number = Number::from_str(test_data)?;

        assert_eq!(
            number
                .root
                .borrow()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1
        );

        assert_eq!(
            number
                .root
                .borrow()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            2
        );

        Ok(())
    }

    #[test]
    fn test_parse_nested() -> Result<(), Box<dyn Error>> {
        let test_data = "[[1,2],3]";

        let number = Number::from_str(test_data)?;

        assert_eq!(
            number
                .root
                .borrow()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1
        );

        assert_eq!(
            number
                .root
                .borrow()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            3
        );

        Ok(())
    }

    #[test]
    fn test_parse_sibling() -> Result<(), Box<dyn Error>> {
        let test_data = "[[1,2],[3,4]]";

        let number = Number::from_str(test_data)?;

        assert_eq!(
            number
                .root
                .borrow()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            2
        );

        assert_eq!(
            number
                .root
                .borrow()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            3
        );

        Ok(())
    }

    #[test]
    fn number_add() {
        let lhs = Number::from_str("[1,2]").unwrap();
        let rhs = Number::from_str("[1,2]").unwrap();

        let new_number = lhs + rhs;

        assert_eq!(
            new_number
                .root
                .borrow()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1
        );
        assert_eq!(
            new_number
                .root
                .borrow()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            2
        );
        assert_eq!(
            new_number
                .root
                .borrow()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .lhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1
        );
        assert_eq!(
            new_number
                .root
                .borrow()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .rhs
                .borrow()
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            2
        );
    }
}
