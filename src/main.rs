use residue::{self, game::Player};

fn main() {
    println!("\n...starting...\n");
    let p = Player::new();

    let mut board: [[char; 3]; 4] = [['.'; 3]; 4];

    board[1][2] = p.get_avatar();

    'outer: for i in board.iter() {
        'inner: for j in i.iter() {
            print!("{}\t", j.to_owned());
        }
        println!("\n");
    }
    let position = p.get_position();

    println!("{:?}", p);
}
