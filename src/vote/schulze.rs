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


pub struct Schulze;

impl VotingAlgorithm for Schulze {

    fn name(&self) -> String {
        "Schulze".to_string()
    }

    fn compute(&self, election: &Election) -> VoteResult {

        let n = election.candidates.len();

        let wins = pairwise_matrix(election);
        let smith = smith_set(election);

        let mut p = vec![vec![0usize; n]; n];

        // initialisation
        for &i in &smith {
            for &j in &smith {

                if i == j { continue; }

                if wins[i][j] > wins[j][i] {
                    p[i][j] = wins[i][j] - wins[j][i];
                }
            }
        }

        // Floyd–Warshall version Schulze
        for &k in &smith {
            for &i in &smith {
                if i == k { continue; }

                for &j in &smith {
                    if j == i || j == k { continue; }

                    let val = std::cmp::min(p[i][k], p[k][j]);
                    if val > p[i][j] {
                        p[i][j] = val;
                    }
                }
            }
        }

        // score Schulze
        let mut scores = vec![0usize; n];

        for &i in &smith {
            for &j in &smith {

                if i == j { continue; }

                if p[i][j] > p[j][i] {
                    scores[i] += 1;
                }
            }
        }

        // classement
        let mut ranking: Vec<CandidateId> = (0..n).collect();

        ranking.sort_by_key(|&c|
            (
                std::cmp::Reverse(scores[c]),
                &election.candidates[c]
            )
        );

        VoteResult { ranking }
    }
}