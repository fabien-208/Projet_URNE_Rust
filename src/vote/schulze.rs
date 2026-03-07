use crate::{types::{CandidateId, VoteResult}, vote::VotingAlgorithm};
use rayon::prelude::*;

pub struct Schulze;

impl VotingAlgorithm for Schulze {
    fn name(&self) -> String { "Schulze".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();

        //calcul de la matrice des duels (wins[i][j] = nombre de votes préférant i à j)
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0usize; n]; n],
                |mut local_wins, ballot| {
                    let mut pos = vec![usize::MAX; n];
                    for (i, &c) in ballot.ranking.iter().enumerate() { pos[c as usize] = i; }
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if pos[i] < pos[j] { local_wins[i][j] += 1; }
                            else if pos[j] < pos[i] { local_wins[j][i] += 1; }
                        }
                    }
                    local_wins
                })
            .reduce(
                || vec![vec![0; n]; n],
                |mut total_wins, local_wins| {
                    for i in 0..n { for j in 0..n { total_wins[i][j] += local_wins[i][j]; } }
                    total_wins
                });

        //initialisation des forces de chemin
        let mut p = vec![vec![0usize; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    if wins[i][j] > wins[j][i] {
                        p[i][j] = wins[i][j] - wins[j][i]; // marge de victoire
                    }
                }
            }
        }

        //calcul des forces de chemin les plus fortes
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if i != j && i != k && j != k {
                        let strength = std::cmp::min(p[i][k], p[k][j]);
                        if strength > p[i][j] { p[i][j] = strength; }
                    }
                }
            }
        }

        //calcul du classement final
        //dans Schulze, A est meilleur que B si p[A][B] > p[B][A]   
        let mut ranking: Vec<CandidateId> = (0..n).map(|i| i as u8).collect();
        ranking.sort_by(|&a, &b| {
            let au = a as usize;
            let bu = b as usize;
            if p[au][bu] > p[bu][au] { std::cmp::Ordering::Less } 
            else if p[bu][au] > p[au][bu] { std::cmp::Ordering::Greater }
            else { election.candidates[au].cmp(&election.candidates[bu]) }
        });

        VoteResult { ranking }
    }
}