use crate::{VotingAlgorithm};

pub struct Plurality;

impl VotingAlgorithm for Plurality {
    fn name(&self) -> String {
        todo!()
    }

    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult {
        todo!()
    }
}