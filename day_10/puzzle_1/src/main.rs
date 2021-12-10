use std::{error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<u32, Box<dyn Error>> {
    // for corrupt chunks they have to close with the wrong character
    // so we need some form of recursive parsing to make sure the characters that close a chunk are
    // valid

    let mut score = 0;
    for line in input {
        let (data, valid) = parse_chunk(line);

        if !valid {
            let invalid_char = data.chars().next().unwrap();

            match invalid_char {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => (),
            };
        }
    }
    Ok(score)
}

fn parse_chunk(data: &str) -> (&str, bool) {
    // look at first character
    //      if it's an open char then we need to pop it and recurse
    //      if it's a close char we should return, something else will need to check it
    //
    //      with recursed data we need to check if it's an open char?
    //
    //
    //
    // (()())
    //
    // pop first, recurse
    // pop second, recurse
    // return third
    // compare third with second, if valid, recurse?
    //
    // that means we do,
    // pop, if open recurse
    // if close return
    //
    // with recursed data, if valid, return recurse?
    // if not valid return incorrect char, invalid

    let mut chars = data.chars();
    let first_char = chars.next();

    // if we're done then we're valid
    if first_char == None {
        return ("", true);
    }

    let first_char = first_char.unwrap();

    if first_char == ')' || first_char == ']' || first_char == '>' || first_char == '}' {
        return (data, true);
    }

    let (rem_data, valid) = parse_chunk(chars.as_str());

    if !valid {
        return (rem_data, valid);
    }

    let mut rem_chars = rem_data.chars();
    let first_rem_char = rem_chars.next();

    // unended line, still valid
    if first_rem_char == None {
        return ("", true);
    }

    let first_rem_char = first_rem_char.unwrap();

    if (first_char == '(' && first_rem_char == ')')
        || (first_char == '[' && first_rem_char == ']')
        || (first_char == '<' && first_rem_char == '>')
        || (first_char == '{' && first_rem_char == '}')
    {
        return parse_chunk(rem_chars.as_str());
    }

    (rem_data, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(compute(&test_data).unwrap(), 26397)
    }
}
