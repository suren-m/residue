mod game;
use game::Game;

fn main() {
    println!("\n...starting...\n");
    let mut g = Game::new();
    g.create_player();

    let players = g.get_players();
    for p in players {
        dbg!(p);
    }
    // let mut g = Game::new();
    // g.create_player();
    // g.create_player();
    // g.create_player();
    // g.create_player();

    // let players = g.get_players();
    // let mut board: [[char; 3]; 4] = [['.'; 3]; 4];

    // board[1][2] = p1.get_avatar();

    // 'outer: for i in board.iter() {
    //     'inner: for j in i.iter() {
    //         print!("{}\t", j.to_owned());
    //     }
    //     println!("\n");
    // }
    // let position = p1.get_position();

    // println!("{:?}", p1);
}
