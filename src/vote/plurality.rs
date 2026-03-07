use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

pub struct Plurality;

impl VotingAlgorithm for Plurality {
    fn name(&self) -> String {
        "Plurality".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();

        //chaque thread compte les premiers choix
        let scores = election.ballots.par_iter()
            .fold(
                || vec![0usize; num_candidates],
                |mut local_scores, ballot| {
                    if let Some(&first_choice) = ballot.ranking.first() {
                        local_scores[first_choice as usize] += 1; 
                    }
                    local_scores
                }
            )
            .reduce(
                || vec![0usize; num_candidates],
                |mut total, local| {
                    for i in 0..num_candidates {
                        total[i] += local[i];
                    }
                    total
                }
            );

        //création du classement final
        let mut ranking: Vec<CandidateId> = (0..num_candidates).map(|i| i as u8).collect();

        ranking.sort_by_key(|&c| (Reverse(scores[c as usize]), &election.candidates[c as usize]));

        VoteResult { ranking }
    }
}