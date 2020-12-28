// player module only available for game module.
// making this pub will make it available to whole binary
mod constants;
pub mod player;

use std::{collections::HashMap, error::Error};

use player::PlayerId;

use self::{constants::rules::*, player::Player};

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

    pub fn create_player(&mut self) -> Result<PlayerId, Box<dyn Error>> {
        match self.players.len() {
            MAX_PLAYERS => Err("Max players limit reached")?,
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
