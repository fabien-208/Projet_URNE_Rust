use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*;

pub struct Copeland;

impl VotingAlgorithm for Copeland {
    fn name(&self) -> String { "Copeland".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();

        // Division des calculs de la matrice
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0usize; n]; n],
                |mut local_wins, ballot| {
                    // Calcul de la position de chaque candidat dans le bulletin
                    let mut pos = vec![usize::MAX; n];
                    for (idx, &c) in ballot.ranking.iter().enumerate() {
                        pos[c as usize] = idx;
                    }
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if pos[i] < pos[j] { local_wins[i][j] += 1; } 
                            else if pos[j] < pos[i] { local_wins[j][i] += 1; }
                        }
                    }
                    local_wins
                })
            .reduce(
                || vec![vec![0; n]; n],
                |mut total_wins, local_wins| {
                    for i in 0..n { for j in 0..n { total_wins[i][j] += local_wins[i][j]; } }
                    total_wins
                });

        // Calcul des scores de chaque candidat
        let mut scores = vec![0isize; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if wins[i][j] > wins[j][i] {
                    scores[i] += 2;
                }
                else if wins[i][j] == wins[j][i] {
                    scores[i] += 1;
                }
            }
        }

        // Tri des candidats par score
        let mut ranking: Vec<CandidateId> = (0..scores.len()).map(|i| i as u8).collect();
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i as usize]), &election.candidates[i as usize]));

        VoteResult { ranking }
    }
}