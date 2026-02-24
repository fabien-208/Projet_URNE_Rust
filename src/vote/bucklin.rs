use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

pub struct Bucklin;

impl VotingAlgorithm for Bucklin {
    fn name(&self) -> String {
        "Bucklin".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();
        let total_votes = election.ballots.len();
        let majority_threshold = total_votes / 2; // Majorité absolue (> 50%)

        //Chaque cœur compte sa part des bulletins
        let rank_counts = election.ballots.par_iter()
            .fold(
                || vec![vec![0; num_candidates]; num_candidates],
                |mut local_counts, ballot| {
                    for (rank, &candidate) in ballot.ranking.iter().enumerate() {
                        if rank < num_candidates {
                            local_counts[rank][candidate] += 1;
                        }
                    }
                    local_counts
                }
            )
            .reduce(
                || vec![vec![0; num_candidates]; num_candidates],
                |mut total_counts, local_counts| {
                    for r in 0..num_candidates {
                        for c in 0..num_candidates {
                            total_counts[r][c] += local_counts[r][c];
                        }
                    }
                    total_counts
                }
            );

        // Tableau pour stocker les scores cumulés
        let mut scores = vec![0; num_candidates];

        // On déroule l'algorithme : round par round (rang 0, puis rang 1, etc.)
        for rank in 0..num_candidates {
            let mut majority_reached = false;

            for c in 0..num_candidates {
                scores[c] += rank_counts[rank][c];
                
                if scores[c] > majority_threshold {
                    majority_reached = true;
                }
            }

            if majority_reached {
                break;
            }
        }

        // Création du classement final
        let mut ranking: Vec<CandidateId> = (0..num_candidates).collect();
        
        ranking.sort_by_key(|&c| (
            Reverse(scores[c]),      // Le plus grand score d'abord
            &election.candidates[c]  // Ordre alphabétique en cas d'égalité absolue
        ));

        VoteResult { ranking }
    }
}