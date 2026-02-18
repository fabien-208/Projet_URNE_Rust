use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;

// On importe les modules voisins pour utiliser leurs fonctions get_score
use crate::vote::borda::Borda;
use crate::vote::copeland::Copeland;

pub struct CopelandBorda;

impl VotingAlgorithm for CopelandBorda {
    fn name(&self) -> String {
        return "Copeland-Borda".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult {
        // 1. Calculer les scores de Copeland (Victoires en duels)
        let copeland_scores = Copeland::get_score(election);

        // 2. Calculer les scores de Borda (Points de position)
        let borda_scores = Borda::get_score(election);

        // liste des candidats
        let mut ranking: Vec<CandidateId> = (0..election.candidates.len()).collect();
        
        // Tri combiné (Tie-breaking) :
        // - Critère 1 : Score Copeland (Décroissant)
        // - Critère 2 : Score Borda (Décroissant) -> Utiliser pour départager les égalités Copeland
        // - Critère 3 : Nom du candidat (Croissant) -> Pour la stabilité alphabétique
        ranking.sort_by_key(|&i| (
            Reverse(copeland_scores[i]), 
            Reverse(borda_scores[i]), 
            &election.candidates[i]
        ));

        return VoteResult {ranking};
    }
}