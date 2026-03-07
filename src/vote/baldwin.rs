use crate::{VotingAlgorithm, types::VoteResult};
use rayon::prelude::*;

pub struct Baldwin;

impl VotingAlgorithm for Baldwin {
    fn name(&self) -> String {
        "Baldwin".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();
        // calculde la Matrix des Duels
        // wins[i][j] = nombre de votes préférant i à j
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0usize; n]; n], //Matrice local pour chaque thread
                |mut local_wins, v| {
                    let mut pos = vec![usize::MAX; n];


                    //On note les positions de chaque candidat dans ce Bulletin
                    for (idx, &c) in v.ranking.iter().enumerate() {
                        pos[c as usize] = idx;
                    }
                    for i in 0..n {
                        for j in (i + 1)..n {
                            // si i est classé avant j
                            if pos[i] < pos[j] { 
                                local_wins[i][j] += 1;
                            } 
                            // si j est classé avant i
                            else if pos[j] < pos[i] {
                                local_wins[j][i] += 1;
                            }
                        }
                    }
                local_wins
            }
        )
            .reduce(|| vec![vec![0; n]; n], |mut t, l| {
                for i in 0..n { for j in 0..n { t[i][j] += l[i][j]; } }
                t
            });


        let mut active = vec![true; n];
        let mut final_ranking = Vec::with_capacity(n);
        
        //on boucle tant qu'il reste des candidats actifs
        for _ in 0..n {
            let mut min_score = isize::MAX;
            let mut candidate_to_eliminate = None;

            //calcul du score de Bordas basé sur la matrice
            for i in 0..n {
                if !active[i] {
                    continue;
                }
                let mut score = 0;
                for j in 0..n {
                    // on ne compte que les points contre les autres candidats actifs
                    if i != j && active[j] {
                        score += wins[i][j] as isize;
                    }
                }
                //on cherche le candidat avec le score le plus bas
                // en cas d'égalité, on élimine le candidat avec l'indice le plus élevé
                if score < min_score {
                    min_score = score;
                    candidate_to_eliminate = Some(i);
                } else if score == min_score {
                    candidate_to_eliminate = Some(i);
                }
            }

            //on élimine le candidat avec le score le plus bas
            if let Some(loser) = candidate_to_eliminate {
                active[loser] = false;
                final_ranking.push(loser as u8);
            } else { break; }
        }

        final_ranking.reverse();
        VoteResult { ranking: final_ranking }
    }
}