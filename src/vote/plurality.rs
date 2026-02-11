use crate::{VotingAlgorithm, types::{CandidateId, VoteResult}};

pub struct Plurality;

impl VotingAlgorithm for Plurality {
    fn name(&self) -> String {
        return "Plurality".to_string();
    }

    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult {
        //creer une liste de la taille de la liste des candidats
        let mut scores = vec![0; election.candidates.len()];
        //prendre les premiers éléments de chaque liste, et regarder a quel candidat il correspondent
        for v in election.ballots.iter() {
            scores[v.ranking[0]]+=1;
        }
        //liste des candidats
        let mut ranking: Vec<CandidateId> = (0..scores.len()).collect();
        //tri de la liste des candidats
        ranking.sort_by_key(|&i| std::cmp::Reverse(scores[i]));

        return VoteResult {ranking};
    }
}