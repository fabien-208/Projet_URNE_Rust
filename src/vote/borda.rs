use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use rayon::prelude::*;

pub struct Borda;

/// Implémente l'algorithme de vote de Borda.
/// 
/// Le vote de Borda est un système de vote où chaque candidat reçoit des points
/// en fonction de sa position dans le classement de chaque électeur.
/// 
/// # Algorithme
/// 
/// Pour chaque scrutin :
/// - Le candidat en première position reçoit (n-1) points, où n est le nombre de candidats
/// - Le candidat en deuxième position reçoit (n-2) points
/// - Et ainsi de suite jusqu'au dernier candidat qui reçoit 0 points
/// 
/// Les points de tous les scrutins sont additionnés pour chaque candidat.
/// Les candidats sont ensuite classés par ordre décroissant de points.
/// En cas d'égalité, l'ordre alphabétique des noms de candidats est utilisé comme critère de départage.
/// 
/// # Complexité
/// 
/// - Temps : O(m*n) où m est le nombre de scrutins et n le nombre de candidats
/// - Espace : O(n)
/// 
/// # Paramètres
/// 
/// * `election` - L'élection contenant les candidats et les scrutins à analyser
/// 
/// # Retour
/// 
/// Un `VoteResult` contenant le classement final des candidats du meilleur au moins bon.
impl VotingAlgorithm for Borda {
    fn name(&self) -> String { "Borda".to_string() }


    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();

        let scores = election.ballots.par_iter()
            .fold(|| vec![0isize; num_candidates], |mut thread_scores, ballot| {
                let m = ballot.ranking.len(); // 🎯 FIX : 'm' est la taille de CE bulletin
                for (pos, &c) in ballot.ranking.iter().enumerate() {
                    thread_scores[c as usize] += (m - pos) as isize;
                }
                thread_scores
            })
            .reduce(|| vec![0isize; num_candidates], |mut a, b| {
                for i in 0..num_candidates { a[i] += b[i]; }
                a
            });

        let mut ranking: Vec<CandidateId> = (0..scores.len()).map(|i| i as u8).collect();
        
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i as usize]), &election.candidates[i as usize]));

        VoteResult { ranking }
    }
}