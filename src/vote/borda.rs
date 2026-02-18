use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};

pub struct Borda;

impl Borda {
    // Fonction publique pour récupérer les scores (utilisée par Copeland-Borda)
    pub fn get_score(election: &crate::types::Election) -> Vec<isize> {
        // creer une liste de la taille de la liste des candidats pour les scores
        let mut scores = vec![0; election.candidates.len()];

        // pour chaque bulletin, attribuer des points aux candidats en fonction de leur position
        for v in election.ballots.iter() {
            for (pos, &c) in v.ranking.iter().enumerate() {
                // le candidat en position pos reçoit (nombre de candidats - pos - 1) points
                // (On convertit en isize pour être compatible avec les scores négatifs potentiels d'autres algos)
                scores[c] += (election.candidates.len() - pos - 1) as isize;
            }
        }
        scores
    }
}

impl VotingAlgorithm for Borda {
    fn name(&self) -> String {
        return "Borda".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult {
        // 1. Récupération des scores via la fonction partagée
        let scores = Borda::get_score(election);

        // liste des candidats (indices)
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        
        // tri de la liste des candidats par score (du plus grand au plus petit)
        // pour garder l'ordre alphabétique en cas d'égalité, on utilise un tuple
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));

        return VoteResult {ranking};
    }
}