use std::ops::Add;

#[derive(Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Add for Position {
    type Output = Position;
    
    fn add(self, other: Position) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
