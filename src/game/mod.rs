// player module only available for game module.
// making this pub will make it available to whole binary
mod constants;
mod player;

use std::error::Error;

use self::{constants::rules::*, player::Player};

pub struct Game {
    players: Vec<Player>,
    board: [[char; BOARD_SIZE_X]; BOARD_SIZE_Y],
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: Vec::new(),
            board: [['x'; BOARD_SIZE_X]; BOARD_SIZE_Y],
        }
    }

    pub fn create_player(&mut self) -> Result<i32, Box<dyn Error>> {
        if let MAX_PLAYERS = self.players.len() {
            Err("Max players limit reached")?
        } else {
            let new_player_id: i32 = (self.players.len() + 1) as i32; 
            let p = Player::new(new_player_id);
            self.players.push(p);
            Ok(new_player_id)
        }
    }

    pub fn get_players(&self) -> &Vec<Player> {
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
