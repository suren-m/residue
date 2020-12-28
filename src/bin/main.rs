use residue::game::{
    player::{position::Position, PlayerId},
    Game,
};

fn main() {
    println!("\n...starting...\n");
    let mut g = Game::new();

    for i in 1..6 {
        match g.create_player() {
            Ok(p) => println!("{:?}", p),
            Err(e) => println!("{}", e),
        };
    }

    let x = g.get_players().get(&PlayerId(1));
    println!("{:?}", x);

    let xpos = Position { x: 2, y: 3 };
    let ypos = Position { x: 1, y: 1 };

    let res = xpos + ypos;
    println!("{:?}", res);

    // let players = g.get_players();
    // for p in players {
    //     dbg!(p);
    // }

    // g.render();
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
