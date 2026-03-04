use crate::{types::{CandidateId, Election, VoteResult}, vote::VotingAlgorithm};

pub fn pairwise_matrix(election: &Election) -> Vec<Vec<usize>> {
    let n = election.candidates.len();
    let mut wins = vec![vec![0; n]; n];

    for ballot in &election.ballots {
        let mut pos = vec![usize::MAX; n];

        for (i, &c) in ballot.ranking.iter().enumerate() {
            pos[c] = i;
        }

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

    wins
}

pub fn smith_set(election: &Election) -> Vec<CandidateId> {
    let n = election.candidates.len();
    let wins = pairwise_matrix(election);

    // score Copeland
    let mut scores = vec![0isize; n];

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

    let max_score = *scores.iter().max().unwrap();

    let mut smith: Vec<bool> = scores.iter()
        .map(|&s| s == max_score)
        .collect();

    let mut changed = true;

    while changed {
        changed = false;

        for i in 0..n {
            if smith[i] { continue; }

            for j in 0..n {
                if smith[j] && wins[i][j] > wins[j][i] {
                    smith[i] = true;
                    changed = true;
                    break;
                }
            }
        }
    }

    (0..n).filter(|&i| smith[i]).collect()
}

pub fn condorcet_winner(
    wins: &Vec<Vec<usize>>,
    candidates: &Vec<CandidateId>
) -> Option<CandidateId> {

    for &i in candidates {
        let mut beats_all = true;

        for &j in candidates {
            if i == j { continue; }
            if wins[i][j] <= wins[j][i] {
                beats_all = false;
                break;
            }
        }

        if beats_all {
            return Some(i);
        }
    }

    None
}


pub struct SmithIRV;

impl VotingAlgorithm for SmithIRV {

    fn name(&self) -> String {
        "Smith+IRV".to_string()
    }

    fn compute(&self, election: &Election) -> VoteResult {

        let mut smith = smith_set(election);
        let wins = pairwise_matrix(election);

        loop {
            if let Some(w) = condorcet_winner(&wins, &smith) {
                return VoteResult { ranking: vec![w] };
            }

            // Comptage des premiers choix restants
            let mut scores = vec![0usize; election.candidates.len()];

            for ballot in &election.ballots {
                for &c in &ballot.ranking {
                    if smith.contains(&c) {
                        scores[c] += 1;
                        break;
                    }
                }
            }

            // Trouver le plus faible dans smith
            let loser = smith.iter()
                .min_by_key(|&&c| (scores[c], &election.candidates[c]))
                .cloned()
                .unwrap();

            smith.retain(|&c| c != loser);
        }
    }
}