use residue::{
    game::constants::{avatars, rules::BOARD_SIZE_X},
    player::position::Position,
    setup,
};
//use rand::Rng;
fn main() {
    println!("\n...starting...\n");

    let mut g = setup();

    let p1_id = g.create_player(avatars::PUMPKIN).unwrap();

    //let mut rng = rand::thread_rng();
    {
        // let randx = rng.gen_range(0, BOARD_SIZE_X);
        // let randy = rng.gen_range(0, BOARD_SIZE_Y);

        let new_pos = Position { x: 2, y: 4 };

        let g = g.move_player(&p1_id, new_pos);

        g.render();
    }
}
