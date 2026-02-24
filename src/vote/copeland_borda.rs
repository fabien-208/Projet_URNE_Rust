use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;

pub struct CopelandBorda;

impl VotingAlgorithm for CopelandBorda {
    fn name(&self) -> String {
        return "Copeland-Borda".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();
        
        let mut wins = vec![vec![0; n]; n];
        let mut borda_scores = vec![0isize; n]; // On prépare Borda
        let mut pos = vec![usize::MAX; n];

        // 🚀 L'ASTUCE EST ICI : UNE SEULE LECTURE POUR LES DEUX ALGOS
        for v in &election.ballots {
            pos.fill(usize::MAX);
            
            for (idx, &c) in v.ranking.iter().enumerate() {
                pos[c] = idx;
                
                // On calcule Borda au passage, ça prend 0.0001 milliseconde de plus
                borda_scores[c] += (n - idx - 1) as isize;
            }

            // On calcule les duels Copeland au même moment
            for i in 0..n {
                for j in (i + 1)..n {
                    if pos[i] < pos[j] {
                        wins[i][j] += 1;
                    } else if pos[j] < pos[i] {
                        wins[j][i] += 1;
                    }
                }
            }
        }

        // On finalise les scores de Copeland à partir de la matrice
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

        // --- Classement Final ---
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