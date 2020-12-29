use std::fmt;

use self::position::Position;

pub mod position;

const MAX_HEALTH: u8 = 100;
pub enum Command<T> {
    Left(T),
    Right(T),
    Up(T),
    Down(T),
}

// note the pub keyword for fields of tuple struct
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct PlayerId(pub u32);

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    name: String,
    avatar: char,
    health: u8,
    active: bool,
    position: Position,
}

impl Player {
    pub fn new(id: PlayerId, avatar: char) -> Self {
        let name = "Player".to_owned() + &id.to_string();

        Player {
            id: id,
            name: name,
            avatar: avatar,
            health: MAX_HEALTH,
            active: true,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn get_position(&self) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y,
        }
    }

    pub fn set_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }

    pub fn get_avatar(&self) -> char {
        self.avatar
    }
    pub fn travel(cmd: Command<u8>) {}
}
