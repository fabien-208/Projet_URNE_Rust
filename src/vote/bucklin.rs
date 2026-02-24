use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};
use std::cmp::Reverse;

pub struct Bucklin;

impl VotingAlgorithm for Bucklin {
    fn name(&self) -> String {
        "Bucklin".to_string()
    }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let num_candidates = election.candidates.len();
        let total_votes = election.ballots.len();
        let majority_threshold = total_votes / 2; // Majorité absolue (> 50%)

        // rank_counts[rang][candidat] = nombre de fois où le candidat a été mis à ce rang
        let mut rank_counts = vec![vec![0; num_candidates]; num_candidates];

        for ballot in &election.ballots {
            for (rank, &candidate) in ballot.ranking.iter().enumerate() {
                // On s'assure de ne pas déborder si un vote a trop de candidats
                if rank < num_candidates {
                    rank_counts[rank][candidate] += 1;
                }
            }
        }

        // Tableau pour stocker les scores cumulés
        let mut scores = vec![0; num_candidates];

        // On déroule l'algorithme : round par round (rang 0, puis rang 1, etc.)
        for rank in 0..num_candidates {
            let mut majority_reached = false;

            // On ajoute les votes de ce rang aux scores actuels des candidats
            for c in 0..num_candidates {
                scores[c] += rank_counts[rank][c];
                
                // Si un candidat dépasse les 50%, on lève le drapeau
                if scores[c] > majority_threshold {
                    majority_reached = true;
                }
            }

            // Dès qu'un ou plusieurs candidats ont atteint la majorité absolue,
            // la méthode de Bucklin dit qu'on arrête de compter !
            if majority_reached {
                break;
            }
        }

        // --- Création du classement final ---
        let mut ranking: Vec<CandidateId> = (0..num_candidates).collect();
        
        // On trie par le score cumulé au round où la boucle s'est arrêtée.
        // S'il y a plusieurs gagnants qui ont dépassé la majorité au même round,
        // celui avec le score le plus élevé l'emporte.
        ranking.sort_by_key(|&c| (
            Reverse(scores[c]),      // Le plus grand score d'abord
            &election.candidates[c]  // Ordre alphabétique en cas d'égalité absolue
        ));

        VoteResult { ranking }
    }
}