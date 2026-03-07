pub trait VotingAlgorithm: Sync {
    fn name(&self) -> String;
    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult;
}

pub mod plurality;
pub mod trs;
pub mod irv;
pub mod borda;
pub mod bucklin;
pub mod baldwin;
pub mod copeland;
pub mod copeland_borda;
pub mod schulze;
pub mod smith_irv;