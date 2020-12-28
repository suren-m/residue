use game::Game;

// entry for other modules

pub mod game;
pub mod player;

pub fn setup() -> Game {
    let g = Game::new();

    g
}
