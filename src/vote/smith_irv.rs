use crate::{types::{CandidateId, Election, VoteResult}, vote::VotingAlgorithm};
use rayon::prelude::*;

pub fn pairwise_matrix(election: &Election) -> Vec<Vec<usize>> {
    let n = election.candidates.len();

    election.ballots.par_iter()
        .fold(
            || vec![vec![0usize; n]; n],
            |mut local, ballot| {
                let mut pos = vec![usize::MAX; n];

                for (i, &c) in ballot.ranking.iter().enumerate() {
                    pos[c] = i;
                }

                for i in 0..n {
                    for j in (i + 1)..n {
                        if pos[i] < pos[j] {
                            local[i][j] += 1;
                        } else if pos[j] < pos[i] {
                            local[j][i] += 1;
                        }
                    }
                }

                local
            }
        )
        .reduce(
            || vec![vec![0usize; n]; n],
            |mut acc, local| {
                for i in 0..n {
                    for j in 0..n {
                        acc[i][j] += local[i][j];
                    }
                }
                acc
            }
        )
}

pub fn smith_set(election: &Election) -> Vec<CandidateId> {
    let n = election.candidates.len();
    let wins = pairwise_matrix(election);

    // Copeland score parallèle
    let scores: Vec<isize> = (0..n).into_par_iter()
        .map(|i| {
            let mut s = 0;
            for j in 0..n {
                if i == j { continue; }
                if wins[i][j] > wins[j][i] { s += 1; }
                else if wins[j][i] > wins[i][j] { s -= 1; }
            }
            s
        })
        .collect();

    let max_score = *scores.par_iter().max().unwrap();

    let mut smith: Vec<bool> =
        scores.par_iter()
            .map(|&s| s == max_score)
            .collect();

    loop {
        let current = smith.clone();

        let new_members: Vec<usize> =
            (0..n).into_par_iter()
                .filter(|&i| {
                    if current[i] { return false; }
                    for j in 0..n {
                        if current[j] && wins[i][j] > wins[j][i] {
                            return true;
                        }
                    }
                    false
                })
                .collect();

        if new_members.is_empty() {
            break;
        }

        for i in new_members {
            smith[i] = true;
        }
    }

    (0..n).filter(|&i| smith[i]).collect()
}

pub fn condorcet_winner(
    wins: &Vec<Vec<usize>>,
    candidates: &[CandidateId],
) -> Option<CandidateId> {

    candidates.par_iter()
        .cloned()
        .find_any(|&i| {
            for &j in candidates {
                if i == j { continue; }
                if wins[i][j] <= wins[j][i] {
                    return false;
                }
            }
            true
        })
}


pub struct SmithIRV;

impl VotingAlgorithm for SmithIRV {

    fn name(&self) -> String {
        "Smith+IRV".to_string()
    }

    fn compute(&self, election: &Election) -> VoteResult {

        let mut smith = smith_set(election);
        let wins = pairwise_matrix(election);
        let n = election.candidates.len();

        loop {

            if let Some(w) = condorcet_winner(&wins, &smith) {
                return VoteResult { ranking: vec![w] };
            }

            let scores = election.ballots.par_iter()
                .fold(
                    || vec![0usize; n],
                    |mut local, ballot| {
                        for &c in &ballot.ranking {
                            if smith.contains(&c) {
                                local[c] += 1;
                                break;
                            }
                        }
                        local
                    }
                )
                .reduce(
                    || vec![0usize; n],
                    |mut acc, local| {
                        for i in 0..n {
                            acc[i] += local[i];
                        }
                        acc
                    }
                );

            let loser = smith.par_iter()
                .min_by_key(|&&c| (scores[c], &election.candidates[c]))
                .cloned()
                .unwrap();

            smith.retain(|&c| c != loser);
        }
    }
}