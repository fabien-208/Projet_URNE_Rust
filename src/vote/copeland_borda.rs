use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*;

/// Implémentation de la méthode de Copeland avec le score de Borda comme mécanisme de départage.
pub struct CopelandBorda;

impl VotingAlgorithm for CopelandBorda {
    fn name(&self) -> String { "Copeland+Borda".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();

        // Calcul des scores de Copeland et Borda en une seule passe parallèle
        let (wins, borda_scores) = election.ballots.par_iter()
            .fold(
                || (vec![vec![0usize; n]; n], vec![0isize; n]),
                |(mut local_wins, mut local_borda), ballot| {
                    let mut pos = vec![usize::MAX; n];
                    for (idx, &c) in ballot.ranking.iter().enumerate() {
                        let cu = c as usize;
                        pos[cu] = idx;
                        // Attribution des points Borda
                        local_borda[cu] += (n - idx - 1) as isize;
                    }
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if pos[i] < pos[j] { local_wins[i][j] += 1; }
                            else if pos[j] < pos[i] { local_wins[j][i] += 1; }
                        }
                    }
                    (local_wins, local_borda)
                })

            .reduce(
                || (vec![vec![0; n]; n], vec![0; n]),
                |(mut tw, mut tb), (lw, lb)| {
                    for i in 0..n {
                        tb[i] += lb[i];
                        for j in 0..n { tw[i][j] += lw[i][j]; }
                    }
                    (tw, tb)
                });

        // Calcul des scores finaux de Copeland
        let mut copeland_scores = vec![0isize; n];
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                if wins[i][j] > wins[j][i] { copeland_scores[i] += 2; }
                else if wins[i][j] == wins[j][i] { copeland_scores[i] += 1; }
            }
        }

        // Classement final : d'abord par score de Copeland, puis par score de Borda, puis lexicographique
        let mut ranking: Vec<CandidateId> = (0..n).map(|i| i as u8).collect();
        ranking.sort_by(|&a, &b| {
            let au = a as usize;
            let bu = b as usize; 
            
            // Copeland Score
            let cmp = copeland_scores[bu].cmp(&copeland_scores[au]);
            if cmp != std::cmp::Ordering::Equal { return cmp; }
            
            // Borda Score (Tie-break)
            let cmp_borda = borda_scores[bu].cmp(&borda_scores[au]);
            if cmp_borda != std::cmp::Ordering::Equal { return cmp_borda; }

            // Lexicographique
            election.candidates[au].cmp(&election.candidates[bu])
        });

        VoteResult { ranking }
    }
}