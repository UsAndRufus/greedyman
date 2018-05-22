use rand::{thread_rng, Rng};

use nineman::game::*;
use nineman::game::Ply::*;
use nineman::player::InputHandler;

pub struct Greedy {
    pub player_id: i8,
}

impl Greedy {

}

impl InputHandler for Greedy {
    fn give_new_game_state(&mut self, game: GameState) {
        // do nowt
    }
    fn get_placement(&mut self, available_places: Vec<String>) -> String {
        "0nw".to_string()
    }
    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        ("0nw".to_string(), "0ne".to_string())
    }
    fn get_mill(&mut self, available_mills: Vec<String>) -> String {
        "0nw".to_string()
    }
    fn to_string(&self) -> String {
        "Greedy InputHandler".to_string()
    }
    fn set_player_id(&mut self, player_id: i8) {
        self.player_id = player_id;
    }
}
