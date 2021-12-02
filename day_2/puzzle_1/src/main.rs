use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input = raw_input.trim();

    let output = compute(input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split_whitespace().collect();

        // start at the end of the vec
        let parsed_value = (parts
            .pop()
            .ok_or_else::<Self::Err, _>(|| "Invalid value".into())?)
        .parse::<i32>()
        .map_err(|err| err.to_string())?;

        let command = parts
            .pop()
            .ok_or_else::<Self::Err, _>(|| "Invalid command".into())?;

        match command {
            "forward" => Ok(Self::Forward(parsed_value)),
            "down" => Ok(Self::Down(parsed_value)),
            "up" => Ok(Self::Up(parsed_value)),
            _ => Err(format!("Incorrect command found: {}", command)),
        }
    }
}

fn compute(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let cmd = Command::from_str(line)?;
        match cmd {
            Command::Forward(delta) => horizontal_pos += delta,
            Command::Down(delta) => depth += delta,
            Command::Up(delta) => depth -= delta,
        }
    }
    println!("Horizontal position: {}, depth: {}", horizontal_pos, depth);
    Ok(horizontal_pos * depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            compute(
                "forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2"
            )
            .unwrap(),
            150
        )
    }
    #[test]
    fn basic_test() {
        assert_eq!(compute("up 1\nforward 1").unwrap(), -1)
    }
    #[test]
    fn no_input() {
        // start at 0, 0
        // anything * 0 = 0
        assert_eq!(compute("up 3").unwrap(), 0)
    }
}
