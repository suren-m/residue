use std::ops::Add;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod player_tests {

    use super::*;

    #[test]
    fn addition_of_two_positions(){
        let p1 = Position{ x: 1, y: 1};
        let p2 = Position{ x: 2, y: 2};
        
        let res = p1 + p2;
        let exp = Position { x: 3, y: 3};
        
        assert_eq!(res, exp);
    }
}