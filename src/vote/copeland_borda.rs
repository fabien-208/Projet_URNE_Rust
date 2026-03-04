use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

pub struct CopelandBorda;

impl VotingAlgorithm for CopelandBorda {
    fn name(&self) -> String {
        return "Copeland-Borda".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();
        
        
        let (wins, borda_scores) = election.ballots.par_iter()
            .fold(
                || (vec![vec![0; n]; n], vec![0isize; n]), // Etat local par cœur
                |mut local_state, v| {
                    let (ref mut local_wins, ref mut local_borda) = local_state;
                    let mut pos = vec![usize::MAX; n];
                    
                    for (idx, &c) in v.ranking.iter().enumerate() {
                        pos[c] = idx;
                        // On calcule Borda au passage
                        local_borda[c] += (n - idx - 1) as isize;
                    }

                    // On calcule les duels Copeland au même moment
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if pos[i] < pos[j] {
                                local_wins[i][j] += 1;
                            } else if pos[j] < pos[i] {
                                local_wins[j][i] += 1;
                            }
                        }
                    }
                    local_state
                }
            )
            .reduce(
                || (vec![vec![0; n]; n], vec![0isize; n]),
                |mut total_state, local_state| {
                    let (ref mut total_wins, ref mut total_borda) = total_state;
                    let (local_wins, local_borda) = local_state;
                    
                    for i in 0..n {
                        total_borda[i] += local_borda[i];
                        for j in 0..n {
                            total_wins[i][j] += local_wins[i][j];
                        }
                    }
                    total_state
                }
            );

        // On finalise les scores de Copeland à partir de la matrice fusionnée
        let mut copeland_scores = vec![0isize; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if wins[i][j] > wins[j][i] {
                    copeland_scores[i] += 1;
                    copeland_scores[j] -= 1;
                } else if wins[j][i] > wins[i][j] {
                    copeland_scores[j] += 1;
                    copeland_scores[i] -= 1;
                }
            }
        }

        //Classement Final
        let mut ranking: Vec<CandidateId> = (0..n).collect();
        
        // On trie : Copeland d'abord, Borda si égalité
        ranking.sort_by_key(|&i| (
            Reverse(copeland_scores[i]), 
            Reverse(borda_scores[i]), 
            &election.candidates[i]
        ));

        VoteResult { ranking }
    }
}