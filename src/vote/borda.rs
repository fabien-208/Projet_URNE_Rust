use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*; // 🟢 L'import magique pour le multi-threading

pub struct Borda;

impl Borda {
    pub fn get_score(election: &crate::types::Election) -> Vec<isize> {
        let num_candidates = election.candidates.len();

        // 🚀 par_iter() lance le calcul sur TOUS les cœurs de ton PC
        let scores = election.ballots.par_iter()
            .fold(
                // 1. Chaque cœur crée son propre petit tableau de scores (pour éviter de se marcher dessus)
                || vec![0isize; num_candidates], 
                
                // 2. Chaque cœur lit sa part des 40 millions de bulletins
                |mut thread_scores, v| {
                    for (pos, &c) in v.ranking.iter().enumerate() {
                        thread_scores[c] += (num_candidates - pos - 1) as isize;
                    }
                    thread_scores
                }
            )
            .reduce(
                // 3. Quand les cœurs ont fini, on prépare le tableau final
                || vec![0isize; num_candidates], 
                
                // 4. On additionne les tableaux de tous les cœurs ensemble
                |mut total_scores, thread_scores| {
                    for i in 0..num_candidates {
                        total_scores[i] += thread_scores[i];
                    }
                    total_scores
                }
            );

        scores
    }
}

impl VotingAlgorithm for Borda {
    fn name(&self) -> String { "Borda".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let scores = Borda::get_score(election);
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));
        VoteResult { ranking }
    }
}