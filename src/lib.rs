// entry for other modules

pub mod game;
use game::{
    player::{position::Position, PlayerId},
    Game,
};

// Game Setup

pub fn setup() -> Game {
    let g = Game::new();


    g

}
