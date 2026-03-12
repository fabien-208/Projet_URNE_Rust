use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

/// Implémentation de la méthode de Bucklin.
/// On cumule les votes rang par rang jusqu'à ce qu'un candidat obtienne la majorité absolue.
pub struct Bucklin;

impl VotingAlgorithm for Bucklin {
    fn name(&self) -> String { "Bucklin".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();
        let total_votes = election.ballots.len();
        let majority = (total_votes / 2) + 1;

        // Pré-calcul parallèle du nombre de fois qu'un candidat apparaît à chaque rang
        let counts_per_rank = election.ballots.par_iter()
            .fold(|| vec![vec![0usize; num_candidates]; num_candidates], |mut local, ballot| {
                for (rank, &candidate) in ballot.ranking.iter().enumerate() {
                    if rank < num_candidates {
                        local[rank][candidate as usize] += 1; // 👈 Fix
                    }
                }
                local
            })
            .reduce(|| vec![vec![0; num_candidates]; num_candidates], |mut t, l| {
                for r in 0..num_candidates {
                    for c in 0..num_candidates { t[r][c] += l[r][c]; }
                }
                t
            });

        let mut scores = vec![0usize; num_candidates];
        let mut round_reached = vec![num_candidates; num_candidates];

        // Cumul itératif des scores rang par rang
        for c in 0..num_candidates {
            let mut total = 0;
            for r in 0..num_candidates {
                total += counts_per_rank[r][c];
                // Si la majorité est atteinte, on enregistre le score gagnant et on arrête le comptage pour lui
                if total >= majority {
                    scores[c] = total; // Score au tour gagnant
                    round_reached[c] = r; // Quel tour ?
                    break;
                }
                // Si pas de majorité, le score est le total cumulé final
                scores[c] = total; 
            }
        }

        let mut ranking: Vec<CandidateId> = (0..num_candidates).map(|i| i as u8).collect(); // 👈 Fix
        
        // Tri final selon le rang le plus bas atteint, puis le meilleur score
        ranking.sort_by_key(|&c| {
            let cu = c as usize;
            (
                round_reached[cu],           // Plus petit tour d'abord
                Reverse(scores[cu]),         // Plus grand score ensuite
                &election.candidates[cu]     // Alpha
            )
        });

        VoteResult { ranking }
    }
}