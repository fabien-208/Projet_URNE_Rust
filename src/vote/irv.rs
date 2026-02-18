use crate::{types::{CandidateId, Election, VoteResult}, vote::VotingAlgorithm};

pub struct IRV;

// note : j'ai fait un peu a ma sauce pour conserver les scores des eliminés pour 
// établir un classement final complet, pas sur à 100% que ça fonctionne (a tester)

impl VotingAlgorithm for IRV {
    fn name(&self) -> String {
        return "IRV".to_string();
    }

    fn compute(&self, election: &Election) -> VoteResult {
        let nb_candidats = election.candidates.len();
        //liste de la taille du nombre de candidats
        let mut scores = vec![0; nb_candidats];
        // liste de candidats toujours en liste
        let mut en_course: Vec<bool> = vec![true; nb_candidats];
        let mut restants = nb_candidats;
        // boucle 
        loop {
            //reset des scores des survivants (pour le classement final)
            for (score, valide) in scores.iter_mut().zip(en_course.iter_mut()) {
                if *valide {*score = 0;}
            }

            // votes
            for ballot in &election.ballots {
                // parcourir les candidats jusqu'a en trouver un valable
                for &candidate in &ballot.ranking {
                    if en_course[candidate] {
                        scores[candidate] += 1;
                        break;
                    }
                }
            }

            //eliminer le pire
            let perdant = (0..nb_candidats)
                .filter(|&i| en_course[i])
                .min_by_key(|&i| scores[i])
                .unwrap();

            en_course[perdant] = false;
            restants -= 1;

            // tous ont été éliminé sauf le dernier, on a fini le vote
            if restants == 1 {
                //trier et retourner la liste
                let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
                //tri de la liste des candidats
                ranking.sort_by_key(|&i| std::cmp::Reverse(scores[i]));

                return VoteResult {ranking}; 
            }
            
        }

    }
}