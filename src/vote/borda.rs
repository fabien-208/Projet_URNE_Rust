use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*;

pub struct Borda;

impl Borda {
    pub fn get_score(election: &crate::types::Election) -> Vec<isize> {
        let num_candidates = election.candidates.len();

        // 
        let scores = election.ballots.par_iter()
            .fold(
                || vec![0isize; num_candidates], 
                
                |mut thread_scores, v| {
                    for (pos, &c) in v.ranking.iter().enumerate() {
                        thread_scores[c] += (num_candidates - pos - 1) as isize;
                    }
                    thread_scores
                }
            )
            .reduce(
                || vec![0isize; num_candidates], 
                
                |mut total_scores, thread_scores| {
                    for i in 0..num_candidates {
                        total_scores[i] += thread_scores[i];
                    }
                    total_scores
                }
            );

        scores
    }
}

impl VotingAlgorithm for Borda {
    fn name(&self) -> String { "Borda".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let scores = Borda::get_score(election);
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));
        VoteResult { ranking }
    }
}