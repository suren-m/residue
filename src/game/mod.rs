pub mod constants;
use self::{board::Board, constants::rules::*};

use std::collections::HashMap;

use crate::player::{position::Position, Player, PlayerId};

mod board;

pub struct Game {
    players: HashMap<PlayerId, Player>,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
            board: Board([['x'; BOARD_SIZE_X]; BOARD_SIZE_Y]),
        }
    }

    pub fn create_player(&mut self, avatar: char) -> Result<PlayerId, String> {
        match self.players.len() {
            MAX_PLAYERS => Err("Max players limit reached".to_string()),
            _ => {
                let current_capacity = self.players.len();
                let id = (current_capacity + 1) as u32;

                let new_player_id = PlayerId(id);
                self.players
                    .insert(new_player_id.clone(), Player::new(PlayerId(id), avatar));

                Ok(new_player_id)
            }
        }
    }

    pub fn get_players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }

    pub fn render(&self) -> &Game {
        'outer: for i in self.board.0.iter() {
            'inner: for j in i.iter() {
                print!("{}\t", j.to_owned());
            }
            println!("\n");
        }
        self
    }

    pub fn move_player(mut self, p_id: &PlayerId, new_pos: Position) -> Game {
        let p = self.players.get_mut(p_id).unwrap();
        let old_pos = p.get_position();
        let avatar = p.get_avatar();

        self.board.0[old_pos.x as usize][old_pos.y as usize] = '*';
        self.board.0[new_pos.x as usize][new_pos.y as usize] = avatar;

        p.set_position(new_pos);

        self
    }
}

#[cfg(test)]
mod game_tests {
    use super::{
        constants::{avatars, rules::MAX_PLAYERS},
        Game,
    };

    #[test]
    fn max_player_limit() {
        let mut g = Game::new();
        for _ in 1..=MAX_PLAYERS {
            let is_successful = g.create_player(avatars::SANTA).is_ok();
            assert_eq!(is_successful, true);
        }
        // can't create more than MAX_PLAYERS
        let p = g.create_player(avatars::TUKTUK);
        let res = p.is_err();
        assert_eq!(res, true);
    }
}
