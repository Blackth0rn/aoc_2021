use crate::Player::{Player1, Player2};
use std::{collections::HashMap, error::Error, fmt::Display, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    // let raw_input = fs::read_to_string(INPUT_PATH)?;
    // let _input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(8, 10)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(player_1_pos: u8, player_2_pos: u8) -> Result<u64> {
    let mut game_states: HashMap<GameState, u64> = HashMap::new();
    let mut new_states: HashMap<GameState, u64> = HashMap::new();

    // mapping of total roll count to the number of states that could result in that number
    // ie. rolls of 1,1,1 can only happen once, so only one new universe is spawned with that roll
    // count
    let die_map = HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

    new_states.insert(GameState::new(player_1_pos, player_2_pos), 1);

    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    while !new_states.is_empty() {
        game_states = new_states.clone();
        new_states.clear();

        for (current_state, universe_total) in game_states.iter() {
            for (roll, spawned_universes) in &die_map {
                let mut new_state = *current_state;
                new_state.next(*roll);
                let new_universe_count = universe_total * *spawned_universes;

                match new_state.has_winner() {
                    None => {
                        // push the game state onto the map with the updated universe count
                        let universe_count = new_states.entry(new_state).or_insert(0);
                        *universe_count += new_universe_count;
                    }
                    Some(Player1) => {
                        // increment p1 win count
                        player_1_wins += new_universe_count;
                    }
                    Some(Player2) => {
                        // increment p2 win count
                        player_2_wins += new_universe_count;
                    }
                };
            }
        }
        game_states.clear();
        println!("new_states count: {}", new_states.len());
    }
    println!("{} vs {}", player_1_wins, player_2_wins);
    Ok(player_1_wins.max(player_2_wins))
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Player {
    Player1,
    Player2,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct GameState {
    p1: PlayerState,
    p2: PlayerState,
    player_turn: Player,
}

impl GameState {
    fn new(p1_pos: u8, p2_pos: u8) -> Self {
        Self {
            p1: PlayerState::new(p1_pos),
            p2: PlayerState::new(p2_pos),
            player_turn: Player1,
        }
    }

    fn next(&mut self, roll: u8) {
        match self.player_turn {
            Player1 => {
                self.p1.update(roll);
                self.player_turn = Player2;
            }
            Player2 => {
                self.p2.update(roll);
                self.player_turn = Player1;
            }
        }
    }

    fn has_winner(&self) -> Option<Player> {
        if self.p1.score >= 21 {
            return Some(Player1);
        } else if self.p2.score >= 21 {
            return Some(Player2);
        }
        None
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct PlayerState {
    pos: u8,
    score: u8,
}

impl PlayerState {
    fn new(pos: u8) -> Self {
        Self { pos, score: 0 }
    }

    fn update(&mut self, roll: u8) {
        self.pos += roll;
        if self.pos > 10 {
            self.pos -= 10;
        }
        self.score += self.pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        assert_eq!(compute(4, 8)?, 444356092776315);
        Ok(())
    }
}
