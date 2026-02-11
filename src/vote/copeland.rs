use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};

pub struct Copeland;

impl VotingAlgorithm for Copeland {
    fn name(&self) -> String {
        return "Copeland".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult {
        //creer une liste de la taille de la liste des candidats pour les scores
        let mut scores = vec![0; election.candidates.len()];

        //comparer chaque paire de candidats (duels 1 contre 1)
        for i in 0..election.candidates.len() {
            for j in (i + 1)..election.candidates.len() {
                let mut i_wins = 0;
                let mut j_wins = 0;

                //regarder qui gagne le duel dans chaque bulletin
                for v in election.ballots.iter() {
                    let pos_i = v.ranking.iter().position(|&c| c == i).unwrap_or(usize::MAX);
                    let pos_j = v.ranking.iter().position(|&c| c == j).unwrap_or(usize::MAX);

                    if pos_i < pos_j {
                        i_wins += 1;
                    } else if pos_j < pos_i {
                        j_wins += 1;
                    }
                }

                //attribuer les points (+1 victoire, -1 defaite, 0 egalite)
                if i_wins > j_wins {
                    scores[i] += 1;
                    scores[j] -= 1;
                } else if j_wins > i_wins {
                    scores[j] += 1;
                    scores[i] -= 1;
                }
            }
        }

        //liste des candidats
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        
        //tri de la liste des candidats par score (du plus grand au plus petit)
        //pour garder l'ordre alphabétique en cas d'égalité, on utilise un tuple
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));

        return VoteResult {ranking};
    }
}