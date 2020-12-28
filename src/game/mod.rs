// player module only available for game module.
// making this pub will make it available to whole binary
mod player;

use self::player::Player;
use std::error::Error;

const MAX_PLAYERS: u8 = 4;

// new game?
pub struct Game {
    players: Vec<Player>,
    max_players: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: Vec::new(),
            max_players: MAX_PLAYERS,
        }
    }

    pub fn create_player(&mut self) {
        let p = Player::new();
        self.players.push(p);
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn add_player(&mut self, p: Player) -> Result<(), Box<dyn Error>> {
        if self.players.len() >= self.max_players.into() {
            Err("Max Capacity Reached".to_owned())?
        } else {
            Ok(self.players.push(p))
        }
    }
}
