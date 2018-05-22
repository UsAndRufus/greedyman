use std::cmp::Ordering;

use nineman::game::Ply;

#[derive(Eq,Debug)]
pub struct Score {
    pub score: i8,
    pub ply: Ply,
}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.score == other.score
    }
}
