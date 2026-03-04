use crate::types::{Election, Ballot};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use rayon::prelude::*;

pub fn parse(filepath: &str) -> io::Result<Election> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    // 1. Header (toujours séquentiel)
    let header = lines_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Fichier vide"))??;

    let candidates: Vec<String> = header
        .split(|c| matches!(c, ';' | ','))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Utilisation d'une Map partagée (lecture seule donc Arc n'est pas nécessaire ici)
    let candidate_map: HashMap<&str, usize> = candidates
        .iter()
        .enumerate()
        .map(|(i, name)| (name.as_str(), i))
        .collect();

    // 2. Chargement des lignes en mémoire
    // On collecte tout pour que Rayon puisse diviser le travail
    let raw_lines: Vec<String> = lines_iter.collect::<Result<Vec<_>, _>>()?;

    // 3. Parsing parallèle avec Rayon
    let ballots: Vec<Ballot> = raw_lines
        .par_iter() // C'est ici que la magie opère
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() { return None; }

            let mut ranking = Vec::with_capacity(candidates.len());
            let mut seen = vec![false; candidates.len()]; // Un par thread, alloué automatiquement

            for part in trimmed.split(|c| matches!(c, '>' | ';' | ',')) {
                let name = part.trim();
                if name.is_empty() { return None; }

                match candidate_map.get(name) {
                    Some(&index) if !seen[index] => {
                        seen[index] = true;
                        ranking.push(index);
                    }
                    _ => return None, // Invalide ou doublon
                }
            }

            if ranking.is_empty() { None } else { Some(Ballot { ranking }) }
        })
        .collect();

    Ok(Election { candidates, ballots })
}