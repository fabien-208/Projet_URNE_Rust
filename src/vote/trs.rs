use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;
use rayon::prelude::*;

/// Implémentation du Vote à Deux Tours (Two-Round System - TRS).
/// Si personne n'a la majorité absolue au premier tour, un second tour est organisé
/// entre les deux candidats de tête.
pub struct Trs;

impl VotingAlgorithm for Trs {
    fn name(&self) -> String { "TRS".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();

        // --- TOUR 1 ---
        // Comptage des voix en première position
        let round1_scores = election.ballots.par_iter()
            .map(|ballot| {
                let mut local = vec![0usize; num_candidates];
                if let Some(&first) = ballot.ranking.first() {
                    local[first as usize] = 1;
                }
                local
            })
            .reduce(|| vec![0; num_candidates], |mut t, l| {
                for i in 0..num_candidates { t[i] += l[i]; }
                t
            });

        let mut round1_ranking: Vec<CandidateId> = (0..num_candidates).map(|i| i as CandidateId).collect();
        round1_ranking.sort_by_key(|&c| (Reverse(round1_scores[c as usize]), &election.candidates[c as usize]));

        // Vérification de la majorité absolue (score > 50%)
        if round1_scores[round1_ranking[0] as usize] * 2 > election.ballots.len() {
            return VoteResult { ranking: round1_ranking };
        }

        // --- TOUR 2 ---
        // Sélection des deux finalistes
        let c1 = round1_ranking[0];
        let c2 = round1_ranking[1];
        
        // Face-à-face entre les deux candidats retenus
        let (score1, score2) = election.ballots.par_iter()
            .map(|ballot| {
                let mut s1 = 0;
                let mut s2 = 0;
                for &c in &ballot.ranking {
                    if c == c1 { s1 = 1; break; }
                    if c == c2 { s2 = 1; break; }
                }
                (s1, s2)
            })
            .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));

        // Détermination du vainqueur avec départage lexicographique
        let winner = if score1 > score2 { c1 }
        else if score2 > score1 { c2 }
        else if election.candidates[c1 as usize] < election.candidates[c2 as usize] { c1 }
        else { c2 };
        
        let loser = if winner == c1 { c2 } else { c1 };
        
        // Construction du classement final (Vainqueur, puis perdant du T2, puis les éliminés du T1)
        let mut final_ranking = vec![winner, loser];
        for &c in round1_ranking.iter().skip(2) {
            final_ranking.push(c);
        }

        VoteResult { ranking: final_ranking }
    }
}