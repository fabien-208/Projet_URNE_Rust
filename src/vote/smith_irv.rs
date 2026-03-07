use crate::{types::VoteResult, vote::VotingAlgorithm};use rayon::prelude::*;

pub struct SmithIRV;

impl VotingAlgorithm for SmithIRV {
    fn name(&self) -> String { "Smith+IRV".to_string() }

    fn compute(&self, election: &crate::types::Election) -> VoteResult {
        let n = election.candidates.len();

        //Matrice des duels
        let wins = election.ballots.par_iter()
            .fold(
                || vec![vec![0usize; n]; n],
                |mut local_wins, ballot| {
                    let mut pos = vec![usize::MAX; n];
                    for (i, &c) in ballot.ranking.iter().enumerate() { pos[c as usize] = i; } // 👈 Fix
                    for i in 0..n {
                        for j in (i+1)..n {
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

        //Ensemble de Smith (simplifié)
        let mut adj = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j && wins[i][j] > wins[j][i] { adj[i][j] = true; }
            }
        }
        let mut reach = adj.clone();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n { reach[i][j] = reach[i][j] || (reach[i][k] && reach[k][j]); }
            }
        }
        let mut smith_members = Vec::new();
        for i in 0..n {
            let mut dominates_all = true;
            for j in 0..n {
                if reach[j][i] && !reach[i][j] { dominates_all = false; break; }
            }
            if dominates_all { smith_members.push(i); }
        }

        //IRV sur Smith
        let mut active = vec![false; n];
        for &c in &smith_members { active[c] = true; }
        if smith_members.is_empty() { for i in 0..n { active[i] = true; } }

        let mut elimination_order = Vec::new();
        let mut remaining = n; // Simplification pour éviter boucle infinie

        while remaining > 0 {
            let scores = election.ballots.par_iter()
                .fold(|| vec![0usize; n], |mut l, b| {
                    for &c in &b.ranking {
                        let cu = c as usize;
                        if active[cu] {
                            l[cu] += 1;
                            break;
                        }
                    }
                    l
                })
                .reduce(|| vec![0; n], |mut t, l| {
                    for i in 0..n { t[i] += l[i]; }
                    t
                });

            let mut min_val = usize::MAX;
            let mut loser = None;
            let mut found_active = false;

            for i in 0..n {
                if active[i] {
                    found_active = true;
                    if scores[i] < min_val {
                        min_val = scores[i];
                        loser = Some(i);
                    } else if scores[i] == min_val {
                        loser = Some(i);
                    }
                }
            }

            if let Some(l) = loser {
                active[l] = false;
                elimination_order.push(l as u8);
                remaining -= 1;
            } else {
                if !found_active { break; }
                // Cas rare de blocage, on vide tout
                for i in 0..n { if active[i] { elimination_order.push(i as u8); } }
                break;
            }
        }

        // Compléter avec ceux hors du Smith Set (s'ils n'ont pas été traités)
        for i in 0..n {
            let iu = i as u8;
            if !elimination_order.contains(&iu) {
                elimination_order.insert(0, iu);
            }
        }

        elimination_order.reverse();
        VoteResult { ranking: elimination_order }
    }
}