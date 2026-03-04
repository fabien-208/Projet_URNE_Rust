use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

pub struct Trs;

impl VotingAlgorithm for Trs {
    fn name(&self) -> String {
        "Two-Round System (2 Tours)".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();

        // TOUR 1 : On compte les premiers choix (comme Plurality)
        let round1_scores = election.ballots.par_iter()
            .fold(
                || vec![0usize; num_candidates],
                |mut local, ballot| {
                    if let Some(&first) = ballot.ranking.first() {
                        local[first] += 1;
                    }
                    local
                }
            )
            .reduce(
                || vec![0usize; num_candidates],
                |mut total, local| {
                    for i in 0..num_candidates { total[i] += local[i]; }
                    total
                }
            );

        let mut round1_ranking: Vec<CandidateId> = (0..num_candidates).collect();
        round1_ranking.sort_by_key(|&c| (Reverse(round1_scores[c]), &election.candidates[c]));

        // Si le 1er a la majorité absolue (> 50%), il gagne direct (pas de Tour 2)
        if round1_scores[round1_ranking[0]] * 2 > election.ballots.len() {
            return VoteResult { ranking: round1_ranking };
        }

        // Duel face-à-face entre les deux premiers du Tour 1
        let finalist1 = round1_ranking[0];
        let finalist2 = round1_ranking[1];

        // On refait un passage parallèle rapide pour le face-à-face
        let (score_f1, score_f2) = election.ballots.par_iter()
            .fold(
                || (0usize, 0usize), // (Score Finaliste 1, Score Finaliste 2)
                |mut local_scores, ballot| {
                    // On parcourt le bulletin de gauche à droite
                    for &c in &ballot.ranking {
                        if c == finalist1 {
                            local_scores.0 += 1;
                            break; // Dès qu'on voit l'un des deux, c'est lui qui prend la voix
                        } else if c == finalist2 {
                            local_scores.1 += 1;
                            break;
                        }
                    }
                    local_scores
                }
            )
            .reduce(
                || (0, 0),
                |total, local| (total.0 + local.0, total.1 + local.1)
            );

        // Classement Final
        let mut final_ranking = round1_ranking.clone();
        // Si le finaliste 2 bat le finaliste 1, on inverse leurs places dans le classement
        if score_f2 > score_f1 {
            final_ranking[0] = finalist2;
            final_ranking[1] = finalist1;
        }

        VoteResult { ranking: final_ranking }
    }
}