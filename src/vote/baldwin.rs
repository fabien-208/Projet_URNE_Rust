use crate::{VotingAlgorithm, types::VoteResult};
use rayon::prelude::*;

pub struct Baldwin;

impl VotingAlgorithm for Baldwin {
    fn name(&self) -> String { "Baldwin".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();
        let mut active = vec![true; n];
        let mut final_ranking = Vec::with_capacity(n);
        
        for _ in 0..n {
            // 🎯 FIX : Calcul du VRAI score de Borda en fonction des survivants (m)
            let scores = election.ballots.par_iter()
                .fold(
                    || vec![0isize; n],
                    |mut local, v| {
                        let mut m = 0;
                        for &c in &v.ranking { if active[c as usize] { m += 1; } }
                        
                        let mut pts = m;
                        for &c in &v.ranking {
                            if active[c as usize] {
                                local[c as usize] += pts as isize;
                                pts -= 1;
                            }
                        }
                        local
                    }
                )
                .reduce(|| vec![0isize; n], |mut t, l| {
                    for i in 0..n { t[i] += l[i]; }
                    t
                });

            let mut min_score = isize::MAX;
            let mut candidate_to_eliminate = None;

            for i in 0..n {
                if !active[i] { continue; }
                let score = scores[i];
                if score < min_score {
                    min_score = score;
                    candidate_to_eliminate = Some(i);
                } else if score == min_score {
                    // 🎯 FIX : Élimination de celui le plus loin dans l'alphabet (ex: D au lieu de B)
                    if let Some(curr) = candidate_to_eliminate {
                        if election.candidates[i] > election.candidates[curr] {
                            candidate_to_eliminate = Some(i);
                        }
                    } else {
                        candidate_to_eliminate = Some(i);
                    }
                }
            }

            if let Some(loser) = candidate_to_eliminate {
                active[loser] = false;
                final_ranking.push(loser as u8);
            } else { break; }
        }

        final_ranking.reverse();
        VoteResult { ranking: final_ranking }
    }
}