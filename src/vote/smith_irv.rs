use crate::{types::{CandidateId, Election, VoteResult}, vote::VotingAlgorithm};
use rayon::prelude::*;

pub fn pairwise_matrix(election: &Election) -> Vec<Vec<usize>> {
    let n = election.candidates.len();
    election.ballots.par_iter()
        .fold(
            || vec![vec![0usize; n]; n],
            |mut local, ballot| {
                let mut pos = vec![usize::MAX; n];
                for (i, &c) in ballot.ranking.iter().enumerate() { pos[c as usize] = i; }
                for i in 0..n {
                    for j in (i + 1)..n {
                        if pos[i] < pos[j] { local[i][j] += 1; } 
                        else if pos[j] < pos[i] { local[j][i] += 1; }
                    }
                }
                local
            }
        )
        .reduce(|| vec![vec![0usize; n]; n], |mut acc, local| {
            for i in 0..n { for j in 0..n { acc[i][j] += local[i][j]; } }
            acc
        })
}

pub fn smith_set(election: &Election) -> Vec<CandidateId> {
    let n = election.candidates.len();
    let wins = pairwise_matrix(election);

    let mut copeland = vec![0isize; n];
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if wins[i][j] > wins[j][i] { copeland[i] += 1; }
            else if wins[j][i] > wins[i][j] { copeland[i] -= 1; }
        }
    }

    let max_score = (0..n).map(|i| copeland[i]).max().unwrap();
    let mut smith: Vec<bool> = (0..n).map(|i| copeland[i] == max_score).collect();

    // 🎯 FIX : Utilisation de `>=` pour inclure les égalités parfaites
    loop {
        let mut added = false;
        for i in 0..n {
            if smith[i] { continue; }
            for j in 0..n {
                if smith[j] && wins[i][j] >= wins[j][i] {
                    smith[i] = true;
                    added = true;
                    break;
                }
            }
        }
        if !added { break; }
    }
    (0..n).filter(|&i| smith[i]).map(|i| i as u8).collect()
}

pub fn condorcet_winner(wins: &Vec<Vec<usize>>, candidates: &[CandidateId]) -> Option<CandidateId> {
    candidates.par_iter().cloned().find_any(|&i| {
        for &j in candidates {
            if i == j { continue; }
            if wins[i as usize][j as usize] <= wins[j as usize][i as usize] { return false; }
        }
        true
    })
}

pub struct SmithIRV;

impl VotingAlgorithm for SmithIRV {
    fn name(&self) -> String { "Smith+IRV".to_string() }

    fn compute(&self, election: &Election) -> VoteResult {
        let n = election.candidates.len();
        let wins = pairwise_matrix(election);
        let mut smith = smith_set(election);
        let mut eliminated = Vec::new();

        loop {
            // 🎯 FIX : On vérifie si un gagnant de Condorcet émerge après chaque élimination
            if let Some(w) = condorcet_winner(&wins, &smith) {
                let mut ranking = vec![w];
                eliminated.reverse();
                ranking.extend(eliminated);
                for i in 0..n {
                    let id = i as u8;
                    if !ranking.contains(&id) { ranking.push(id); }
                }
                return VoteResult { ranking };
            }

            let scores = election.ballots.par_iter()
                .fold(|| vec![0usize; n], |mut local, ballot| {
                    for &c in &ballot.ranking {
                        if smith.contains(&c) {
                            local[c as usize] += 1;
                            break;
                        }
                    }
                    local
                })
                .reduce(|| vec![0usize; n], |mut acc, local| {
                    for i in 0..n { acc[i] += local[i]; }
                    acc
                });

            // 🎯 FIX : Élimination du plus loin dans l'alphabet en cas d'égalité (Reverse)
            let loser = smith.iter()
                .max_by_key(|&&c| (std::cmp::Reverse(scores[c as usize]), &election.candidates[c as usize]))
                .cloned()
                .unwrap();

            smith.retain(|&c| c != loser);
            eliminated.push(loser);

            if smith.len() == 1 {
                let mut ranking = vec![smith[0]];
                eliminated.reverse();
                ranking.extend(eliminated);
                for i in 0..n {
                    let id = i as u8;
                    if !ranking.contains(&id) { ranking.push(id); }
                }
                return VoteResult { ranking };
            }
        }
    }
}