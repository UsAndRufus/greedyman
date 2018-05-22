extern crate greedyman;
extern crate nineman;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::Human;
use nineman::player::Random;

use greedyman::Greedy;

fn main() {
    let p1 = Player::new(String::from("Daphne"), 1, Box::new(Random {}));

    let p2 = Player::new(String::from("Gary"), 2,
                            Box::new(Greedy { player_id: 2, current_game_state: None }));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.game_loop();

}
