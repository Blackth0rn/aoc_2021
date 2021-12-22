use std::{error::Error, fmt::Display, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    // let raw_input = fs::read_to_string(INPUT_PATH)?;
    // let _input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(8, 10)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(player_1_pos: u8, player_2_pos: u8) -> Result<u32> {
    let mut player_1 = CurrentPlayer::Player1(Player::new(player_1_pos));
    let mut player_2 = CurrentPlayer::Player2(Player::new(player_2_pos));

    let mut current_player = &mut player_2;

    let mut die = Die::new();

    while !current_player.has_won() {
        match current_player {
            CurrentPlayer::Player1(_) => current_player = &mut player_2,
            CurrentPlayer::Player2(_) => current_player = &mut player_1,
        }
        let roll = die.roll_3();

        current_player.move_position(roll);
    }

    let losing_player_score = match current_player {
        CurrentPlayer::Player1(_) => {
            if let CurrentPlayer::Player2(p) = player_2 {
                p.score
            } else {
                panic!("Odd things happening here")
            }
        }
        CurrentPlayer::Player2(_) => {
            if let CurrentPlayer::Player1(p) = player_1 {
                p.score
            } else {
                panic!("Odd things happening here")
            }
        }
    };

    Ok(losing_player_score * die.rolls)
}

#[derive(Debug, Copy, Clone)]
enum CurrentPlayer {
    Player1(Player),
    Player2(Player),
}

impl CurrentPlayer {
    fn has_won(&self) -> bool {
        match self {
            CurrentPlayer::Player1(p) => p.has_won(),
            CurrentPlayer::Player2(p) => p.has_won(),
        }
    }

    fn move_position(&mut self, roll: u16) {
        match self {
            CurrentPlayer::Player1(p) => p.move_position(roll),
            CurrentPlayer::Player2(p) => p.move_position(roll),
        }
    }
}

#[derive(Debug)]
struct Die {
    current_val: u8,
    rolls: u32,
}

impl Die {
    fn new() -> Self {
        Self {
            current_val: 0,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> u8 {
        self.rolls += 1;

        let mut next_value = self.current_val + 1;
        if next_value > 100 {
            next_value = 1;
        }
        self.current_val = next_value;
        next_value
    }

    fn roll_3(&mut self) -> u16 {
        let mut roll_3 = 0_u16;
        roll_3 += self.roll() as u16;
        roll_3 += self.roll() as u16;
        roll_3 += self.roll() as u16;

        roll_3
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    position: u8,
    score: u32,
}

impl Player {
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    fn move_position(&mut self, roll: u16) {
        self.position += (roll % 10) as u8;
        if self.position > 10 {
            self.position -= 10;
        }

        self.score += self.position as u32;
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        assert_eq!(compute(4, 8)?, 739785);
        Ok(())
    }
}
