use crate::{VotingAlgorithm, types::VoteResult};
use rayon::prelude::*;

pub struct Baldwin;

impl VotingAlgorithm for Baldwin {
    fn name(&self) -> String {
        "Baldwin (Borda éliminatoire)".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();

        // Calcul de la Matrice des Duels
        // wins[i][j] = nombre de personnes qui préfèrent i à j
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0usize; n]; n], // Matrice locale par cœur
                |mut local_wins, v| {
                    let mut pos = vec![usize::MAX; n];
                    
                    // On note les positions de chaque candidat dans ce bulletin
                    for (idx, &c) in v.ranking.iter().enumerate() {
                        pos[c] = idx;
                    }

                    // On remplit la matrice : i bat j si i est mieux classé que j
                    for i in 0..n {
                        for j in (i + 1)..n {
                            // Si i est classé avant j
                            if pos[i] < pos[j] {
                                local_wins[i][j] += 1;
                            } 
                            // Si j est classé avant i
                            else if pos[j] < pos[i] {
                                local_wins[j][i] += 1;
                            }
                        }
                    }
                    local_wins
                }
            )
            .reduce(
                || vec![vec![0usize; n]; n],
                |mut total, local| {
                    for i in 0..n {
                        for j in 0..n {
                            total[i][j] += local[i][j];
                        }
                    }
                    total
                }
            );

        // La boucle d'élimination (Instantanée sur la matrice)
        
        let mut active = vec![true; n]; 
        let mut final_ranking = Vec::with_capacity(n); // Pour stocker l'ordre de sortie
        
        // On boucle tant qu'il reste des candidats
        for _ in 0..n {
            let mut min_borda_score = usize::MAX;
            let mut candidate_to_eliminate = None;

            // Calcul du score Borda "Virtuel" basé sur la matrice
            for i in 0..n {
                if !active[i] { continue; }

                let mut score = 0;
                for j in 0..n {
                    // On ne compte les points que contre les candidats ENCORE ACTIFS
                    if i != j && active[j] {
                        score += wins[i][j];
                    }
                }

                // On cherche celui qui a le score le plus faible
                // En cas d'égalité, on prend le plus grand index (arbitraire) pour départager
                if score < min_borda_score {
                    min_borda_score = score;
                    candidate_to_eliminate = Some(i);
                } else if score == min_borda_score {
                     // Petite astuce stable pour les égalités
                     candidate_to_eliminate = Some(i);
                }
            }

            // On élimine le perdant
            if let Some(loser) = candidate_to_eliminate {
                active[loser] = false;
                final_ranking.push(loser);
            } else {
                break; // Ne devrait pas arriver
            }
        }

        // Comme on a ajouté les éliminés du premier au dernier, 
        // le gagnant est à la fin. On inverse pour avoir le classement 1er -> Dernier.
        final_ranking.reverse();

        VoteResult { ranking: final_ranking }
    }
}