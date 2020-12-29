use super::constants::rules::*;

#[derive(Copy, Clone, Debug)]
pub struct Board(pub [[char; BOARD_SIZE_X]; BOARD_SIZE_Y]);
