use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

#[derive(Debug, PartialEq)]
struct BingoNumber {
    number: i32,
    drawn: bool,
}

impl BingoNumber {
    fn new(number: i32) -> Self {
        Self {
            number,
            drawn: false,
        }
    }
}

impl FromStr for BingoNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            number: s.parse::<i32>().map_err(|err| err.to_string())?,
            drawn: false,
        })
    }
}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<BingoNumber>,
    rows: usize,
    columns: usize,
}

impl BingoBoard {
    fn from_vec(lines: &[&str]) -> Result<Self, Box<dyn Error>> {
        let numbers = lines
            .iter()
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|number| BingoNumber::from_str(number))
            })
            .collect::<Result<Vec<BingoNumber>, String>>()?;
        Ok(Self {
            numbers,
            rows: 5,
            columns: 5,
        })
    }

    fn apply_drawn_number(&mut self, drawn_number: i32) {
        for bingo_number in self.numbers.iter_mut() {
            if bingo_number.number == drawn_number {
                (*bingo_number).drawn = true;
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers.iter().fold(0, |total, bingo_number| {
            total
                + match bingo_number.drawn {
                    true => 0,
                    false => bingo_number.number,
                }
        })
    }

    fn is_complete(&self) -> bool {
        // work out if a row is done
        // work out if a column is done

        for row_idx in 0..self.rows {
            let row_offset = row_idx * self.columns;
            let mut row_complete = true;
            for col_idx in 0..self.columns {
                let value = &self.numbers[row_offset + col_idx];
                row_complete &= value.drawn;
            }
            if row_complete {
                return true;
            }
        }

        for col_idx in 0..self.columns {
            let mut col_complete = true;

            for row_idx in 0..self.rows {
                let value = &self.numbers[row_idx * self.rows + col_idx];
                col_complete &= value.drawn;
            }
            if col_complete {
                return true;
            }
        }

        false
    }
}

fn build_boards(
    input_iter: &mut std::slice::Iter<&str>,
) -> Result<Vec<BingoBoard>, Box<dyn Error>> {
    // need to iterate the lines, grouping into fives and then converting those 5 to a board
    let mut boards = Vec::new();
    let mut board_lines: Vec<&str> = Vec::with_capacity(5);
    for line in input_iter {
        if !line.is_empty() {
            board_lines.push(line);
        }

        if board_lines.len() == 5 {
            boards.push(BingoBoard::from_vec(&board_lines)?);
            board_lines.clear();
        }
    }
    Ok(boards)
}

fn compute(input: &[&str]) -> Result<i32, Box<dyn Error>> {
    let mut input_iter = input.iter();

    let drawn_numbers = input_iter
        .next()
        .ok_or_else(|| String::from("Invalid input for drawn_numbers"))?;

    let drawn_numbers = drawn_numbers
        .split(',')
        .map(|val| val.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    // build the bingo boards
    let mut boards = build_boards(&mut input_iter)?;

    // start applying numbers
    for drawn_number in drawn_numbers {
        for board in boards.iter_mut() {
            board.apply_drawn_number(drawn_number);
        }
        // stop when a winner is found
        for board in &boards {
            if board.is_complete() {
                return Ok(board.sum_unmarked() * drawn_number);
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        assert_eq!(compute(&test_data).unwrap(), 4512)
    }

    #[test]
    fn row_complete() {
        let mut board = BingoBoard::from_vec(&[
            "1 2 3 4 5",
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
        ])
        .unwrap();
        for idx in 0..5 {
            board.numbers[idx].drawn = true;
        }

        assert!(board.is_complete())
    }

    #[test]
    fn col_complete() {
        let mut board = BingoBoard::from_vec(&[
            "1 0 0 0 0",
            "2 0 0 0 0",
            "3 0 0 0 0",
            "4 0 0 0 0",
            "5 0 0 0 0",
        ])
        .unwrap();
        board.numbers[0].drawn = true;
        board.numbers[5].drawn = true;
        board.numbers[10].drawn = true;
        board.numbers[15].drawn = true;
        board.numbers[20].drawn = true;

        assert!(board.is_complete())
    }

    #[test]
    fn col_not_complete() {
        let mut board = BingoBoard::from_vec(&[
            "1 0 0 0 0",
            "2 0 0 0 0",
            "3 0 0 0 0",
            "4 0 0 0 0",
            "5 0 0 0 0",
        ])
        .unwrap();
        board.numbers[0].drawn = true;
        board.numbers[5].drawn = true;
        board.numbers[10].drawn = true;
        board.numbers[15].drawn = true;
        board.numbers[20].drawn = false;

        assert!(!board.is_complete())
    }

    #[test]
    fn test_build_boards() {
        let test_data = vec![
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        assert_eq!(build_boards(&mut test_data.iter()).unwrap().len(), 3)
    }

    #[test]
    fn test_board_from_vec() {
        let test_data = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        assert_eq!(
            BingoBoard::from_vec(&test_data).unwrap().numbers,
            vec![
                BingoNumber::new(22),
                BingoNumber::new(13),
                BingoNumber::new(17),
                BingoNumber::new(11),
                BingoNumber::new(0),
                BingoNumber::new(8),
                BingoNumber::new(2),
                BingoNumber::new(23),
                BingoNumber::new(4),
                BingoNumber::new(24),
                BingoNumber::new(21),
                BingoNumber::new(9),
                BingoNumber::new(14),
                BingoNumber::new(16),
                BingoNumber::new(7),
                BingoNumber::new(6),
                BingoNumber::new(10),
                BingoNumber::new(3),
                BingoNumber::new(18),
                BingoNumber::new(5),
                BingoNumber::new(1),
                BingoNumber::new(12),
                BingoNumber::new(20),
                BingoNumber::new(15),
                BingoNumber::new(19),
            ]
        )
    }

    #[test]
    fn test_sum_unmarked_none_drawn() {
        let test_data = vec![
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 5 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
        ];

        assert_eq!(BingoBoard::from_vec(&test_data).unwrap().sum_unmarked(), 5)
    }

    #[test]
    fn test_sum_unmarked_some_drawn() {
        let test_data = vec![
            "2 3 0 0 0",
            "0 0 0 0 0",
            "0 0 5 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
        ];
        let mut board = BingoBoard::from_vec(&test_data).unwrap();
        board.numbers[0].drawn = true;
        assert_eq!(board.sum_unmarked(), 8)
    }

    #[test]
    fn test_apply_drawn() {
        let test_data = vec![
            "2 3 0 0 0",
            "0 0 0 0 0",
            "0 0 5 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
        ];
        let mut board = BingoBoard::from_vec(&test_data).unwrap();
        board.apply_drawn_number(2);
        assert_eq!(board.sum_unmarked(), 8)
    }
}
