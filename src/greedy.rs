use std::collections::BinaryHeap;

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

        ordered_plys.peek().unwrap().ply.clone()
    }
}

impl InputHandler for Greedy {
    fn give_new_game_state(&mut self, game_state: GameState) {
        self.current_game_state = Some(game_state);
    }

    fn get_placement(&mut self, available_places: Vec<Ply>) -> Ply {
        let chosen = self.best_child();

        match chosen {
            Placement {..} => {
                assert!(available_places.contains(&chosen),
                    format!("Placement impossible: available_places: {:?}, chosen: {:?}",
                                available_places, chosen));
                chosen
            },
            _ => panic!("Moved from a placement node using {:?}", chosen),
        }
    }

    fn get_move(&mut self, available_moves: Vec<Ply>) -> Ply {
        let chosen = self.best_child();

        match chosen {
            Move {..} => {
                assert!(available_moves.contains(&chosen),
                    format!("Move impossible: available_moves: {:?}, chosen: {:?}",
                                available_moves, chosen));
                chosen
            },
            _ => panic!("Moved from a move node using {:?}", chosen),
        }
    }

    fn get_mill(&mut self, available_mills: Vec<Ply>) -> Ply {
        let chosen = self.best_child();

        match chosen {
            Mill {..} => {
                assert!(available_mills.contains(&chosen),
                    format!("Mill impossible: available_mills: {:?}, chosen: {:?}",
                                available_mills, chosen));
                chosen
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
