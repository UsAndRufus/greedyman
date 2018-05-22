use std::collections::BinaryHeap;

use std::io;
use std::io::prelude::*;

use rand::{thread_rng, Rng};

use nineman::game::*;
use nineman::game::Ply::*;
use nineman::player::InputHandler;

use score::Score;

pub struct Greedy {
    pub player_id: i8,
    pub current_game_state: Option<GameState>,
}

impl Greedy {
    fn best_child(&self) -> Ply {

        let children = match self.current_game_state {
            Some(ref gs) => gs.children(),
            None => return Root,
        };

        let id = self.player_id;

        let ordered_plys: BinaryHeap<Score>
            = children.iter()
                .map(|c| Score{ score: c.player_score(id), ply: c.ply_to_get_here.clone() })
                .collect();

        println!("ordered_plys: {:?}", ordered_plys);

        // let mut line = String::new();
        // io::stdin().read_line(&mut line).expect("Failed to read line");

        ordered_plys.peek().unwrap().ply.clone()
    }
}

impl InputHandler for Greedy {
    fn give_new_game_state(&mut self, game_state: GameState) {
        self.current_game_state = Some(game_state);
    }

    fn get_placement(&mut self, available_places: Vec<String>) -> String {
        let chosen = self.best_child();

        match chosen {
            Placement {piece_id, ..} => {
                assert!(available_places.contains(&piece_id),
                    format!("Placement impossible: available_places: {:?}, piece_id: {}", available_places, piece_id));
                piece_id
            },
            _ => panic!("Moved from a placement node using {:?}", chosen),
        }
    }

    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        let chosen = self.best_child();

        match chosen {
            Move {mv, ..} => {
                assert!(available_moves.contains(&mv),
                    format!("Move impossible: available_moves: {:?}, mv: {:?}", available_moves, mv));
                mv
            },
            _ => panic!("Moved from a move node using {:?}", chosen),
        }
    }

    fn get_mill(&mut self, available_mills: Vec<String>) -> String {
        //thread_rng().choose(&available_mills).unwrap().to_string()

        let chosen = self.best_child();

        match chosen {
            Mill {piece_id, ..} => {
                assert!(available_mills.contains(&piece_id),
                    format!("Mill impossible: available_mills: {:?}, piece_id: {}", available_mills, piece_id));
                piece_id
            },
            _ => panic!("Moved from a mill node using {:?}", chosen),
        }
    }

    fn to_string(&self) -> String {
        "Greedy InputHandler".to_string()
    }

    fn set_player_id(&mut self, player_id: i8) {
        self.player_id = player_id;
    }
}
