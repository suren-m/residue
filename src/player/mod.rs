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
    pub fn new(id: PlayerId) -> Self {
        Player {
            id: id,
            name: "demo".to_owned(),
            avatar: '🍕',
            health: MAX_HEALTH,
            active: true,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn get_position(&self) -> (u8, u8) {
        (self.position.x, self.position.y)
    }

    pub fn get_avatar(&self) -> char {
        self.avatar
    }
    pub fn travel(cmd: Command<u8>) {}
}
