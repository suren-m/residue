use game::{constants::avatars, Game};
use player::PlayerId;

// entry for other modules

pub mod game;
pub mod player;

pub fn setup() -> Game {
    let mut g = Game::new();

    g
}
