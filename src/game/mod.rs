// player module only available for game module.
// making this pub will make it available to whole binary
mod constants;

use std::{collections::HashMap, error::Error};

use crate::player::{Player, PlayerId};

use self::constants::rules::*;

pub struct Game {
    players: HashMap<PlayerId, Player>,
    board: [[char; BOARD_SIZE_X]; BOARD_SIZE_Y],
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
            board: [['x'; BOARD_SIZE_X]; BOARD_SIZE_Y],
        }
    }

    pub fn create_player(&mut self) -> Result<PlayerId, String> {
        match self.players.len() {
            MAX_PLAYERS => Err("Max players limit reached".to_string()),
            _ => {
                let current_capacity = self.players.len();
                let id = (current_capacity + 1) as u32;

                let new_player_id = PlayerId(id);
                self.players
                    .insert(new_player_id.clone(), Player::new(PlayerId(id)));

                Ok(new_player_id)
            }
        }
    }

    pub fn get_players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }

    pub fn render(&mut self) {
        'outer: for i in self.board.iter() {
            'inner: for j in i.iter() {
                print!("{}\t", j.to_owned());
            }
            println!("\n");
        }
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn max_player_limit() {
        let mut g = Game::new();
        for _ in 1..=MAX_PLAYERS {
            let is_successful = g.create_player().is_ok();
            assert_eq!(is_successful, true);
        }
        // can't create more than MAX_PLAYERS
        let p = g.create_player();
        let res = p.is_err();
        assert_eq!(res, true);
    }
}
