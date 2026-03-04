use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*;

pub struct Copeland;

impl Copeland {
    pub fn get_score(election: &crate::types::Election) -> Vec<isize> {
        let n = election.candidates.len();

        // Division des calculs de la matrice
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0; n]; n],
                |mut local_wins, v| {
                    // Chaque thread a son propre tableau `pos` pour éviter les collisions
                    let mut pos = vec![usize::MAX; n];
                    
                    for (idx, &c) in v.ranking.iter().enumerate() {
                        pos[c] = idx;
                    }

                    // On met à jour la matrice locale des duels
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if pos[i] < pos[j] {
                                local_wins[i][j] += 1;
                            } else if pos[j] < pos[i] {
                                local_wins[j][i] += 1;
                            }
                        }
                    }
                    local_wins
                }
            )
            .reduce(
                || vec![vec![0; n]; n],
                |mut total_wins, local_wins| {
                    for i in 0..n {
                        for j in 0..n {
                            total_wins[i][j] += local_wins[i][j];
                        }
                    }
                    total_wins
                }
            );

        let mut scores = vec![0; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if wins[i][j] > wins[j][i] {
                    scores[i] += 1;
                    scores[j] -= 1;
                } else if wins[j][i] > wins[i][j] {
                    scores[j] += 1;
                    scores[i] -= 1;
                }
            }
        }

        scores
    }
}

impl VotingAlgorithm for Copeland {
    fn name(&self) -> String { "Copeland".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let scores = Copeland::get_score(election);
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));
        VoteResult { ranking }
    }
}