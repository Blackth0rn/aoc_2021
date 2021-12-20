use std::{borrow::BorrowMut, error::Error, fs, ops::Add, str::FromStr};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(_input: &[&str]) -> Result<i32, Box<dyn Error>> {
    // parse input into nested stacks of Numbers
    // reduce item to a number
    //      based off of split/explode semantics
    Ok(0)
}

#[derive(Debug, PartialEq, Clone)]
enum Number {
    Value(u32),
    Pair(Box<Pair>),
}

impl Number {
    fn reduce(&mut self) {
        // exploding is going to be weird
        // as is splitting
        // as they affect this pair and it's parents somehow

        // need to reduce the left and right
        loop {
            let mut action_taken = None;

            // do some things, set action to Some if something has happened
            Number::reduce_worker(self, 1, &mut action_taken);

            if action_taken.is_none() {
                break;
            }
        }
    }

    fn reduce_worker(number: &mut Number, depth: u32, action_taken: &mut Option<Action>) {
        match number {
            Number::Value(_) => {}
            Number::Pair(pair) => {
                // check for explosions
                if depth > 4 {
                    // explode the leftmost pair
                    // get left and right Numbers
                    // set into the action
                    // replace pair with Number?
                    if let (Some(Number::Value(left)), Some(Number::Value(right))) =
                        (pair.left.as_ref(), pair.right.as_ref())
                    {
                        *action_taken = Some(Action::Explode(Some(*left), Some(*right)));
                        // also need to replace with a 0
                        *number = Number::Value(0);
                    }
                    return;
                }

                let mut_pair: &mut Pair = pair.borrow_mut();
                for (i, side) in [mut_pair.left.as_mut(), mut_pair.right.as_mut()]
                    .iter_mut()
                    .enumerate()
                {
                    match side {
                        None => panic!("reducing a pair with a side of None"), // how did we get here??
                        Some(value) => {
                            match value {
                                Number::Value(n) if *n > 10 => {
                                    // split
                                    // need to somehow modify self to be a pair
                                    match *n % 2 {
                                        0 => {
                                            **value = Number::Pair(Box::new(Pair::new(
                                                Number::Value(*n / 2),
                                                Number::Value(*n / 2),
                                            )));
                                        }
                                        1 => {
                                            **value = Number::Pair(Box::new(Pair::new(
                                                Number::Value(*n / 2),
                                                Number::Value((*n / 2) + 1),
                                            )));
                                        }
                                        _ => panic!("mod 2 returned a value outside of [0,1]"),
                                    }
                                    *action_taken = Some(Action::Split);
                                    return;
                                }
                                Number::Pair(pair) => {
                                    // recurse
                                    Number::reduce_worker(
                                        &mut Number::Pair(Box::new(*pair.clone())),
                                        depth + 1,
                                        action_taken,
                                    );
                                    match action_taken {
                                        None => (),
                                        Some(Action::Explode(left, right)) => {
                                            // need to apply the left/right values here and None
                                            // them as necessary
                                            // if side == left && right_explosion is not None, add right explosion value to our
                                            // right value
                                            match i {
                                                0 => {
                                                    // if our left side pair has exploded
                                                    if right.is_some() {
                                                        mut_pair.right
                                                    }
                                                }
                                                1 => {}
                                                _ => (),
                                            }

                                            // if side == right && left_explosion is not None, add left explosion value to our
                                            // left value
                                        }
                                        Some(Action::Split) => {
                                            return;
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        };
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_pair = Self::Pair(Box::new(Pair::new(self, rhs)));

        new_pair.reduce();
        new_pair
    }
}

enum Action {
    Split,
    Explode(Option<u32>, Option<u32>),
}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    left: Option<Number>,
    right: Option<Number>,
}

impl Pair {
    fn new(left: Number, right: Number) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
        }
    }

    fn blank() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for val in s.chars() {
            match val {
                // push a new blank pair onto the stack
                '[' => {
                    stack.push(Pair::blank());
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let value =
                        Some(Number::Value(val.to_digit(10).ok_or_else(|| {
                            format!("Error converting char {} to digit", val)
                        })?));

                    let mut current_pair =
                        stack.pop().expect("Found a digit but the stack was empty");
                    if current_pair.left.is_none() {
                        current_pair.left = value;
                    } else {
                        current_pair.right = value;
                    }

                    stack.push(current_pair);
                }
                ']' => {
                    // get this pair and add to the previous pair as it's left or right value
                    // (depending on what is already set)
                    let finished_pair = stack
                        .pop()
                        .expect("Found a close brace but the stack was empty");

                    let previous_pair = stack.pop();
                    stack.push(match previous_pair {
                        None => finished_pair,
                        Some(mut pair) => {
                            if pair.left.is_none() {
                                pair.left = Some(Number::Pair(Box::new(finished_pair)));
                            } else {
                                pair.right = Some(Number::Pair(Box::new(finished_pair)));
                            }
                            pair
                        }
                    })
                }

                _ => (),
            }
        }

        if stack.is_empty() {
            Err("Couldn't parse number from string".into())
        } else {
            Ok(stack.pop().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["target area: x=20..30, y=-10..-5"];
        assert_eq!(compute(&test_data).unwrap(), 45);
    }

    #[test]
    fn pair_parse_1() {
        let test_data = "[1,2]";
        assert_eq!(
            Pair::from_str(test_data).unwrap(),
            Pair::new(Number::Value(1), Number::Value(2))
        )
    }

    #[test]
    fn pair_parse_2() {
        let test_data = "[[1,2],3]";
        assert_eq!(
            Pair::from_str(test_data).unwrap(),
            Pair::new(
                Number::Pair(Box::new(Pair::new(Number::Value(1), Number::Value(2)))),
                Number::Value(3),
            )
        )
    }

    #[test]
    fn pair_parse_3() {
        let test_data = "[3,[1,2]]";
        assert_eq!(
            Pair::from_str(test_data).unwrap(),
            Pair::new(
                Number::Value(3),
                Number::Pair(Box::new(Pair::new(Number::Value(1), Number::Value(2)))),
            )
        )
    }
}
