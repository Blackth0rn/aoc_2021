use std::{error::Error, fs};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<u64, Box<dyn Error>> {
    // for corrupt chunks they have to close with the wrong character
    // so we need some form of recursive parsing to make sure the characters that close a chunk are
    // valid

    let mut valid_scores: Vec<u64> = Vec::new();
    for line in input {
        let (_data, valid, score) = parse_chunk(line, 0);

        if valid {
            valid_scores.push(score);
        }
    }
    valid_scores.sort_unstable();

    // find middle score
    // middle index = len / 2
    let middle_index = valid_scores.len() / 2;
    let score = valid_scores[middle_index];
    Ok(score)
}

fn parse_chunk(data: &str, score: u64) -> (&str, bool, u64) {
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
        return ("", true, score);
    }

    let first_char = first_char.unwrap();

    if first_char == ')' || first_char == ']' || first_char == '>' || first_char == '}' {
        return (data, true, score);
    }

    let (rem_data, valid, score) = parse_chunk(chars.as_str(), score);

    if !valid {
        return (rem_data, valid, score);
    }

    let mut rem_chars = rem_data.chars();
    let first_rem_char = rem_chars.next();

    // unended line, still valid
    // now we need to end the line validly
    if first_rem_char == None {
        return match first_char {
            '(' => ("", true, score * 5 + 1),
            '[' => ("", true, score * 5 + 2),
            '{' => ("", true, score * 5 + 3),
            '<' => ("", true, score * 5 + 4),
            _ => ("", true, score),
        };
    }

    let first_rem_char = first_rem_char.unwrap();

    if (first_char == '(' && first_rem_char == ')')
        || (first_char == '[' && first_rem_char == ']')
        || (first_char == '<' && first_rem_char == '>')
        || (first_char == '{' && first_rem_char == '}')
    {
        return parse_chunk(rem_chars.as_str(), score);
    }

    (rem_data, false, score)
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
        assert_eq!(compute(&test_data).unwrap(), 288957)
    }
}
