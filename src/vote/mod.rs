use crate::types::{Election, VoteResult};

pub trait VotingAlgorithm {
    fn name(&self) -> String;

    fn compute(&self, election: &Election) -> VoteResult;
}

pub mod baldwin;
pub mod borda;
pub mod bucklin;
pub mod copeland;
pub mod copeland_borda;
pub mod irv;
pub mod plurality;
pub mod schulze;
pub mod smith_irv;
pub mod trs;
