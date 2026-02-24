use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};

pub struct Copeland;

impl Copeland {
    pub fn get_score(election: &crate::types::Election) -> Vec<isize> {
        let n = election.candidates.len();
        let mut wins = vec![vec![0; n]; n];

        // 🚀 OPTIMISATION ULTIME : On alloue la mémoire UNE SEULE FOIS ici !
        let mut pos = vec![usize::MAX; n];

        for v in election.ballots.iter() {
            // On remet juste le tableau à zéro sans redemander de la RAM à l'OS
            pos.fill(usize::MAX);
            
            for (idx, &c) in v.ranking.iter().enumerate() {
                pos[c] = idx;
            }

            // On met à jour la matrice des duels
            for i in 0..n {
                for j in (i + 1)..n {
                    if pos[i] < pos[j] {
                        wins[i][j] += 1;
                    } else if pos[j] < pos[i] {
                        wins[j][i] += 1;
                    }
                }
            }
        }

        let mut scores = vec![0; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if wins[i][j] > wins[j][i] {
                    scores[i] += 1;
                    scores[j] -= 1;
                } else if wins[j][i] > wins[i][j] {
                    scores[j] += 1;
                    scores[i] -= 1;
                }
            }
        }

        scores
    }
}

impl VotingAlgorithm for Copeland {
    fn name(&self) -> String { "Copeland".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let scores = Copeland::get_score(election);
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        ranking.sort_by_key(|&i| (std::cmp::Reverse(scores[i]), &election.candidates[i]));
        VoteResult { ranking }
    }
}