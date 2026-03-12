use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

/// Implémentation de la méthode Plurality (Vote à un tour).
/// Le candidat ayant le plus de premières places l'emporte.
pub struct Plurality;

impl VotingAlgorithm for Plurality {
    fn name(&self) -> String {
        "Plurality".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();

        // MapReduce : Comptage parallèle des votes en première position
        let scores = election.ballots.par_iter()
            .map(|ballot| {
                let mut local_scores = vec![0usize; num_candidates];
                // Seul le premier candidat du bulletin compte
                if let Some(&first_choice) = ballot.ranking.first() {
                    local_scores[first_choice as usize] = 1;
                }
                local_scores
            })
            .reduce(
                || vec![0usize; num_candidates],
                |mut total, local| {
                    for i in 0..num_candidates {
                        total[i] += local[i];
                    }
                    total
                }
            );

        // Création et tri du classement
        let mut ranking: Vec<CandidateId> = (0..num_candidates).map(|i| i as CandidateId).collect();
        // Tri décroissant selon le score, puis ordre alphabétique en cas d'égalité
        ranking.sort_by_key(|&c| (Reverse(scores[c as usize]), &election.candidates[c as usize]));

        VoteResult { ranking }
    }
}