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


pub struct Schulze;

impl VotingAlgorithm for Schulze {

    fn name(&self) -> String {
        "Schulze".to_string()
    }

    fn compute(&self, election: &Election) -> VoteResult {

        let smith = smith_set(election);
        let wins = pairwise_matrix(election);

        let mut edges = Vec::new();

        for &i in &smith {
            for &j in &smith {
                if i >= j { continue; }

                let margin = wins[i][j] as isize - wins[j][i] as isize;

                if margin > 0 {
                    edges.push((i, j, margin));
                } else if margin < 0 {
                    edges.push((j, i, -margin));
                }
            }
        }

        edges.sort_by_key(|e| e.2);

        while !edges.is_empty() {

            let min_margin = edges[0].2;
            edges.retain(|e| e.2 != min_margin);

            for &candidate in &smith {
                let defeated = edges.iter().any(|e| e.1 == candidate);
                if !defeated {
                    return VoteResult { ranking: vec![candidate] };
                }
            }
        }

        panic!("No winner found");
    }
}