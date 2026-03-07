use crate::{types::{Election, VoteResult}, vote::VotingAlgorithm};
use rayon::prelude::*;

pub struct IRV;

impl VotingAlgorithm for IRV {
    fn name(&self) -> String { "IRV".to_string() }

    fn compute(&self, election: &Election) -> VoteResult {
        let nb_candidats = election.candidates.len();
        // true = en course / false = éliminé
        let mut active = vec![true; nb_candidats];
        //on va empiler les candidats éliminés dans l'ordre d'élimination, le gagnant sera le dernier
        let mut elimination_order = Vec::with_capacity(nb_candidats);

        //Boucle d'élimination
        for _ in 0..(nb_candidats - 1) { //on s arrete quand il ne reste plus qu'un candidat
            let scores = election.ballots.par_iter()
                .fold(
                    || vec![0usize; nb_candidats],
                    |mut local_wins, ballot| {
                        // on cherche le premier choix valide du bulletin
                        for &c in &ballot.ranking {
                            if active[c as usize] {
                                local_wins[c as usize] += 1;
                                break; 
                            }
                        }
                        local_wins
                    })
                .reduce(
                    || vec![0; nb_candidats],
                    |mut total_wins, local_wins| {
                        for i in 0..nb_candidats {
                            total_wins[i] += local_wins[i];
                        }
                        total_wins
                    });

            //trouver le candidat avec le moins de votes parmi les actifs
            let mut min_score = usize::MAX;
            let mut candidate_to_eliminate = None;

            for c in 0..nb_candidats {
                if active[c] {
                    if scores[c] < min_score {
                        min_score = scores[c];
                        candidate_to_eliminate = Some(c);
                    } else if scores[c] == min_score {
                        // Egalité : on prend arbitrairement le candidat avec le plus grand ID
                        candidate_to_eliminate = Some(c);
                    }
                }
            }

            if let Some(loser) = candidate_to_eliminate {
                active[loser] = false;
                elimination_order.push(loser as u8);
            } else {
                break;// ne devrait pas arriver
            }
        }

        // ajoute le vainqueur à la fin
        for c in 0..nb_candidats {
            if active[c] {
                elimination_order.push(c as u8);
                break;
            }
        }
        // le gagnant est le dernier ajouté donc on inverse l'ordre pour que le gagnant soit en premier
        elimination_order.reverse();
        VoteResult { ranking: elimination_order }
    }
}